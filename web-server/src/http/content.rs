use std::collections::HashMap;
use std::convert::TryFrom;
use std::default::Default;
use std::fmt::{self, Display};
use super::errors::ParseContentError;


#[derive(Debug, PartialEq)]
pub struct Content {
  headers: HashMap<String, String>,
  body: String,
}

impl Content {
  pub fn new(body: String) -> Self {
    Content {
      headers: HashMap::new(),
      body,
    }
  }
}

impl Content {
  pub fn body(&self) -> &str {
    &self.body
  }
  pub fn has_header(&self, header: &str) -> Option<&str> {
    self.headers.get(header).map(|s| s.as_str())
  }
}

impl Default for Content {
  fn default() -> Self {
    Self::new("".to_string())
  }
}

impl Display for Content {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    let header_str: String = self.headers.iter()
      .map(|(k,v)| format!("{}: {}", k.to_string(), v.to_string()))
      .fold(String::new(), |acc, l| format!("{}\r\n{}", l, acc));

    write!(fmt, "{}\r\n{}", header_str, self.body)
  }
}

impl TryFrom<String> for Content {
  type Error = ParseContentError;

  fn try_from(mut s: String) -> Result<Self, Self::Error> {
    if s.is_empty() {
      return Err(ParseContentError::empty());
    }

    let body = {
      let body_start_pos = s 
        .find("\r\n\r\n").map(|pos| pos + 4)
        .or(
          s.find("\n\n").map(|pos| pos + 2)
        )?;
      s.split_off(body_start_pos)
    };

    let headers: HashMap<String, String> = s.lines()
        .filter(|line| !line.is_empty())
        .map(|line|
          line.splitn(2, ':')
            .map(|s| s.trim())
            .collect::<Vec<_>>()
        )
        .try_fold(HashMap::new(), 
          |mut headers, vec|
          if vec.len() == 2 {
            headers.insert(vec[0].to_string(), vec[1].to_string());
            Ok(headers)
          } else {
            Err(ParseContentError::invalid())
          }
        )?;

    Ok(Content {
      headers,
      body
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn content_from_string_empty_valid() {
    let content_str = "\r\n\r\n".to_string();
    let content = match Content::try_from(content_str.clone()) {
      Ok(content) => content,
      Err(e) => panic!("Error: {}: {}", e, content_str),      
    };

    let expected_content = Content {
      headers: HashMap::new(),
      body: "".to_string(),
    };

    assert_eq!(expected_content, content);
  }

  #[test]
  fn content_from_string_empty_unvalid() {
    let content_str = "\r\n".to_string();
    match Content::try_from(content_str.clone()) {
      Ok(_) => panic!("Should not get content when not following format rules."),
      Err(_) => {},      
    };
  }

  #[test]
  fn content_from_string_valid() {
    let content_str = "Host: Localhost\r\nCache: 3000\r\n\r\nHello world in the body".to_string();
    let content = match Content::try_from(content_str.clone()) {
      Ok(content) => content,
      Err(e) => panic!("Error: {}: {}", e, content_str),
    };

    let mut headers = HashMap::new();
    headers.insert("Host".to_string(), "Localhost".to_string());
    headers.insert("Cache".to_string(), "3000".to_string());

    let expected_content = Content {
      headers,
      body: "Hello world in the body".to_string(),
    };

    assert_eq!(expected_content, content);
  }

  #[test]
  fn content_from_string_unvalid_header() {
    let content_str = "Host: Localhost\r\nCache 3000\r\n\r\nHello world in the body".to_string();
    match Content::try_from(content_str.clone()) {
      Ok(_) => panic!("Should get error when not correct HTTP-format."),
      Err(_) => {},      
    };
  }
}
