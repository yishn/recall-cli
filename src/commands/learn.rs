use clap::{ArgMatches, App, Arg, SubCommand};
use super::Result;

pub fn subcommand<'a>() -> App<'a, 'static> {
  SubCommand::with_name("learn")
  .about("Learn new cards")
  .arg(
    Arg::with_name("names")
    .help("Name of the lists to learn")
    .multiple(true)
    .required(true)
  )
  .arg(
    Arg::with_name("count")
    .short("c")
    .long("count")
    .help("The number of new cards you want to learn")
    .takes_value(true)
  )
}

pub fn dispatch(matches: &ArgMatches) -> Result {
  Some(matches);

  Ok(())
}
