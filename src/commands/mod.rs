use clap::ArgMatches;
use std::fmt::Display;

mod info;
mod learn;
mod list;
mod study;

pub use info::Dispatcher as InfoDispatcher;
pub use learn::Dispatcher as LearnDispatcher;
pub use list::Dispatcher as ListDispatcher;
pub use study::Dispatcher as StudyDispatcher;

pub struct Error {
  pub message: &'static str
}

impl Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl Error {
  pub fn new(message: &'static str) -> Error {
    Error {
      message
    }
  }
}

pub type Result = std::result::Result<(), Error>;

pub trait SubCommandDispatcher {
  fn dispatch(matches: &ArgMatches) -> Result;
}
