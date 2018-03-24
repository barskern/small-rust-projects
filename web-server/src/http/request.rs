use std::collections::HashMap;

#[derive(Debug)]
pub enum Methods {
  GET,
  PUT,
  POST,
}

impl Methods {
  fn from_str(method_str: &str) -> Result<Self, String> {
    match method_str {
      "GET" => Ok(Methods::GET),
      "PUT" => Ok(Methods::PUT),
      "POST" => Ok(Methods::POST),
      _ => Err(format!(
        "Error: Couldn't process the method: {}",
        method_str
      )),
    }
  }
}

#[derive(Debug)]
pub struct Request<'a> {
  method: Methods,
  uri: &'a str,
  version: &'a str,
  headers: HashMap<&'a str, &'a str>,
  body: Vec<&'a str>,
  raw_data: &'a str,
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

    let mut headers: HashMap<&'a str, &'a str> = HashMap::new();
    while let Some(header_line) = lines.next() {
      if header_line.is_empty() {
        // We found the start of the body!
        break;
      }
      let mut header_line_iter = header_line.splitn(2, ':').map(|s| s.trim());
      let name = header_line_iter.next().unwrap();
      let value = header_line_iter.next().unwrap();

      headers.insert(name, value);
    }

    Ok(Request {
      method,
      uri,
      version,
      headers,
      body: lines.collect::<Vec<&'a str>>(),
      raw_data: request_str,
    })
  }
}
