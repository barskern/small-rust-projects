use std::{error, fmt, io, string};
use http::ParseHttpError;

#[derive(Debug)]
pub enum HandleStreamError {
  Io(io::Error),
  ParseHttp(ParseHttpError),
  ReadStream(ReadStreamError),
}

impl fmt::Display for HandleStreamError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      HandleStreamError::Io(ref err) => write!(f, "IO error: {}", err),
      HandleStreamError::ParseHttp(ref err) => write!(f, "Parse error: {}", err),
      HandleStreamError::ReadStream(ref err) => write!(f, "Read stream error: {}", err),
    }
  }
}

impl error::Error for HandleStreamError {
  fn description(&self) -> &str {
    match *self {
      HandleStreamError::Io(ref err) => err.description(),
      HandleStreamError::ParseHttp(ref err) => err.description(),
      HandleStreamError::ReadStream(ref err) => err.description(),
    }
  }

  fn cause(&self) -> Option<&error::Error> {
    match *self {
      HandleStreamError::Io(ref err) => Some(err),
      HandleStreamError::ParseHttp(ref err) => Some(err),
      HandleStreamError::ReadStream(ref err) => Some(err),
    }
  }
}

impl From<io::Error> for HandleStreamError {
  fn from(err: io::Error) -> HandleStreamError {
    HandleStreamError::Io(err)
  }
}

impl From<ParseHttpError> for HandleStreamError {
  fn from(err: ParseHttpError) -> HandleStreamError {
    HandleStreamError::ParseHttp(err)
  }
}

impl From<ReadStreamError> for HandleStreamError {
  fn from(err: ReadStreamError) -> HandleStreamError {
    HandleStreamError::ReadStream(err)
  }
}

#[derive(Debug)]
/// An error which is a wrapper around possible errors on reading a TcpStream.
pub enum ReadStreamError {
  Io(io::Error),
  Parse(string::FromUtf8Error),
}

impl fmt::Display for ReadStreamError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      ReadStreamError::Io(ref err) => write!(f, "IO error: {}", err),
      ReadStreamError::Parse(ref err) => write!(f, "Parse error: {}", err),
    }
  }
}

impl error::Error for ReadStreamError {
  fn description(&self) -> &str {
    match *self {
      ReadStreamError::Io(ref err) => err.description(),
      ReadStreamError::Parse(ref err) => err.description(),
    }
  }

  fn cause(&self) -> Option<&error::Error> {
    match *self {
      ReadStreamError::Io(ref err) => Some(err),
      ReadStreamError::Parse(ref err) => Some(err),
    }
  }
}

impl From<io::Error> for ReadStreamError {
  fn from(err: io::Error) -> ReadStreamError {
    ReadStreamError::Io(err)
  }
}

impl From<string::FromUtf8Error> for ReadStreamError {
  fn from(err: string::FromUtf8Error) -> ReadStreamError {
    ReadStreamError::Parse(err)
  }
}
