use std::collections::HashMap;

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
