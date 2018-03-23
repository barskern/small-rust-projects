use std::collections::HashMap;
use std::fs;
use std::fs::{DirEntry, File};
use std::path::{Path, PathBuf};

pub struct Router {
  paths: HashMap<String, File>,
}

impl Router {
  pub fn new(dir_path: &Path) -> Router {
    let mut paths: HashMap<String, File> = HashMap::new();

    visit_dir(
      dir_path,
      &mut |entry, dir_depth| {
        let file_path = entry.path();
        let file = File::open(&file_path).unwrap();
        let uri = file_path
          .iter()
          .rev()
          .skip(1)
          .take(dir_depth)
          .collect::<Vec<_>>()
          .iter()
          .rev()
          .collect::<PathBuf>();

        paths.insert(String::from(uri.to_str().unwrap()), file);
      },
      0,
    );

    paths.iter().for_each(|(k, v)| println!("{:?}: {:?}", k, v));

    Router { paths }
  }

  pub fn handle_request(&self, request: &str) {
    // println!("{:?}", request);
  }
}

fn visit_dir2<F>(dir_path: &Path, f: &mut F, dir_depth: usize)
where
  F: FnMut(DirEntry, usize),
{
  for dir_entry in fs::read_dir(dir_path).expect("Wasn't able to read the html directory") {
    if let Some(dir_entry) = dir_entry.ok() {
      if fs::metadata(dir_entry.path()).unwrap().is_dir() {
        visit_dir(dir_entry.path().as_path(), f, dir_depth + 1)
      } else {
        f(dir_entry, dir_depth)
      }
    }
  }
}

fn visit_dir<'a, F>(dir_path: &'a Path, f: &mut F, dir_depth: usize)
where
  F: FnMut(DirEntry, usize),
{
  let (dirs, files): (Vec<&'a Path>, Vec<&'a Path>) = fs::read_dir(dir_path)
    .expect(&format!(
      "Wasn't able to read directory at path: {:?}",
      dir_path
    ))
    .into_iter()
    .map(|dir_entry| dir_entry.unwrap().path().as_path())
    .partition(|path| fs::metadata(path).unwrap().is_dir());

  // dirs
  //   .iter()
  //   .for_each(|path| visit_dir(path, f, dir_depth + 1));
  //
  // println!("dirs: {:?}", dirs);
  // println!("files: {:?}", files);
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
  fn test_good_get_path() {
    let path = get_path("GET /home/aabbcc HTTP1.1\n\r\n\r").unwrap();
    assert_eq!(path, "/home/aabbcc");
  }

  #[test]
  #[should_panic]
  fn test_bad_get_path() {
    get_path("GET HTTP1.1").unwrap();
  }
}
