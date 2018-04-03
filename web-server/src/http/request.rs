use super::content::Content;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct RequestContent<'a> {
  uri: &'a str,
  version: &'a str,
  content: Content<'a>,
}

impl<'a> FromStr for RequestContent<'a> {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Err(String::from(format!(
      "Failed to create http-request-content from: {}",
      s
    )))
  }
}

#[derive(Debug, PartialEq)]
pub enum Request<'a> {
  GET(RequestContent<'a>),
  HEAD(RequestContent<'a>),
  PUT(RequestContent<'a>),
  POST(RequestContent<'a>),
}

impl<'a> FromStr for Request<'a> {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut lines = s.lines();

    if let Some(request_line) = lines.next() {
      println!("{}", request_line);
      let mut request_line_iter = request_line.split_whitespace();

      if let (Some(method), Some(uri), Some(version)) = (
        request_line_iter.next(),
        request_line_iter.next(),
        request_line_iter.next()
      ) {
        return Ok(Request::GET(RequestContent {
          uri: uri,
          version: version,
          content: Content::new(""),
        }));
      }
    }
    Err(String::from(format!(
      "Failed to create http-request from: {}",
      s
    )))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn request_from_str_request_line() {
    let request = "GET / HTTP/1.1\r\n\r\n".parse::<Request>().unwrap();

    let expected_request = Request::GET(RequestContent {
      uri: "/",
      version: "HTTP/1.1",
      content: Content::new(""),
    });

    assert_eq!(request, expected_request);
  }

  #[test]
  fn request_from_str_possible_uris() {
    let different_uris = vec![
      "*",
      "http://barskern.github.io/index.html",
      "/about/me/index.html ",
    ];

    different_uris.into_iter().for_each(|uri| {
      let request = format!("GET {} HTTP/1.1\r\n\r\n", uri)
        .parse::<Request>()
        .unwrap();

      let expected_request = Request::GET(RequestContent {
        uri,
        version: "HTTP/1.1",
        content: Content::new(""),
      });

      assert_eq!(request, expected_request);
    })
  }
}
