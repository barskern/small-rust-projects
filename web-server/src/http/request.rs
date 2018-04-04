use super::content::Content;
use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub struct Request {
  method: Method,
  uri: String,
  version: String,
  content: Content,
}

#[derive(Debug, PartialEq)]
pub enum Method {
  GET,
  HEAD,
  PUT,
  POST,
}