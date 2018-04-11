#![feature(try_from, try_trait, iterator_try_fold)]

pub mod errors;
pub mod content;
pub mod request;
pub mod response;

pub const HTTP_VERSION: &str = "HTTP/1.1";

pub use self::errors::ParseHttpError;
pub use self::request::Request;
pub use self::response::Response;
