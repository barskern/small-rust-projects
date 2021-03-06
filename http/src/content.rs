use std::{mem, collections::HashMap, convert::TryFrom, default::Default, fmt::{self, Display}};
use super::errors::ParseContentError;

/// A struct which has contains the content of a
/// http message. This includes headers and body.
///
/// This structure is only used inside Request
/// or Response. This generalization has been
/// made because the "content" of an http message
/// is the same in both the request and response.
/// A single definition will limit code duplication.
#[derive(Debug, PartialEq)]
pub struct Content {
  headers: HashMap<String, String>,
  body: String,
}

impl Content {
  pub fn new<S: Into<String>>(body: S) -> Self {
    Content {
      headers: HashMap::new(),
      body: body.into(),
    }
  }
}

impl Contentable for Content {
  fn get_body(&self) -> &str {
    &self.body
  }
  fn set_body<S: Into<String>>(&mut self, new_body: S) -> String {
    mem::replace(&mut self.body, new_body.into())
  }
  fn has_header(&self, name: &str) -> Option<&str> {
    self.headers.get(name).map(|s| s.as_str())
  }
  fn add_header<S: Into<String>>(&mut self, name: S, value: S) -> Option<String> {
    self.headers.insert(name.into(), value.into())
  }
}

impl Default for Content {
  fn default() -> Self {
    Self::new("")
  }
}

impl Display for Content {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    let header_str: String = self
      .headers
      .iter()
      .map(|(k, v)| format!("{}: {}", k.to_string(), v.to_string()))
      .fold(String::new(), |acc, l| format!("{}\r\n{}", l, acc));

    write!(fmt, "{}\r\n{}", header_str, self.body)
  }
}

/// Try to get http-content from string. Should
/// give error on wrong format.
impl TryFrom<String> for Content {
  type Error = ParseContentError;

  fn try_from(mut s: String) -> Result<Self, Self::Error> {
    if s.is_empty() {
      return Err(ParseContentError::empty());
    }

    let body = {
      let body_start_pos = s.find("\r\n\r\n")
        .map(|pos| pos + 4)
        .or(s.find("\n\n").map(|pos| pos + 2))?;

      if s.len() <= body_start_pos {
        "".to_string()
      } else {
        s.split_off(body_start_pos)
      }
    };

    let headers: HashMap<String, String> = s.lines()
      .filter(|line| !line.is_empty())
      .map(|line| line.splitn(2, ':').map(|s| s.trim()).collect::<Vec<_>>())
      .try_fold(HashMap::new(), |mut headers, vec| {
        if vec.len() == 2 {
          headers.insert(vec[0].to_string(), vec[1].to_string());
          Ok(headers)
        } else {
          Err(ParseContentError::invalid())
        }
      })?;

    Ok(Content { headers, body })
  }
}

/// Trait given to types that has content to provide
/// a seemless transition between the content and the
/// outer parent. This makes it easy to interact with
/// the content within a Request and a Response.
pub trait Contentable {
  /// Gets a immutable borrow of the body of the message
  fn get_body(&self) -> &str;
  /// Sets the body to a new string and returns the old body
  fn set_body<S: Into<String>>(&mut self, new_body: S) -> String;
  /// Checks to see if header exists and returns value of said header
  fn has_header(&self, name: &str) -> Option<&str>;
  /// Adds a header to the message. Will return "Some()" with the value of
  /// the previously defined header if overwriting.
  fn add_header<S: Into<String>>(&mut self, name: S, value: S) -> Option<String>;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn content_from_empty_string() {
    let content_str = "\r\n\r\n".to_string();
    let content = match Content::try_from(content_str.clone()) {
      Ok(content) => content,
      Err(e) => panic!("Error: {}: {}", e, content_str),
    };
    assert_eq!(
      Content::default(),
      content,
      "Default content not equal to content from empty string"
    );
  }

  #[test]
  fn content_from_empty_string_unvalid() {
    let content_str = "\r\n".to_string();
    match Content::try_from(content_str.clone()) {
      Ok(_) => panic!("Should not get content when not following protocol."),
      Err(_) => {}
    };
  }

  #[test]
  fn content_from_string() {
    let content_str = "Host: Localhost\r\nCache: 3000\r\n\r\nHello world in the body".to_string();
    let content = match Content::try_from(content_str.clone()) {
      Ok(content) => content,
      Err(e) => panic!("Should not get error on valid http: {}", e),
    };

    let mut headers = HashMap::new();
    headers.insert("Host".to_string(), "Localhost".to_string());
    headers.insert("Cache".to_string(), "3000".to_string());

    let expected_content = Content {
      headers,
      body: "Hello world in the body".to_string(),
    };

    assert_eq!(
      expected_content, content,
      "Did not parse content into expected structure"
    );
  }

  #[test]
  fn content_from_unvalid_string_header() {
    let content_str = "Host: Localhost\r\nCache 3000\r\n\r\nHello world in the body".to_string();
    match Content::try_from(content_str.clone()) {
      Ok(_) => panic!("Should get error when unvalid http"),
      Err(_) => {}
    };
  }

  #[test]
  fn use_headers() {
    let mut cont = Content::new("hello_world");
    cont.add_header("Host", "Localhost");

    assert_eq!(
      Some("Localhost"),
      cont.has_header("Host"),
      "Didn't return expected value for header"
    );
    assert_eq!(
      Some("Localhost"),
      cont.has_header("Host"),
      "Content gave away ownership when getting header"
    );
  }

  #[test]
  fn replace_body() {
    let mut cont = Content::new("{\"username\": \"johnny\"}");
    cont.add_header("Host", "Localhost");

    assert_eq!(
      "{\"username\": \"johnny\"}",
      cont.get_body(),
      "Didn't give back the correct body"
    );

    let old_body = cont.set_body("{\"username\": \"karl\"}");

    assert_eq!(
      "{\"username\": \"johnny\"}", old_body,
      "Didn't give back the correct \"old\" body after replacement"
    );

    assert_eq!(
      "{\"username\": \"karl\"}",
      cont.get_body(),
      "Didn't give back the correct body after replacement"
    );
  }
}
