initSidebarItems({"constant":[["API_VERSION","Version of the Sentry API we can communicate with, AFAICT this is just hardcoded into sentry-native, so...two can play at that game!"],["ENVELOPE_MIME","The MIME type for Sentry envelopes."],["SDK_USER_AGENT","SDK Version"]],"enum":[["Consent","The state of user consent."],["Error","[`sentry-contrib-native`] errors."],["Interface","Sentry event interface."],["Level","Sentry event level."],["Message","Message received for custom logger."],["TransportError","Sentry errors."],["TransportShutdown","The return from [`Transport::shutdown`], which determines if we tell the Sentry SDK if we were able to send all requests to the remote service or not in the allotted time."],["Value","Represents a Sentry protocol value."]],"fn":[["clear_modulecache","Clears the internal module cache."],["end_session","Ends a session."],["remove_context","Removes the context object with the specified key."],["remove_extra","Removes the extra with the specified `key`."],["remove_fingerprint","Removes the fingerprint."],["remove_tag","Removes the tag with the specified `key`."],["remove_transaction","Removes the transaction."],["remove_user","Removes a user."],["set_context","Sets a context object."],["set_extra","Sets extra information."],["set_fingerprint","Sets the event fingerprint."],["set_hook","Panic handler to send an [`Event`] with the current stacktrace to Sentry."],["set_level","Sets the event level."],["set_tag","Sets a tag."],["set_transaction","Sets the transaction."],["shutdown","Shuts down the Sentry client and forces transports to flush out."],["start_session","Starts a new session."],["user_consent_get","Checks the current state of user consent."],["user_consent_give","Gives user consent."],["user_consent_reset","Resets the user consent (back to unknown)."],["user_consent_revoke","Revokes user consent."]],"mod":[["before_send","Implementation details for [`Options::set_before_send`]."],["breadcrumb","Sentry breadcrumb implementation."],["event","Sentry event implementation."],["ffi","FFI helper functions, traits and types to communicate with `sentry-native`."],["logger","Implementation details for [`Options::set_logger`]."],["object","Sentry object implementation, represents common functionality between [`Map`], [`Breadcrumb`], [`Event`], and [`User`]."],["options","Sentry options implementation."],["panic","Sentry supported panic handler."],["transport","Sentry custom transport implementation."],["user","Sentry user implementation."],["value","Sentry value implementation."]],"struct":[["Breadcrumb","A Sentry breadcrumb."],["Dsn","Contains the pieces we need to send requests based on the DSN the user set on [`Options`]"],["Envelope","The actual body which transports send to Sentry."],["Event","A Sentry event."],["Options","The Sentry client options."],["Parts","[`Parts`] aquired from [`Dsn::into_parts`]."],["RawEnvelope","Wrapper for the raw Envelope that we should send to Sentry."],["Shutdown","Automatically shuts down the Sentry client on drop."],["User","A Sentry user."],["Uuid","A Sentry UUID."]],"trait":[["BeforeSend","Trait to help pass data to [`Options::set_before_send`]."],["Transport","Trait used to define a custom transport that Sentry can use to send events to a Sentry service."]],"type":[["Request","The [`http::Request`] request your [`Transport`] is expected to send."]]});