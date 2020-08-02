extern crate clap;

mod commands;
mod list;

use commands::SubCommandDispatcher;
use clap::{Arg, App, SubCommand};

fn main() {
  let get_name_arg = || Arg::with_name("name").help("Name of the list");

  let matches = App::new(env!("CARGO_PKG_NAME"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .version(env!("CARGO_PKG_VERSION"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .subcommand(
      SubCommand::with_name("list")
      .about("Shows and manages lists")
      .subcommand(
        SubCommand::with_name("add")
        .about("Adds a new list")
        .arg(
          get_name_arg()
          .required(true)
        )
      )
      .subcommand(
        SubCommand::with_name("remove")
        .about("Removes an existing list")
        .arg(
          get_name_arg()
          .required(true)
        )
      )
    )
    .subcommand(
      SubCommand::with_name("info")
      .about("Shows overall progress on all lists or a specific one")
      .arg(get_name_arg())
    )
    .subcommand(
      SubCommand::with_name("study")
      .about("Starts a study session")
      .arg(get_name_arg())
    )
    .subcommand(
      SubCommand::with_name("learn")
      .about("Learn new cards")
      .arg(get_name_arg())
    )
    .get_matches();

  match matches.subcommand() {
    ("info", Some(matches)) => commands::InfoDispatcher::dispatch(matches),
    ("learn", Some(matches)) => commands::LearnDispatcher::dispatch(matches),
    ("list", Some(matches)) => commands::ListDispatcher::dispatch(matches),
    ("study", Some(matches)) => commands::StudyDispatcher::dispatch(matches),
    _ => panic!("No subcommand found")
  }
}
