mod cli;
mod commands;
mod list;
mod card;

use commands::RecallError;
use clap::App;

fn main() {
  let mut app = App::new(env!("CARGO_PKG_NAME"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .version(env!("CARGO_PKG_VERSION"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .subcommand(commands::list::subcommand())
    .subcommand(commands::info::subcommand())
    .subcommand(commands::study::subcommand())
    .subcommand(commands::learn::subcommand());

  let matches = app.clone().get_matches();

  let result = match matches.subcommand() {
    ("info", Some(matches)) => commands::info::dispatch(matches),
    ("learn", Some(matches)) => commands::learn::dispatch(matches),
    ("list", Some(matches)) => commands::list::dispatch(matches),
    ("study", Some(matches)) => commands::study::dispatch(matches),
    ("", _) => app.print_help().map_err(|_| RecallError::new("Printing help failed")),
    _ => Err(RecallError::new("Subcommand not found"))
  };

  match result {
    Err(err) => {
      println!();
      cli::print_error_strip(err);
      println!();
    },
    _ => {}
  };
}
