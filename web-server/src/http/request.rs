use std::convert::TryFrom;
use std::str::FromStr;
pub use super::errors::{
  ParseRequestError, 
  ParseRequestMethodError
};
use super::content::Content;

#[derive(Debug, PartialEq)]
pub struct Request {
  method: RequestMethod,
  uri: String,
  version: String,
  content: Content,
}

impl Request {
  pub fn method(&self) -> &RequestMethod {
    &self.method
  }
  pub fn uri(&self) -> &str {
    &self.uri
  }
  pub fn version(&self) -> &str {
    &self.version
  }
  pub fn body(&self) -> &str {
    self.content.body()
  }
  pub fn header(&self, header: &str) -> Option<&str> {
    self.content.header(header)
  }
}

impl TryFrom<String> for Request {
  type Error = ParseRequestError;

  fn try_from(mut s: String) -> Result<Self, Self::Error> {
    if s.len() == 0 {
      return Err(ParseRequestError::empty());
    }

    let content_str = {    
      let newline_pos = s
        .find("\r\n")
        .map(|pos| pos + 2)
        .or(s
          .find('\n')
          .map(|pos| pos + 1)
        )?;
      s.split_off(newline_pos)
    };

    let request_line: Vec<&str> = s
      .split_whitespace()
      .collect();

    if request_line.len() < 3 {
      return Err(ParseRequestError::invalid());
    }

    let method = RequestMethod::from_str(request_line[0])?;
    let uri = request_line[1].to_string();
    let version = request_line[2].to_string();
    let content = Content::try_from(content_str)?;

    Ok(Request {
      method,
      uri,
      version,
      content,
    })
  }
}

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
      content: Content::new("".to_string()),
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
}
