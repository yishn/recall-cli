use std::collections::VecDeque;
use clap::{ArgMatches, App, Arg, SubCommand};
use rand::seq::SliceRandom;
use super::{RecallError, Result};
use crate::{cli, list::{get_lists, list_exists}, app, card::{Proficiency, get_cards}};

pub fn subcommand<'a>() -> App<'a, 'static> {
  SubCommand::with_name("learn")
  .about("Learn new cards")
  .arg(
    Arg::with_name("names")
    .help("Name of the lists to learn")
    .multiple(true)
  )
  .arg(
    Arg::with_name("count")
    .short("c")
    .long("count")
    .help("The maximum number of new cards you want to learn")
    .takes_value(true)
  )
}

pub fn dispatch(matches: &ArgMatches) -> Result {
  let names = matches.values_of("names")
    .map(|names| names.collect::<Vec<_>>());
  let count = matches.value_of("count")
    .map(|x| Ok(x))
    .map(|count| count.and_then(|x| {
      x.parse::<usize>()
      .map_err(|_| RecallError::new("Could not parse `count` option."))
    }));

  if let Some(Err(err)) = count {
    return Err(err);
  }

  let has_invalid_names = names.as_ref()
    .map(|names| names.iter().any(|name| !list_exists(".", name)))
    .unwrap_or(false);

  if has_invalid_names {
    println!();
    println!("List not found.");
    println!();

    cli::print_help_strip(
      format_args!(
        "Execute {} to learn from all lists",
        cli::inline_code(format_args!("{} learn", app::name()))
      )
    );

    cli::print_help_strip(
      format_args!(
        "Execute {} to add a new list",
        cli::inline_code(format_args!("{} list add <name>", app::name()))
      )
    );

    println!();
    return Ok(());
  }

  let cards = {
    let mut cards = get_cards(
      get_lists(".")
      .map_err(|_| RecallError::new("Cannot read lists."))?
      .filter(|list| {
        names.as_ref()
        .map(|names| names.contains(&list.name()))
        .unwrap_or(true)
      })
    )
    .filter(|(_, card)| card.proficiency() == Proficiency::Inactive)
    .collect::<Vec<_>>();

    let mut rng = rand::thread_rng();
    cards.shuffle(&mut rng);

    if let Some(Ok(count)) = count {
      cards.into_iter().take(count).collect::<VecDeque<_>>()
    } else {
      cards.into_iter().collect::<VecDeque<_>>()
    }
  };

  println!();
  cli::print_header_strip("Learning");

  println!();

  let cards = cli::loop_cards(cards)?;

  println!();
  println!("Learned {} new card(s).", cards.len());
  println!();

  Ok(())
}
