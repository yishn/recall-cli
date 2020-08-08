use std::fs::{read_dir, remove_file, File};
use std::path::{Path, PathBuf};
use std::io::{BufReader, BufRead, BufWriter, Write, Result};
use chrono::{Utc, DateTime};
use serde_json::{json, Value};
use crate::card::Card;

#[derive(Debug)]
pub struct List {
  path: PathBuf,
  name: String
}

impl List {
  pub fn new<P: AsRef<Path>>(path: P) -> Option<List> {
    let path = path.as_ref();
    if path.is_dir() {
      return None;
    }

    let full_path = path.to_str();
    let file_stem = path.file_stem().and_then(|x| x.to_str());

    match (full_path, file_stem) {
      (Some(full_path), Some(file_stem)) => Some(
        List {
          path: Path::new(full_path).to_path_buf(),
          name: file_stem.to_string()
        }
      ),
      _ => None
    }
  }

  pub fn path(&self) -> &Path {
    &self.path
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn cards(&self) -> Result<impl Iterator<Item = Card>> {
    let file = File::open(self.path())?;
    let buf_reader = BufReader::new(file);

    Ok(
      buf_reader.lines()
      .filter_map(|line| line.ok())
      .filter_map(|line| serde_json::from_str::<Vec<Value>>(&line).ok())
      .map(|arr| {
        let mut iter = arr.into_iter();

        let front = iter.next()
          .and_then(|x| serde_json::from_value::<String>(x).ok())
          .unwrap_or_else(|| String::new());
        let back = iter.next()
          .and_then(|x| serde_json::from_value::<String>(x).ok())
          .unwrap_or_else(|| String::new());
        let notes = iter.next()
          .and_then(|x| serde_json::from_value::<String>(x).ok())
          .unwrap_or_else(|| String::new());
        let level = iter.next()
          .and_then(|x| serde_json::from_value::<i8>(x).ok())
          .unwrap_or(0);
        let due_time = iter.next()
          .and_then(|x| serde_json::from_value::<String>(x).ok())
          .and_then(|x| DateTime::parse_from_rfc3339(&x).ok())
          .map(|date_time| date_time.with_timezone(&Utc));
        let correct_count = iter.next()
          .and_then(|x| serde_json::from_value::<u32>(x).ok())
          .unwrap_or(0);
        let total_count = iter.next()
          .and_then(|x| serde_json::from_value::<u32>(x).ok())
          .unwrap_or(0);

        let mut card = Card::new(front, back, notes);

        card.level = level;
        card.due_time = due_time;
        card.correct_count = correct_count;
        card.total_count = total_count;

        card
      })
    )
  }

  pub fn save_cards<I: IntoIterator<Item = Card>>(&self, cards: I) -> Result<()> {
    let file = File::create(self.path())?;
    let mut buf_writer = BufWriter::new(file);

    let lines = cards.into_iter()
      .map(|card| {
        serde_json::to_string(&json!([
          card.front,
          card.back,
          card.notes,
          card.level,
          card.due_time.map(|x| x.to_rfc3339()),
          card.correct_count,
          card.total_count
        ])).unwrap()
      });

    for line in lines {
      writeln!(buf_writer, "{}", line)?;
    }

    buf_writer.flush()?;
    Ok(())
  }

  pub fn delete(&self) -> Result<()> {
    remove_file(self.path())?;

    Ok(())
  }
}

pub fn get_lists<P: AsRef<Path>>(dirname: P) -> Result<impl Iterator<Item = List>> {
  Ok(
    read_dir(dirname)?
    .filter_map(|entry| entry.ok())
    .map(|entry| entry.path())
    .filter(|path| path.extension().and_then(|x| x.to_str()) == Some("jsonl"))
    .filter_map(|path| List::new(&path))
  )
}

pub fn list_exists<P: AsRef<Path>>(dirname: P, name: &str) -> bool {
  get_lists(dirname).ok()
  .map(|lists| lists.into_iter().any(|list| list.name() == name))
  .unwrap_or(false)
}
