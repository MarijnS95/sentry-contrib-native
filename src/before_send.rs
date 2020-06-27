//! Implementation details for [`Options::set_before_send`].

use crate::{ffi, Value};
#[cfg(doc)]
use crate::{Event, Options};
use once_cell::sync::OnceCell;
#[cfg(doc)]
use std::process::abort;
use std::{mem::ManuallyDrop, os::raw::c_void, sync::Mutex};

/// How global [`BeforeSend`] data is stored.
pub type Data = Box<Box<dyn BeforeSend>>;

/// Store [`Options::set_before_send`] data to properly deallocate later.
pub static BEFORE_SEND: OnceCell<Mutex<Option<Data>>> = OnceCell::new();

/// Trait to help pass data to [`Options::set_before_send`].
///
/// # Examples
/// ```
/// # use sentry_contrib_native::{BeforeSend, Options, Value};
/// # use std::sync::atomic::{AtomicUsize, Ordering};
/// # fn main() -> anyhow::Result<()> {
/// struct Filter {
///     filtered: AtomicUsize,
/// };
///
/// impl BeforeSend for Filter {
///     fn before_send(&self, value: Value) -> Value {
///         self.filtered.fetch_add(1, Ordering::SeqCst);
///         // do something with the value and then return it
///         value
///     }
/// }
///
/// let mut options = Options::new();
/// options.set_before_send(Filter {
///     filtered: AtomicUsize::new(0),
/// });
/// let _shutdown = options.init()?;
/// # Ok(()) }
/// ```
pub trait BeforeSend: 'static + Send + Sync {
    /// Before send callback.
    ///
    /// # Notes
    /// The caller of this function will catch any unwinding panics and
    /// [`abort`] if any occured.
    ///
    /// # Panics
    /// Panics if any [`String`] in the returning [`Value`] contains a null
    /// byte.
    ///
    /// # Examples
    /// ```
    /// # use sentry_contrib_native::{BeforeSend, Value};
    /// # use std::sync::atomic::{AtomicUsize, Ordering};
    /// struct Filter {
    ///     filtered: AtomicUsize,
    /// };
    ///
    /// impl BeforeSend for Filter {
    ///     fn before_send(&self, value: Value) -> Value {
    ///         self.filtered.fetch_add(1, Ordering::SeqCst);
    ///         // do something with the value and then return it
    ///         value
    ///     }
    /// }
    /// ```
    fn before_send(&self, value: Value) -> Value;
}

impl<T: Fn(Value) -> Value + 'static + Send + Sync> BeforeSend for T {
    fn before_send(&self, value: Value) -> Value {
        self(value)
    }
}

/// Function to pass to [`sys::options_set_before_send`], which in turn calls
/// the user defined one.
///
/// This function will catch any unwinding panics and [`abort`] if any occured.
#[allow(clippy::module_name_repetitions)]
pub extern "C" fn sentry_contrib_native_before_send(
    event: sys::Value,
    _hint: *mut c_void,
    closure: *mut c_void,
) -> sys::Value {
    let before_send =
        ManuallyDrop::new(unsafe { Box::<Box<dyn BeforeSend>>::from_raw(closure as _) });

    ffi::catch(|| {
        before_send
            .before_send(unsafe { Value::from_raw(event) })
            .into_raw()
    })
}

#[cfg(test)]
#[rusty_fork::test_fork(timeout_ms = 10000)]
fn before_send() -> anyhow::Result<()> {
    use crate::{Event, Options, Value};
    use std::{
        cell::RefCell,
        sync::atomic::{AtomicUsize, Ordering},
    };

    thread_local! {
        static COUNTER: RefCell<usize> = RefCell::new(0);
    }

    struct Filter {
        counter: AtomicUsize,
    }

    impl BeforeSend for Filter {
        fn before_send(&self, value: Value) -> Value {
            self.counter.fetch_add(1, Ordering::SeqCst);
            value
        }
    }

    impl Drop for Filter {
        fn drop(&mut self) {
            COUNTER.with(|counter| *counter.borrow_mut() = *self.counter.get_mut())
        }
    }

    let mut options = Options::new();
    options.set_before_send(Filter {
        counter: AtomicUsize::new(0),
    });
    let shutdown = options.init()?;

    Event::new().capture();
    Event::new().capture();
    Event::new().capture();

    shutdown.shutdown();

    COUNTER.with(|counter| assert_eq!(3, *counter.borrow()));

    Ok(())
}

#[cfg(test)]
#[rusty_fork::test_fork(timeout_ms = 5000)]
#[should_panic]
fn catch_panic() -> anyhow::Result<()> {
    use crate::{Event, Options};

    let mut options = Options::new();
    options.set_before_send(|_| panic!("this is a test"));
    let _shutdown = options.init()?;

    Event::new().capture();

    Ok(())
}
