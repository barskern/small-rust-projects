use std::fs::{self, DirEntry};
use std::path::Path;

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
