use anyhow::Result;
use sentry::{Options, User};
use sentry_contrib_native as sentry;
use std::io::{self, Read};

#[cfg(feature = "custom-transport")]
#[path = "../util/custom_transport.rs"]
mod custom_transport;

#[cfg(feature = "custom-transport")]
use custom_transport::Transport;

fn main() -> Result<()> {
    sentry::set_hook(None, None);

    let mut options = Options::new();
    options.set_debug(true);
    #[cfg(feature = "custom-transport")]
    options.set_transport(Transport::new);
    let _shutdown = options.init()?;

    let mut buffer = [0; 16];
    io::stdin().read_exact(&mut buffer)?;
    let id = hex::encode(buffer);

    let mut user = User::new();
    user.insert("id", id);
    user.set();

    panic!("test panic")
}
