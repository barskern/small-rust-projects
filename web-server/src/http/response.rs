use super::message::{Message, MessageBuilder};

pub struct Response<'a> {
  version: &'a str,
  status_code: StatusCode,
  reason_phrase: &'a str,
  message: Message<'a>,
}

pub struct ResponseBuilder<'a> {
  __version: &'a str,
  __status_code: StatusCode,
  __reason_phrase: &'a str,
  __message_builder: MessageBuilder<'a>,
}

impl<'a> ResponseBuilder<'a> {
  pub fn new(status_code: StatusCode, body: &'a str) -> Self {
    ResponseBuilder {
      __status_code: status_code,
      __reason_phrase: "",
      __version: "HTTP/1.1",
      __message_builder: MessageBuilder::new().body(body),
    }
  }

  pub fn reason(mut self, reason_phrase: &'a str) -> Self {
    self.__reason_phrase = reason_phrase;
    self
  }

  pub fn status(mut self, status_code: StatusCode) -> Self {
    self.__status_code = status_code;
    self
  }

  pub fn version(mut self, version: &'a str) -> Self {
    self.__version = version;
    self
  }

  pub fn add_header(mut self, name: &'a str, value: &'a str) -> Self {
    self.__message_builder = self.__message_builder.add_header(name, value);
    self
  }

  pub fn build(self) -> Response<'a> {
    Response {
      version: self.__version,
      status_code: self.__status_code,
      reason_phrase: self.__reason_phrase,
      message: self.__message_builder.build(),
    }
  }
}

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
