use std::fs::read_dir;
use std::path::Path;
use std::io::Result;

#[derive(Debug)]
pub struct List {
  pub path: String,
  pub name: String
}

impl List {
  pub fn new<P: AsRef<Path>>(path: P) -> Option<List> {
    let path = path.as_ref();
    if !path.is_file() {
      return None;
    }

    let full_path = path.to_str();
    let file_stem = path.file_stem().and_then(|x| x.to_str());
    let extension = path.extension().and_then(|x| x.to_str());

    match (full_path, file_stem, extension) {
      (Some(full_path), Some(file_stem), Some("tsv")) => Some(
        List {
          path: full_path.to_string(),
          name: file_stem.to_string()
        }
      ),
      _ => None
    }
  }
}

pub fn get_lists<P: AsRef<Path>>(dirname: P) -> Result<Vec<List>> {
  Ok(
    read_dir(dirname)?
    .filter_map(|entry| entry.ok())
    .filter_map(|entry| List::new(&entry.path()))
    .collect()
  )
}
