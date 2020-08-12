use clap::{ArgMatches, App, Arg, SubCommand};
use super::{RecallError, Result};
use crate::{list::{get_lists, list_exists}, cli, card::{update_cards, get_cards}, app};
use rand::prelude::SliceRandom;
use std::collections::VecDeque;

pub fn subcommand<'a>() -> App<'a, 'static> {
  SubCommand::with_name("review")
  .about("Starts a review session")
  .arg(
    Arg::with_name("names")
    .help("Name of the lists to review")
    .multiple(true)
  )
}

pub fn dispatch(matches: &ArgMatches) -> Result {
  let names = matches.values_of("names")
    .map(|names| names.collect::<Vec<_>>());

  let has_invalid_names = names.as_ref()
    .map(|names| names.iter().any(|name| !list_exists(".", name)))
    .unwrap_or(false);

  if has_invalid_names {
    println!();
    println!("List not found.");
    println!();

    cli::print_help_strip(
      format_args!(
        "Execute {} to review from all lists",
        cli::inline_code(format_args!("{} review", app::name()))
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
    .filter(|(_, card)| card.is_due())
    .collect::<Vec<_>>();

    let mut rng = rand::thread_rng();
    cards.shuffle(&mut rng);
    cards.into_iter().collect::<VecDeque<_>>()
  };

  println!();
  cli::print_header_strip("Review");

  if cards.len() > 0 {
    println!();
    let mut cards = cli::loop_cards(cards)?;
    let remembered_count = cards.iter().filter(|&&(_, _, remembered)| remembered).count();
    println!();
    println!("Reviewed {} card(s).", cards.len());
    println!();
    println!("Remembered:  {}", cli::progress_bar(remembered_count as f64 / cards.len() as f64, 18));
    println!();

    for &mut (_, ref mut card, remembered) in cards.iter_mut() {
      card.review(remembered);
    }

    update_cards(cards.into_iter().map(|(path, card, _)| (path, card)))
    .map_err(|_| RecallError::new("Updating cards failed."))?;
  } else {
    println!();
    println!("No cards to review right now.");
    println!();
    cli::print_help_strip(
      format_args!(
        "Check {} to see when review is due.",
        cli::inline_code(format_args!("{} info", app::name()))
      )
    );
    println!();
  };

  Ok(())
}
