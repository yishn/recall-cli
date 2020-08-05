use std::time::Duration as StdDuration;
use clap::{ArgMatches, SubCommand, App, Arg};
use chrono::{Utc, Duration};
use humantime::format_duration;
use super::{RecallError, Result};
use crate::{card::{get_cards, list_proficiencies}, list::{list_exists, get_lists}, cli, app};

pub fn subcommand<'a>() -> App<'a, 'static> {
  SubCommand::with_name("info")
  .about("Shows overall progress on all lists or specific ones")
  .arg(
    Arg::with_name("names")
    .help("Name of the lists to see")
    .multiple(true)
  )
}

pub fn dispatch(matches: &ArgMatches) -> Result {
  let names = matches.values_of("names")
    .map(|names| names.collect::<Vec<_>>());
  let names_args = names.as_ref()
    .map(|names| " ".to_string() + &names.join(" "))
    .unwrap_or_else(|| String::new());
  let has_invalid_names = names.as_ref()
    .map(|names| names.iter().any(|name| !list_exists(".", name)))
    .unwrap_or(false);

  if has_invalid_names {
    println!();
    println!("List not found.");
    println!();

    cli::print_help_strip(
      format_args!(
        "Execute {} to get all lists",
        cli::inline_code(format_args!("{} list", app::name()))
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

  let cards = get_cards(
    get_lists(".")
    .map_err(|_| RecallError::new("Cannot read lists."))?
    .filter(|list| {
      names.as_ref()
      .map(|names| names.contains(&list.name()))
      .unwrap_or(true)
    })
  ).collect::<Vec<_>>();

  let total_count = cards.len();

  if total_count == 0 {
    println!();
    println!("No cards found.");
    println!();

    cli::print_help_strip(
      format_args!(
        "Execute {} to append new card to an existing list.",
        cli::inline_code(format_args!("{} list append <name>", app::name()))
      )
    );

    println!();
    return Ok(());
  }

  let critical_count = cards.iter()
    .filter(|card| card.critical())
    .count();

  let due_time = cards.iter()
    .filter_map(|card| card.due_time())
    .min();
  let due_count = cards.iter().filter(|card| card.is_due()).count();
  let due_next_hour_count = cards.iter()
    .filter(|card| {
      card.is_due_in(Utc::now() + Duration::hours(1))
    })
    .count();
  let due_tomorrow_count = cards.iter()
    .filter(|card| {
      card.is_due_in(Utc::now() + Duration::days(1))
    })
    .count();

  let count_by_proficiencies = list_proficiencies().into_iter()
    .map(|proficiency| (
      proficiency,
      cards.iter()
        .filter(|card| &card.proficiency() == &proficiency)
        .count()
    ))
    .collect::<Vec<_>>();

  let inactive_count = count_by_proficiencies[0].1;
  let col1_width = 14;
  let col2_width = count_by_proficiencies.iter()
    .map(|(_, count)| count.to_string().len())
    .max()
    .unwrap_or(0);

  let print_row = |text, value| {
    print!(
      "{:>col1_width$}:  {:<col2_width$}",
      text,
      value,
      col1_width = col1_width,
      col2_width = col2_width
    );
  };

  println!();
  cli::print_header_strip("Info");
  println!();

  if due_count > 0 {
    print_row("Due Now".to_string(), due_count.to_string());
  } else {
    print_row(
      "Next Review In".to_string(),
      due_time
        .and_then(|x| Duration::to_std(&(x - Utc::now())).ok())
        .map(|duration| StdDuration::new(duration.as_secs(), 0))
        .map(|duration| format_duration(duration).to_string())
        .unwrap_or("-".to_string())
    );
  }

  println!();
  print_row("Due Next Hour".to_string(), due_next_hour_count.to_string());
  println!();
  print_row("Due Tomorrow".to_string(), due_tomorrow_count.to_string());
  println!();
  println!();

  count_by_proficiencies.iter()
    .for_each(|&(proficiency, count)| {
      print_row(
        proficiency.colorize(&format!("{:>14}", proficiency.to_string())).to_string(),
        count.to_string()
      );

      println!("  {}", cli::progress_bar(count as f64 / total_count as f64, 18));
    });

  println!();

  if due_count > 0 {
    cli::print_help_strip(
      format_args!(
        "Execute {} to start a review session.",
        cli::inline_code(format_args!("{} review{}", app::name(), names_args))
      )
    );
    println!();
  } else if due_time.is_none() && inactive_count > 0 {
    cli::print_help_strip(
      format_args!(
        "Execute {} to learn some inactive cards.",
        cli::inline_code(format_args!("{} learn{}", app::name(), names_args))
      )
    );
    println!();
  }

  Ok(())
}
