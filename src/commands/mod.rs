use std::fmt::Display;
use std::error::Error;

pub mod info;
pub mod learn;
pub mod list;
pub mod study;

#[derive(Debug)]
pub struct RecallError {
  pub message: &'static str
}

impl Error for RecallError {}

impl Display for RecallError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl RecallError {
  pub fn new(message: &'static str) -> RecallError {
    RecallError {
      message
    }
  }
}

pub type Result = std::result::Result<(), RecallError>;
