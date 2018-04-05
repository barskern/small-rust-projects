use std::error::Error;
use std::fmt;

pub mod content;
pub mod request;
pub mod response;

pub use self::request::Request;
// pub use self::response::Response;

/// Error from not being able to parse a string into a Request
#[derive(Debug)]
pub enum ParseHttpError {
  Request,
  Response
}

impl fmt::Display for ParseHttpError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.description())
  }
}

impl Error for ParseHttpError {
  fn description(&self) -> &str {
    match self {
      Request => "Unable to parse String into Request",
      Response => "Unable to parse String into Response"
    }
  }
}