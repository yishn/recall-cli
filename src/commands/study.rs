use clap::{ArgMatches, App, Arg, SubCommand};
use super::{Result, SubCommandDispatcher};

pub fn subcommand<'a>() -> App<'a, 'static> {
  let get_name_arg = || Arg::with_name("name").help("Name of the list");

  SubCommand::with_name("study")
  .about("Starts a study session")
  .arg(get_name_arg())
}

pub struct Dispatcher;

impl SubCommandDispatcher for Dispatcher {
  fn dispatch(matches: &ArgMatches) -> Result {
    Some(matches);

    Ok(())
  }
}
