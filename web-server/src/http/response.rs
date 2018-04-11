use std::fmt::{self, Display};
use super::{content::{Content, Contentable}, HTTP_VERSION};

/// A struct which is a representation of a http-response
/// message. When written to string, it is valid http, which can 
/// be sent directly across a TCP-connection.
#[derive(Debug, PartialEq)]
pub struct Response {
  status_code: StatusCode,
  content: Content,
}

impl Response {
  pub fn new(content: String) -> Response {
    Response {
      status_code: StatusCode::Accepted,
      content: Content::new(content),
    }
  }

  pub fn not_found() -> Response {
    Response {
      status_code: StatusCode::NotFound,
      content: Content::default(),
    }
  }
}

impl Contentable for Response {
  fn get_body(&self) -> &str {
    self.content.get_body()
  }
  fn set_body(&mut self, new_body: String) -> String {
    self.content.set_body(new_body)    
  }
  fn has_header(&self, name: &str) -> Option<&str> {
    self.content.has_header(name)
  }
  fn add_header(&mut self, name: String, value: String) -> Option<String> {
    self.content.add_header(name, value)
  }
}

impl Display for Response {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    write!(
      fmt,
      "{} {} {}\r\n{}",
      HTTP_VERSION,
      self.status_code,
      self.status_code.to_reason_phrase(),
      self.content
    )
  }
}

/// Encodes the status of a http-response
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum StatusCode {
  Continue = 100,
  SwitchingProtocols = 101,
  OK = 200,
  Created = 201,
  Accepted = 202,
  NonAuthoritativeInformation = 203,
  NoContent = 204,
  ResetContent = 205,
  PartialContent = 206,
  MultipleChoices = 300,
  MovedPermanently = 301,
  Found = 302,
  SeeOther = 303,
  NotModified = 304,
  UseProxy = 305,
  TemporaryRedirect = 307,
  BadRequest = 400,
  Unauthorized = 401,
  PaymentRequired = 402,
  Forbidden = 403,
  NotFound = 404,
  MethodNotAllowed = 405,
  NotAcceptable = 406,
  ProxyAuthenticationRequired = 407,
  RequestTimeout = 408,
  Conflict = 409,
  Gone = 410,
  LengthRequired = 411,
  PreconditionFailed = 412,
  RequestEntityTooLarge = 413,
  RequestURITooLarge = 414,
  UnsupportedMediaType = 415,
  Requestedrangenotsatisfiable = 416,
  ExpectationFailed = 417,
  InternalServerError = 500,
  NotImplemented = 501,
  BadGateway = 502,
  ServiceUnavailable = 503,
  GatewayTimeout = 504,
  HTTPVersionnotsupported = 505,
}

impl StatusCode {
  pub fn to_reason_phrase(&self) -> String {
    match *self {
      StatusCode::Continue => "Continue",
      StatusCode::SwitchingProtocols => "Switching Protocols",
      StatusCode::OK => "OK",
      StatusCode::Created => "Created",
      StatusCode::Accepted => "Accepted",
      StatusCode::NonAuthoritativeInformation => "Non Authoritative Information",
      StatusCode::NoContent => "No Content",
      StatusCode::ResetContent => "Reset Content",
      StatusCode::PartialContent => "Partial Content",
      StatusCode::MultipleChoices => "Multiple Choices",
      StatusCode::MovedPermanently => "Moved Permanently",
      StatusCode::Found => "Found",
      StatusCode::SeeOther => "See Other",
      StatusCode::NotModified => "Not Modified",
      StatusCode::UseProxy => "Use Proxy",
      StatusCode::TemporaryRedirect => "Temporary Redirect",
      StatusCode::BadRequest => "Bad Request",
      StatusCode::Unauthorized => "Unauthorized",
      StatusCode::PaymentRequired => "Payment Required",
      StatusCode::Forbidden => "Forbidden",
      StatusCode::NotFound => "Not Found",
      StatusCode::MethodNotAllowed => "Method Not Allowed",
      StatusCode::NotAcceptable => "Not Acceptable",
      StatusCode::ProxyAuthenticationRequired => "Proxy Authentication Required",
      StatusCode::RequestTimeout => "Request Timeout",
      StatusCode::Conflict => "Conflict",
      StatusCode::Gone => "Gone",
      StatusCode::LengthRequired => "Length Required",
      StatusCode::PreconditionFailed => "Precondition Failed",
      StatusCode::RequestEntityTooLarge => "Request Entity Too Large",
      StatusCode::RequestURITooLarge => "Request-URI Too Large",
      StatusCode::UnsupportedMediaType => "Unsupported Media Type",
      StatusCode::Requestedrangenotsatisfiable => "Requested range not satisfiable",
      StatusCode::ExpectationFailed => "Expectation Failed",
      StatusCode::InternalServerError => "Internal Server Error",
      StatusCode::NotImplemented => "Not Implemented",
      StatusCode::BadGateway => "Bad Gateway",
      StatusCode::ServiceUnavailable => "Service Unavailable",
      StatusCode::GatewayTimeout => "Gateway Timeout",
      StatusCode::HTTPVersionnotsupported => "HTTP-Version not supported",
    }.to_string()
  }
}

impl Display for StatusCode {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    write!(fmt, "{}", *self as u16)
  }
}

#[cfg(test)]
mod tests {
  // use super::*;
  // TODO
}
