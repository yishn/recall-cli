use clap::{ArgMatches, App, Arg, SubCommand};
use super::{Result, SubCommandDispatcher};

pub fn subcommand<'a>() -> App<'a, 'static> {
  let get_name_arg = || Arg::with_name("name").help("Name of the list");

  SubCommand::with_name("learn")
  .about("Learn new cards")
  .arg(get_name_arg())
  .arg(
    Arg::with_name("count")
    .short("c")
    .long("count")
    .help("The number of new cards you want to learn")
    .takes_value(true)
  )
}

pub struct Dispatcher;

impl SubCommandDispatcher for Dispatcher {
  fn dispatch(matches: &ArgMatches) -> Result {
    Some(matches);

    Ok(())
  }
}
