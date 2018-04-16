use std::{convert::TryFrom, fmt::{self, Display}, str::FromStr};

use super::content::{Content, Contentable};
use super::HTTP_VERSION;

pub use super::errors::{ParseRequestError, ParseRequestMethodError};
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
  pub fn new<S: Into<String>>(method: RequestMethod, uri: S) -> Request {
    Request {
      method,
      uri: uri.into(),
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
  fn set_body<S: Into<String>>(&mut self, new_body: S) -> String {
    self.content.set_body(new_body)
  }
  fn has_header(&self, name: &str) -> Option<&str> {
    self.content.has_header(name)
  }
  fn add_header<S: Into<String>>(&mut self, name: S, value: S) -> Option<String> {
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

/// A small enum which encodes the type of http-request.
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
    use self::RequestMethod::*;
    match s {
      "GET" => Ok(GET),
      "HEAD" => Ok(HEAD),
      "PUT" => Ok(PUT),
      "POST" => Ok(POST),
      _ => Err(ParseRequestMethodError::invalid()),
    }
  }
}

impl Display for RequestMethod {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    use self::RequestMethod::*;
    let method_str = match *self {
      GET => "GET",
      HEAD => "HEAD",
      PUT => "PUT",
      POST => "POST",
    };
    write!(fmt, "{}", method_str)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn request_from_string_simple() {
    let req_str = "GET / HTTP/1.1\r\n\r\n\r\n".to_string();
    let req = match Request::try_from(req_str) {
      Ok(req) => req,
      Err(e) => panic!("Should not get error on valid http: {}", e),
    };

    let expected_req = Request {
      method: RequestMethod::GET,
      uri: "/".to_string(),
      content: Content::default(),
    };

    assert_eq!(expected_req, req, "Creating Request from String failed");
  }

  #[test]
  fn request_from_string() {
    let req_str = "GET /about/us HTTP/1.1\r\nHost: Localhost\r\nCache: 3000\r\n\r\n".to_string();
    let req = match Request::try_from(req_str) {
      Ok(req) => req,
      Err(e) => panic!("Should not get error on valid http: {}", e),
    };

    let mut expected_cont = Content::default();
    expected_cont.add_header("Host", "Localhost");
    expected_cont.add_header("Cache", "3000");
    let expected_req = Request {
      method: RequestMethod::GET,
      uri: "/about/us".to_string(),
      content: expected_cont,
    };

    assert_eq!(expected_req, req, "Creating Request from String failed");
  }

  #[test]
  fn request_from_string_with_body() {
    let req_str = "PUT /new HTTP/1.1\r\nHost: Localhost\r\n\r\n{\"name\": \"John\"}".to_string();
    let req = match Request::try_from(req_str) {
      Ok(req) => req,
      Err(e) => panic!("Should not get error on valid http: {}", e),
    };

    let mut expected_cont = Content::new("{\"name\": \"John\"}");
    expected_cont.add_header("Host", "Localhost");
    let expected_req = Request {
      method: RequestMethod::PUT,
      uri: "/new".to_string(),
      content: expected_cont,
    };

    assert_eq!(expected_req, req, "Creating Request from String failed");
  }

  #[test]
  fn request_to_string() {
    let mut req = Request::new(RequestMethod::GET, "/new_page");
    req.add_header("Host", "Remotehost");

    let expected_str = "GET /new_page HTTP/1.1\r\nHost: Remotehost\r\n\r\n";
    assert_eq!(
      expected_str,
      req.to_string(),
      "Didn't convert to valid http"
    )
  }

  #[test]
  fn construct_request() {
    let mut req = Request::new(RequestMethod::GET, "/about/");
    req.add_header("Host", "Localhost");

    let mut expected_cont = Content::default();
    expected_cont.add_header("Host", "Localhost");
    let expected_req = Request {
      method: RequestMethod::GET,
      uri: "/about/".to_string(),
      content: expected_cont,
    };

    assert_eq!(expected_req, req, "Creation not matching expectation");
  }

  #[test]
  fn method_from_string_good() {
    use self::RequestMethod::*;
    let possible_methods = vec![("GET", GET), ("HEAD", HEAD), ("PUT", PUT), ("POST", POST)];

    for (method_str, expected_method) in possible_methods {
      let method = match RequestMethod::from_str(method_str) {
        Ok(method) => method,
        Err(e) => panic!("Should not get error on valid http: {}", e),
      };
      assert_eq!(
        expected_method, method,
        "Didn't get correct result when converting String to RequestMethod"
      );
    }
  }

}
