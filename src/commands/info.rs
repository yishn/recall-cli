use clap::{ArgMatches, SubCommand, App, Arg};
use super::{Result, SubCommandDispatcher};

pub fn subcommand<'a>() -> App<'a, 'static> {
  let get_name_arg = || Arg::with_name("name").help("Name of the list");

  SubCommand::with_name("info")
  .about("Shows overall progress on all lists or a specific one")
  .arg(get_name_arg())
}

pub struct Dispatcher;

impl SubCommandDispatcher for Dispatcher {
  fn dispatch(matches: &ArgMatches) -> Result {
    Some(matches);

    Ok(())
  }
}
