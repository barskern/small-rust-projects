use super::content::Content;
use std::convert::TryFrom;
use std::str::FromStr;
use std;

#[derive(Debug, PartialEq)]
pub struct Request {
  method: RequestMethod,
  uri: String,
  version: String,
  content: Content,
}

impl TryFrom<String> for Request {
  type Error = ParseRequestError;

  fn try_from(s: String) -> Result<Self, Self::Error> {
    Err(ParseRequestError::invalid())
  }
}

parse_from_string_error!(ParseRequestError, Request);

#[derive(Debug, PartialEq)]
pub enum RequestMethod {
  GET,
  HEAD,
  PUT,
  POST,
}

impl FromStr for RequestMethod {
  type Err = ParseRequestMethodError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "GET" => Ok(RequestMethod::GET),
      "HEAD" => Ok(RequestMethod::HEAD),
      "PUT" => Ok(RequestMethod::PUT),
      "POST" => Ok(RequestMethod::POST),
      _ => Err(ParseRequestMethodError::invalid()),
    }
  }
}

parse_from_string_error!(ParseRequestMethodError, RequestMethod);

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn request_from_string_simple() {
    let request_str = "GET / HTTP/1.1\r\n\r\n\r\n".to_string();
    let request = match Request::try_from(request_str) {
      Ok(request) => request,
      Err(e) => panic!("Error: {}", e),
    };

    let expected_request = Request {
      method: RequestMethod::GET,
      uri: "/".to_string(),
      version: "HTTP/1.1".to_string(),
      content: Content::new(String::new()),
    };

    assert_eq!(expected_request, request);
  }

  #[test]
  fn method_from_string_good() {
    let possible_methods = vec![
      ("GET", RequestMethod::GET),
      ("HEAD", RequestMethod::HEAD),
      ("PUT", RequestMethod::PUT),
      ("POST", RequestMethod::POST),
    ];

    for (method_str, expected_method) in possible_methods {
      let method = match RequestMethod::from_str(method_str) {
        Ok(method) => method,
        Err(e) => panic!("Error: {}", e),
      };
      assert_eq!(expected_method, method);
    }
  }

  #[test]
  #[should_panic]
  fn method_from_string_bad() {
    let method_str = "WRONG";
    RequestMethod::from_str(method_str).unwrap();
  }
}
