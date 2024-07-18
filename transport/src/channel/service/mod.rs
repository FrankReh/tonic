mod add_origin;
use self::add_origin::AddOrigin;

mod user_agent;
use self::user_agent::UserAgent;

mod reconnect;
use self::reconnect::Reconnect;

mod connection;
pub(super) use self::connection::Connection;

mod discover;
pub(super) use self::discover::DynamicServiceStream;

mod io;
use self::io::BoxedIo;

mod connector;
pub(crate) use self::connector::Connector;
// TODO This next import is/was used in tonic to allow the tonic crate
// to access this error type when the "channel" feature is enabled.
// Question: what does this mean to an external transport that would want
// the same treatment? Even if tonic is compiled with "channel", the error
// type won't match the one created by this external transport.
#[allow(unused_imports)]
pub(crate) use self::connector::ConnectError;

mod executor;
pub(super) use self::executor::{Executor, SharedExec};

#[cfg(feature = "tls")]
mod tls;
#[cfg(feature = "tls")]
pub(super) use self::tls::TlsConnector;
