use std::collections::HashMap;

#[derive(Debug)]
pub struct Message<'a> {
  headers: HashMap<&'a str, &'a str>,
  body: &'a str,
}

impl<'a> Message<'a> {
  pub fn from_str(message_str: &'a str) -> Option<Self> {
    let mut lines = message_str.lines();
    let mut headers: HashMap<&'a str, &'a str> = HashMap::new();
    while let Some(header_line) = lines.next() {
      if header_line.is_empty() {
        // We found the start of the body!
        break;
      }
      let mut header_line_iter = header_line.splitn(2, ':').map(|s| s.trim());
      let name = header_line_iter.next()?;
      let value = header_line_iter.next()?;

      headers.insert(name, value);
    }

    let body = {
      let newline_char_len = 2;
      let remaining_bytes = lines.fold(0, |acc, line| acc + line.len() + newline_char_len);

      if remaining_bytes < message_str.len() {
        &message_str[message_str.len() - remaining_bytes..]
      } else {
        ""
      }
    };

    Some(Message { headers, body })
  }
}

pub struct MessageBuilder<'a> {
  __headers: HashMap<&'a str, &'a str>,
  __body: &'a str,
}

impl<'a> MessageBuilder<'a> {
  pub fn new() -> Self {
    MessageBuilder {
      __headers: HashMap::new(),
      __body: "",
    }
  }

  pub fn body(mut self, body: &'a str) -> Self {
    self.__body = body;
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
