use clap::{ArgMatches};
use super::{Error, Result, SubCommandDispatcher};
use crate::list::{List, get_lists};
use crate::cli;

pub struct Dispatcher {}

impl SubCommandDispatcher for Dispatcher {
  fn dispatch(matches: &ArgMatches) -> Result {
    match matches.subcommand() {
      ("", None) => {
        // Get lists

        let lists = get_lists(".")
          .map_err(|_| Error::new("Unable to read from working directory"))?;

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
