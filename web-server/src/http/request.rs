use super::content::Content;
use std::convert::TryFrom;
use std::str::FromStr;
use std::fmt;
use std::error;

#[derive(Debug, PartialEq)]
pub struct Request {
  method: Method,
  uri: String,
  version: String,
  content: Content,
}

impl TryFrom<String> for Request {
  type Error = ParseRequestError;

  fn try_from(s: String) -> Result<Self, Self::Error> {
    Err(ParseRequestError {})
  }
}

/// Error from not being able to parse a string into a Request
#[derive(Debug)]
pub struct ParseRequestError {}

impl fmt::Display for ParseRequestError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self)
  }
}

impl error::Error for ParseRequestError {
  fn description(&self) -> &str {
    "Unable to parse String into Request"
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

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn request_from_string_simple() {
    let request_str = "GET / HTTP/1.1\r\n\r\n\r\n".to_string();
    let request = Request::try_from(request_str).unwrap();

    let expected_request = Request {
      method: Method::GET,
      uri: "/".to_string(),
      version: "HTTP/1.1".to_string(),
      content: Content::new(String::new()),
    };

    assert_eq!(expected_request, request);
  }
  #[test]
  fn method_from_string_good() {
    let possible_methods = vec![
      ("GET", Method::GET),
      ("HEAD", Method::HEAD),
      ("PUT", Method::PUT),
      ("POST", Method::POST),
    ];

    for (method_str, expected_method) in possible_methods {
      assert_eq!(expected_method, Method::from_str(method_str).unwrap());
    }
  }
  #[test]
  #[should_panic]
  fn method_from_string_bad() {
    let method_str = "WRONG";
    Method::from_str(method_str).unwrap();
  }
}
