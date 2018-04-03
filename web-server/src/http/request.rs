use super::message::{Message, MessageBuilder};

#[derive(Debug)]
pub struct Request<'a> {
  method: Methods,
  uri: &'a str,
  version: &'a str,
  message: Message<'a>,
}

impl<'a> Request<'a> {
  pub fn from_str(request_str: &'a str) -> Option<Self> {
    let mut lines = request_str.lines();

    let mut request_line = lines.next()?.split(' ');

    let method = {
      let method_str = request_line.next()?;
      if let Ok(m) = Methods::from_str(method_str) {
        m
      } else {
        return None;
      }
    };
    let uri = request_line.next()?;
    let version = request_line.next()?;

    let message = {
      let newline_char_len = 2;
      let remaining_bytes = lines.fold(0, |acc, line| acc + line.len() + newline_char_len);
      if let Some(m) = Message::from_str(&request_str[request_str.len() - remaining_bytes..]) {
        m
      } else {
        MessageBuilder::new().build()
      }
    };

    Some(Request {
      method,
      uri,
      version,
      message,
    })
  }
}

#[derive(Debug)]
pub enum Methods {
  GET,
  HEAD,
  PUT,
  POST,
}

impl Methods {
  fn from_str(method_str: &str) -> Result<Self, String> {
    match method_str {
      "GET" => Ok(Methods::GET),
      "HEAD" => Ok(Methods::HEAD),
      "PUT" => Ok(Methods::PUT),
      "POST" => Ok(Methods::POST),
      _ => Err(format!(
        "Error: Couldn't process the method: {}",
        method_str
      )),
    }
  }
}
