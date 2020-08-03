use clap::{ArgMatches, App, Arg, SubCommand};
use super::{RecallError, Result};
use crate::{app, cli};
use crate::list::{List, get_lists, list_exists};
use crate::card::Card;

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
    .about("Removes existing lists")
    .arg(
      Arg::with_name("names")
      .help("Name of the lists to remove")
      .multiple(true)
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

pub fn dispatch(matches: &ArgMatches) -> Result {
  match matches.subcommand() {
    ("add", Some(matches)) => add(matches),
    ("remove", Some(matches)) => remove(matches),
    ("append", Some(matches)) => append(matches),
    ("", Some(matches)) => list(matches),
    _ => unimplemented!()
  }
}

fn add(matches: &ArgMatches) -> Result {
  let name = matches.value_of("name").unwrap();

  if list_exists(".", name) {
    return Err(RecallError::new("List already exists."));
  }

  let list = List::new(format!("./{}.jsonl", name))
    .ok_or(RecallError::new("List initialization failed."))?;
  list.save_cards(vec![])
    .map_err(|_| RecallError::new("Add list failed."))?;

  println!();
  println!("List '{}' added.", name);
  println!();

  Ok(())
}

fn remove(matches: &ArgMatches) -> Result {
  let names = matches.values_of("names").unwrap();

  println!();

  for name in names {
    if !list_exists(".", name) {
      cli::print_error_strip(format!("List '{}' does not exist.", name));
      continue;
    }

    let list = List::new(format!("./{}.jsonl", name))
      .ok_or(RecallError::new("List initialization failed"))?;
    list.delete()
      .map_err(|_| RecallError::new("Remove list failed."))?;

    println!("List '{}' removed.", name);
  }

  println!();
  Ok(())
}

fn append(matches: &ArgMatches) -> Result {
  let name = matches.value_of("name").unwrap();

  if !list_exists(".", name) {
    return Err(RecallError::new("List does not exist."));
  }

  let list = List::new(format!("./{}.jsonl", name))
    .ok_or(RecallError::new("List initialization failed."))?;
  let mut cards = list.cards()
    .map_err(|_| RecallError::new("Failed to read cards."))
    .map(|cards| cards.collect::<Vec<_>>())?;

  println!();

  let front = cli::prompt("Front");
  let duplicate = cards.iter().any(|card| card.front == front);

  if duplicate {
    return Err(RecallError::new("Duplicate entry detected."));
  }

  let back = cli::prompt_multiline("Back");
  let notes = cli::prompt_multiline("Notes");
  let new_card = Card::new(front, back, notes);

  cards.push(new_card);
  list.save_cards(cards)
    .map_err(|_| RecallError::new("Failed to append card."))?;

  println!();
  println!("Card appended to list {}.", name);
  println!();

  Ok(())
}

fn list(_matches: &ArgMatches) -> Result {
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
      cli::inline_code(format!("{} list add <name>", app::name()))
    ),
  );
  println!();

  Ok(())
}
