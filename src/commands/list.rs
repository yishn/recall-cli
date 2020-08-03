use clap::{ArgMatches, App, Arg, SubCommand};
use super::{RecallError, Result, SubCommandDispatcher};
use crate::list::get_lists;
use crate::cli;

pub fn subcommand<'a>() -> App<'a, 'static> {
  let get_name_arg = || Arg::with_name("name").help("Name of the list");

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
  .subcommand(
    SubCommand::with_name("append")
    .about("Appends a new card to an existing list")
    .arg(
      get_name_arg()
      .required(true)
    )
  )
}

pub struct Dispatcher;

impl SubCommandDispatcher for Dispatcher {
  fn dispatch(matches: &ArgMatches) -> Result {
    match matches.subcommand() {
      ("", None) => {
        // Get lists

        let lists = get_lists(".")
          .map_err(|_| RecallError::new("Unable to read from working directory"))?;

        println!();
        cli::print_header_strip("Lists");
        println!();

        if lists.len() > 0 {
          cli::print_bullet_list(
            lists.iter()
            .map(|list| list.name())
          );
        } else {
          println!("No lists found.");
        }

        println!();
        cli::print_help_strip(
          format!(
            "Add a new list by calling {}.",
            cli::inline_code("recall list add <name>")
          ),
        );
        println!();
      },
      _ => unimplemented!()
    }

    Ok(())
  }
}
