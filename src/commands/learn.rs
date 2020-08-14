use clap::{ArgMatches, App, Arg, SubCommand};
use rand::seq::SliceRandom;
use super::{RecallError, Result};
use crate::{cli, list::{get_lists, list_exists}, app, card::{Proficiency, get_cards, update_cards}};

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
  let count = Ok(matches.value_of("count"))
    .and_then(|count| {
      count
      .map(|x| {
        x.parse::<usize>()
        .map(|x| Some(x))
        .map_err(|_| RecallError::new("Could not parse `count` option."))
      })
      .unwrap_or(Ok(None))
    })?;

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

    if let Some(count) = count {
      cards.truncate(count);
    }

    cards
  };

  println!();
  cli::print_header_strip("Learning");

  if cards.len() > 0 {
    println!();
    let mut cards = cli::loop_cards(cards)?;
    println!();
    println!("Learned {} new card(s).", cards.len());
    println!();

    for &mut (_, ref mut card, remembered) in cards.iter_mut() {
      card.review(remembered);
    }

    update_cards(cards.into_iter().map(|(path, card, _)| (path, card)))
    .map_err(|_| RecallError::new("Updating cards failed."))?;
  } else {
    println!();
    println!("No new cards to learn.");
    println!();
  }

  Ok(())
}
