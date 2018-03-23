use std::collections::HashMap;

pub struct HTTPRequest<'a> {
  method: &'a str,
  uri: &'a str,
  version: &'a str,
  headers: HashMap<&'a str, &'a str>,
  body: &'a str,
  raw_data: &'a str,
}

impl<'a> From<&'a str> for HTTPRequest<'a> {
  fn from(request_str: &'a str) -> Self {
    HTTPRequest {
      method: "GET",
      uri: "",
      version: "HTTP/1.1",
      headers: HashMap::new(),
      body: "",
      raw_data: request_str,
    }
  }
}
