use std::{
  fs::{self, DirEntry},
  path::{Path,PathBuf},
  io::Read,
  net::TcpStream
};

use super::errors::ReadStreamError;

const MAX_REQUEST_SIZE: usize = 1024;

/// Visits all files in from given dir to deepest nested 
/// subdir. Applies the function to all files.
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

/// Turns a possible global file path into an uri path
pub fn turn_path_into_uri(path: &Path, dir_depth: usize, inc_filename: bool) -> Option<String> {
  let skip_amount = if inc_filename { 0 } else { 1 };
  let take_amount = if inc_filename { dir_depth + 1 } else { dir_depth };

  path
    .iter()
    .rev()
    .skip(skip_amount)
    .take(take_amount)
    .collect::<Vec<_>>()
    .iter()
    .rev()
    .collect::<PathBuf>()
    .to_str()
    .map(|s| s.to_string())
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
