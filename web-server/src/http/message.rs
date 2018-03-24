use std;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Message<'a> {
  headers: HashMap<&'a str, &'a str>,
  body: Vec<&'a str>,
}

impl<'a> Message<'a> {
  pub fn from_str(message_str: &'a str) -> Self {
    let lines = message_str.lines();
    Message::from_lines(lines)
  }

  pub fn from_lines(mut message_lines: std::str::Lines<'a>) -> Self {
    let mut headers: HashMap<&'a str, &'a str> = HashMap::new();
    while let Some(header_line) = message_lines.next() {
      if header_line.is_empty() {
        // We found the start of the body!
        break;
      }
      let mut header_line_iter = header_line.splitn(2, ':').map(|s| s.trim());
      let name = header_line_iter.next().unwrap();
      let value = header_line_iter.next().unwrap();

      headers.insert(name, value);
    }

    Message {
      headers,
      body: message_lines.collect(), // Collects remaining lines
    }
  }
}

pub struct MessageBuilder<'a> {
  __headers: HashMap<&'a str, &'a str>,
  __body: Vec<&'a str>,
}

impl<'a> MessageBuilder<'a> {
  pub fn new() -> Self {
    MessageBuilder {
      __headers: HashMap::new(),
      __body: Vec::new(),
    }
  }

  pub fn body(mut self, body: &'a str) -> Self {
    self.__body = body.lines().collect();
    self
  }

  pub fn add_lines_to_body(mut self, new_lines: Vec<&'a str>) -> Self {
    self.__body.extend(new_lines);
    self
  }

  pub fn add_header(mut self, name: &'a str, value: &'a str) -> Self {
    self.__headers.insert(name, value);
    self
  }

  pub fn build(self) -> Message<'a> {
    Message {
      headers: self.__headers,
      body: self.__body,
    }
  }
}
