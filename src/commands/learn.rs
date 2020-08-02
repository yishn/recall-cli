use clap::{ArgMatches};
use super::{Result, SubCommandDispatcher};

pub struct Dispatcher {}

impl SubCommandDispatcher for Dispatcher {
  fn dispatch(matches: &ArgMatches) -> Result {
    Some(matches);

    Ok(())
  }
}
