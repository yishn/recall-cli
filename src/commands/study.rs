use clap::{ArgMatches};
use super::SubCommandDispatcher;

pub struct Dispatcher {}

impl SubCommandDispatcher for Dispatcher {
  fn dispatch(matches: &ArgMatches) {
    Some(matches);
  }
}
