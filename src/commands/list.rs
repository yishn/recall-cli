use clap::{ArgMatches};
use super::{Error, Result, SubCommandDispatcher};
use crate::list;

pub struct Dispatcher {}

impl SubCommandDispatcher for Dispatcher {
  fn dispatch(matches: &ArgMatches) -> Result {
    match matches.subcommand() {
      ("", None) => {
        // Get lists

        let lists = list::get_lists(".")
          .map_err(|_| Error::new("Unable to read from working directory"))?;

        println!();
        println!("# Lists");
        println!();

        if lists.len() > 0 {
          for list in lists {
            println!("* {}", list.name);
          }
        } else {
          println!("No lists found.");
          println!();
          println!("? Add a new list by calling `recall list add <name>`.");
        }

        println!();
      },
      _ => unimplemented!()
    }

    Ok(())
  }
}
