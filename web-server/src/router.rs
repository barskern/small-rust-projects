use std::{collections::HashMap, fs::{DirEntry, File}, io::Read, path::{Path, PathBuf}};

use http;
use utils;

pub struct Router {
  paths: HashMap<String, String>,
}

impl Router {
  pub fn new(dir_path: &Path) -> Router {
    let mut paths: HashMap<String, String> = HashMap::new();

    utils::visit_dir(dir_path, &mut |entry: DirEntry, dir_depth: usize| {
      let file_path = entry.path();
      let mut file = File::open(&file_path).expect(&format!(
        "Unable to open file at: {}",
        file_path.to_string_lossy()
      ));

      let mut file_contents = String::new();
      let _ = file.read_to_string(&mut file_contents);

      let uri = utils::turn_path_into_uri(&file_path, dir_depth, false)
        .expect("Unable to turn filepath into uri");

      paths.insert(uri, file_contents);
    });
    Router { paths }
  }

  pub fn handle_request(&self, request: http::Request) -> http::Response {
    if let Some(res_string) = self.paths.get(request.uri()) {
      http::Response::new(http::StatusCode::OK, res_string.clone())
    } else {
      http::Response::new(http::StatusCode::NotFound, "".to_string())
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn turn_path_into_uri_valid() {
    let path: PathBuf = PathBuf::from(r"./html/about/us/index.html");
    let dir_depth = 2;
    let path_str =
      utils::turn_path_into_uri(&path, dir_depth, false).expect("Unable to turn filepath into uri");

    assert_eq!(path_str, "/about/us/");
  }
}
