use clap::{ArgMatches, App, Arg, SubCommand};
use super::Result;

pub fn subcommand<'a>() -> App<'a, 'static> {
  SubCommand::with_name("study")
  .about("Starts a study session")
  .arg(
    Arg::with_name("names")
    .help("Name of the lists to study")
    .multiple(true)
    .required(true)
  )
}

pub fn dispatch(matches: &ArgMatches) -> Result {
  Some(matches);

  Ok(())
}
