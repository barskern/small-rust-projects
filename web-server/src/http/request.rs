use super::message::Message;

#[derive(Debug)]
pub struct Request<'a> {
  method: Methods,
  uri: &'a str,
  version: &'a str,
  message: Message<'a>,
}

impl<'a> Request<'a> {
  pub fn from_str(request_str: &'a str) -> Result<Self, String> {
    let mut lines = request_str.lines();

    let mut request_line = lines.next().unwrap().split(' ');

    let method = {
      let method_str = request_line.next().unwrap();
      Methods::from_str(method_str)?
    };
    let uri = request_line.next().unwrap();
    let version = request_line.next().unwrap();

    let message = Message::from_lines(lines);

    Ok(Request {
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
