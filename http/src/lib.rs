#![feature(try_from, try_trait, iterator_try_fold)]

//! A crate to simplify using the http protocol
//! across a Tcp-connection. The crate was mainly
//! created to be a learning experience.

pub mod errors;
pub mod content;
pub mod request;
pub mod response;

pub const HTTP_VERSION: &str = "HTTP/1.1";

pub use self::errors::ParseHttpError;
pub use self::request::Request;
pub use self::response::Response;
