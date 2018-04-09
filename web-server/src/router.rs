use std::collections::HashMap;
use std::fs::{DirEntry, File};
use std::path::{Path, PathBuf};
use std::io::Read;

use utils::visit_dir;
use http;

pub struct Router {
  paths: HashMap<String, String>,
}

impl Router {
  pub fn new(dir_path: &Path) -> Router {
    let mut paths: HashMap<String, String> = HashMap::new();

    visit_dir(dir_path, &mut |entry: DirEntry, dir_depth: usize| {
      let file_path = entry.path();
      let mut file = File::open(&file_path).unwrap();

      let mut file_contents = String::new();
      file.read_to_string(&mut file_contents);

      let uri = format!(
        "/{}",
        turn_path_into_uri(&file_path, dir_depth).to_str().unwrap()
      );

      paths.insert(uri, file_contents);
    });
    Router { paths }
  }

  pub fn handle_request(&self, request: http::Request) -> http::Response {
    if let Some(res_string) = self.paths.get(request.uri()) {
      http::Response::new(res_string.clone())
    } else {
      http::Response::not_found()
    }
  }
}

/// Turns a possible global file path into an uri path
fn turn_path_into_uri(path: &Path, dir_depth: usize) -> PathBuf {
  path
    .iter()
    .rev()
    .skip(1)
    .take(dir_depth)
    .collect::<Vec<_>>()
    .iter()
    .rev()
    .collect::<PathBuf>()
}

fn get_path(request: &str) -> Option<&str> {
  if let &Some(start) = &request.find("/") {
    if let &Some(end) = &request[start..].find(" ") {
      return if end > 0 {
        Some(&request[start..(start + end)])
      } else {
        None
      };
    }
  }
  None
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn good_get_path() {
    let path = get_path("GET /home/aabbcc HTTP1.1\n\r\n\r").unwrap();
    assert_eq!(path, "/home/aabbcc");
  }

  #[test]
  fn bad_get_path() {
    assert_eq!(get_path("GET HTTP1.1"), None);
  }

  #[test]
  fn good_turn_path_into_uri() {
    use std::path::PathBuf;
    let path = PathBuf::from(r"./html/about/us/index.html");
    let dir_depth = 2;
    let path_buf = turn_path_into_uri(&path, dir_depth);
    let path_str = path_buf.to_str().unwrap();

    assert_eq!(path_str, "about/us")
  }
}
