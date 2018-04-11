use std::{convert::TryFrom, fmt::{self, Display}, str::FromStr};

pub use super::{content::{Content, Contentable}, errors::{ParseRequestError, ParseRequestMethodError},
                HTTP_VERSION};

/// A struct which contains information for an http request. 
/// When written to string, the struct is valid http, which 
/// can be directly sent across a TCP-connection.
#[derive(Debug, PartialEq)]
pub struct Request {
  method: RequestMethod,
  uri: String,
  content: Content,
}

impl Request {
  pub fn new(uri: String) -> Request {
    Request {
      method: RequestMethod::GET,
      uri,
      content: Content::default(),
    }
  }

  pub fn method(&self) -> RequestMethod {
    self.method
  }
  pub fn uri(&self) -> &str {
    &self.uri
  }
}

impl Contentable for Request {
  fn get_body(&self) -> &str {
    self.content.get_body()
  }
  fn set_body(&mut self, new_body: String) -> String {
    self.content.set_body(new_body)    
  }
  fn has_header(&self, name: &str) -> Option<&str> {
    self.content.has_header(name)
  }
  fn add_header(&mut self, name: String, value: String) -> Option<String> {
    self.content.add_header(name, value)
  }
}

impl TryFrom<String> for Request {
  type Error = ParseRequestError;

  fn try_from(mut s: String) -> Result<Self, Self::Error> {
    if s.len() == 0 {
      return Err(ParseRequestError::empty());
    }

    let content_str = {
      let newline_pos = s.find("\r\n")
        .map(|pos| pos + 2)
        .or(s.find('\n').map(|pos| pos + 1))?;
      s.split_off(newline_pos)
    };

    let request_line: Vec<&str> = s.split_whitespace().collect();

    if request_line.len() < 3 {
      return Err(ParseRequestError::invalid());
    }
    let version = request_line[2];
    if version != HTTP_VERSION {
      return Err(ParseRequestError::invalid());
    }

    let method = RequestMethod::from_str(request_line[0])?;
    let uri = request_line[1].to_string();
    let content = Content::try_from(content_str)?;

    Ok(Request {
      method,
      uri,
      content,
    })
  }
}

impl Display for Request {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    write!(
      fmt,
      "{} {} {}\r\n{}",
      self.method, self.uri, HTTP_VERSION, self.content
    )
  }
}

/// A small enum which encodes the type of
/// http-request.
#[derive(Copy, Clone, Debug, PartialEq)]
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

impl Display for RequestMethod {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    let method_str = match *self {
      RequestMethod::GET => "GET",
      RequestMethod::HEAD => "HEAD",
      RequestMethod::PUT => "PUT",
      RequestMethod::POST => "POST",
    };
    write!(fmt, "{}", method_str)
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
      content: Content::default(),
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
