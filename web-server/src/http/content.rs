use std::collections::HashMap;
use std::default::Default;

#[derive(Debug, PartialEq)]
pub struct Content<'a> {
  headers: HashMap<&'a str, &'a str>,
  body: &'a str,
}

impl<'a> Content<'a> {
  pub fn new(body: &'a str) -> Self {
    Content {
      headers: HashMap::new(),
      body,
    }
  }
}
