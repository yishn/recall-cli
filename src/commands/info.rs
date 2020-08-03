use clap::{ArgMatches, SubCommand, App, Arg};
use super::Result;

pub fn subcommand<'a>() -> App<'a, 'static> {
  SubCommand::with_name("info")
  .about("Shows overall progress on all lists or a specific one")
  .arg(
    Arg::with_name("names")
    .help("Name of the lists to see")
    .multiple(true)
    .required(true)
  )
}

pub fn dispatch(matches: &ArgMatches) -> Result {
  Some(matches);

  Ok(())
}
