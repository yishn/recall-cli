use clap::{ArgMatches, App, Arg, SubCommand};
use super::Result;

pub fn subcommand<'a>() -> App<'a, 'static> {
  SubCommand::with_name("review")
  .about("Starts a review session")
  .arg(
    Arg::with_name("names")
    .help("Name of the lists to review")
    .multiple(true)
    .required(true)
  )
}

pub fn dispatch(matches: &ArgMatches) -> Result {
  Some(matches);

  Ok(())
}
