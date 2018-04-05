use std::fs::{self, DirEntry};
use std::path::Path;
use std::io::{self, Read};
use std::string;
use std::error;
use std::fmt;
use std::net::TcpStream;

const MAX_REQUEST_SIZE: usize = 1024;

/// Visits all files in from given dir to deepest nested subdir. Applies the function to all files.
pub fn visit_dir<F>(dir_path: &Path, f: &mut F)
where
  F: FnMut(DirEntry, usize),
{
  _visit_dir(dir_path, f, 0)
}

/// Private function to keep track of current depth of recursion
fn _visit_dir<F>(dir_path: &Path, f: &mut F, dir_depth: usize)
where
  F: FnMut(DirEntry, usize),
{
  let dir_entries = fs::read_dir(dir_path).expect(&format!(
    "Wasn't able to read directory at path: {:?}",
    dir_path
  ));

  let (dirs, files): (Vec<DirEntry>, Vec<DirEntry>) = dir_entries
    .map(|dir_entry| dir_entry.unwrap())
    .partition(|dir_entry| fs::metadata(dir_entry.path()).unwrap().is_dir());

  dirs
    .into_iter()
    .map(|dir_entry| dir_entry.path())
    .for_each(|path_buf| _visit_dir(&path_buf, f, dir_depth + 1));

  files
    .into_iter()
    .for_each(|dir_entry| f(dir_entry, dir_depth));
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

/// Reads request from stream and returns a string which contains the request in UTF8
pub fn read_string_from_stream(stream: &mut TcpStream) -> Result<String, ReadStreamError> {
  let mut buffer: Box<[u8]> = Box::new([0; MAX_REQUEST_SIZE]);
  let bytes_read = stream.read(&mut buffer)?;
  let mut buffer_vec = buffer.into_vec();
  buffer_vec.truncate(bytes_read);

  let res = String::from_utf8(buffer_vec)?;
  Ok(res)
}
