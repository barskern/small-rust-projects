use super::content::Content;
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Request {
  method: Method,
  uri: String,
  version: String,
  content: Content,
}

impl TryFrom<String> for Request {
  type Error = String;

  fn try_from(s: String) -> Result<Self, Self::Error> {
    Err(format!("Failed to create request from: {}", s).to_string())
  }
}

#[derive(Debug, PartialEq)]
pub enum Method {
  GET,
  HEAD,
  PUT,
  POST,
}

impl FromStr for Method {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "GET" => Ok(Method::GET),
      "HEAD" => Ok(Method::HEAD),
      "PUT" => Ok(Method::PUT),
      "POST" => Ok(Method::POST),
      _ => Err(format!("Failed to create request method from: {}", s).to_string()),
    }
  }
}
