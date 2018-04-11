use std::{convert::TryFrom, fmt::{self, Display}, str::FromStr};

pub use super::{content::{Content, Contentable},
                errors::{ParseResponseError, ParseStatusCodeError}};
use super::HTTP_VERSION;

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

impl TryFrom<String> for Response {
  type Error = ParseResponseError;

  fn try_from(mut s: String) -> Result<Self, Self::Error> {
    if s.len() == 0 {
      return Err(ParseResponseError::empty());
    }

    let content_str = {
      let newline_pos = s.find("\r\n")
        .map(|pos| pos + 2)
        .or(s.find('\n').map(|pos| pos + 1))?;
      s.split_off(newline_pos)
    };

    let response_line: Vec<&str> = s.split_whitespace().collect();

    if response_line.len() < 3 {
      return Err(ParseResponseError::invalid());
    }
    let version = response_line[0];
    if version != HTTP_VERSION {
      return Err(ParseResponseError::invalid());
    }

    let status_code = StatusCode::from_str(response_line[1])?;
    let content = Content::try_from(content_str)?;

    Ok(Response {
      status_code,
      content,
    })
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
    use self::StatusCode::*;
    match *self {
      Continue => "Continue",
      SwitchingProtocols => "Switching Protocols",
      OK => "OK",
      Created => "Created",
      Accepted => "Accepted",
      NonAuthoritativeInformation => "Non Authoritative Information",
      NoContent => "No Content",
      ResetContent => "Reset Content",
      PartialContent => "Partial Content",
      MultipleChoices => "Multiple Choices",
      MovedPermanently => "Moved Permanently",
      Found => "Found",
      SeeOther => "See Other",
      NotModified => "Not Modified",
      UseProxy => "Use Proxy",
      TemporaryRedirect => "Temporary Redirect",
      BadRequest => "Bad Request",
      Unauthorized => "Unauthorized",
      PaymentRequired => "Payment Required",
      Forbidden => "Forbidden",
      NotFound => "Not Found",
      MethodNotAllowed => "Method Not Allowed",
      NotAcceptable => "Not Acceptable",
      ProxyAuthenticationRequired => "Proxy Authentication Required",
      RequestTimeout => "Request Timeout",
      Conflict => "Conflict",
      Gone => "Gone",
      LengthRequired => "Length Required",
      PreconditionFailed => "Precondition Failed",
      RequestEntityTooLarge => "Request Entity Too Large",
      RequestURITooLarge => "Request-URI Too Large",
      UnsupportedMediaType => "Unsupported Media Type",
      Requestedrangenotsatisfiable => "Requested range not satisfiable",
      ExpectationFailed => "Expectation Failed",
      InternalServerError => "Internal Server Error",
      NotImplemented => "Not Implemented",
      BadGateway => "Bad Gateway",
      ServiceUnavailable => "Service Unavailable",
      GatewayTimeout => "Gateway Timeout",
      HTTPVersionnotsupported => "HTTP-Version not supported",
    }.to_string()
  }
}

impl TryFrom<u16> for StatusCode {
  type Error = ParseStatusCodeError;

  fn try_from(num: u16) -> Result<Self, Self::Error> {
    use self::StatusCode::*;
    match num {
      num if num == Continue as u16 => Ok(Continue),
      num if num == SwitchingProtocols as u16 => Ok(SwitchingProtocols),
      num if num == OK as u16 => Ok(OK),
      num if num == Created as u16 => Ok(Created),
      num if num == Accepted as u16 => Ok(Accepted),
      num if num == NonAuthoritativeInformation as u16 => Ok(NonAuthoritativeInformation),
      num if num == NoContent as u16 => Ok(NoContent),
      num if num == ResetContent as u16 => Ok(ResetContent),
      num if num == PartialContent as u16 => Ok(PartialContent),
      num if num == MultipleChoices as u16 => Ok(MultipleChoices),
      num if num == MovedPermanently as u16 => Ok(MovedPermanently),
      num if num == Found as u16 => Ok(Found),
      num if num == SeeOther as u16 => Ok(SeeOther),
      num if num == NotModified as u16 => Ok(NotModified),
      num if num == UseProxy as u16 => Ok(UseProxy),
      num if num == TemporaryRedirect as u16 => Ok(TemporaryRedirect),
      num if num == BadRequest as u16 => Ok(BadRequest),
      num if num == Unauthorized as u16 => Ok(Unauthorized),
      num if num == PaymentRequired as u16 => Ok(PaymentRequired),
      num if num == Forbidden as u16 => Ok(Forbidden),
      num if num == NotFound as u16 => Ok(NotFound),
      num if num == MethodNotAllowed as u16 => Ok(MethodNotAllowed),
      num if num == NotAcceptable as u16 => Ok(NotAcceptable),
      num if num == ProxyAuthenticationRequired as u16 => Ok(ProxyAuthenticationRequired),
      num if num == RequestTimeout as u16 => Ok(RequestTimeout),
      num if num == Conflict as u16 => Ok(Conflict),
      num if num == Gone as u16 => Ok(Gone),
      num if num == LengthRequired as u16 => Ok(LengthRequired),
      num if num == PreconditionFailed as u16 => Ok(PreconditionFailed),
      num if num == RequestEntityTooLarge as u16 => Ok(RequestEntityTooLarge),
      num if num == RequestURITooLarge as u16 => Ok(RequestURITooLarge),
      num if num == UnsupportedMediaType as u16 => Ok(UnsupportedMediaType),
      num if num == Requestedrangenotsatisfiable as u16 => Ok(Requestedrangenotsatisfiable),
      num if num == ExpectationFailed as u16 => Ok(ExpectationFailed),
      num if num == InternalServerError as u16 => Ok(InternalServerError),
      num if num == NotImplemented as u16 => Ok(NotImplemented),
      num if num == BadGateway as u16 => Ok(BadGateway),
      num if num == ServiceUnavailable as u16 => Ok(ServiceUnavailable),
      num if num == GatewayTimeout as u16 => Ok(GatewayTimeout),
      num if num == HTTPVersionnotsupported as u16 => Ok(HTTPVersionnotsupported),
      _ => Err(ParseStatusCodeError::invalid()),
    }
  }
}

impl FromStr for StatusCode {
  type Err = ParseStatusCodeError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let num = s.parse::<u16>()
      .map_err(|_| ParseStatusCodeError::invalid())?;
    StatusCode::try_from(num)
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
