use std::{collections::{VecDeque, HashSet}, fmt::Display, path::PathBuf};
use colored::Colorize;
use rustyline::{error::ReadlineError, Editor, KeyPress, Cmd};
use crate::{card::Card, commands::RecallError, list::List};

pub fn print_strip<S: Display, T: Display>(symbol: S, text: T) {
  println!("{} {}", symbol, text);
}

pub fn print_header_strip<T: Display>(text: T) {
  print_strip("#", text.to_string().bold());
}

pub fn print_error_strip<T: Display>(text: T) {
  print_strip("!".red().bold(), text);
}

pub fn print_help_strip<T: Display>(text: T) {
  print_strip("?".blue().bold(), text);
}

pub fn print_bullet_list<D: Display, I: IntoIterator<Item = D>>(list: I) {
  for item in list {
    print_strip("*".bright_black(), item);
  }
}

pub fn prompt<T: Display>(text: T) -> Result<String, RecallError> {
  prompt_with_prefix(text, "> ")
}

pub fn prompt_with_prefix<T: Display, P: AsRef<str>>(text: T, prefix: P) -> Result<String, RecallError> {
  let text = text.to_string();

  if text.len() > 0 {
    println!("{}:", text.bright_white());
  }

  let mut editor = Editor::<()>::new();

  editor.readline(prefix.as_ref())
  .map(|x| Ok(x))
  .unwrap_or_else(|err| match err {
    ReadlineError::Eof => Ok(String::new()),
    ReadlineError::Interrupted => std::process::exit(130),
    _ => Err(RecallError::new("Unable to prompt user."))
  })
}

pub fn prompt_multiline<T: Display>(text: T) -> Result<String, RecallError> {
  prompt_multiline_with_initial(text, ("", ""))
}

pub fn prompt_multiline_with_initial<T: Display>(text: T, initial: (&str, &str)) -> Result<String, RecallError> {
  let text = text.to_string();

  if text.len() > 0 {
    print!("{}: ", text.bright_white());
  }

  println!("{}", "(Press ^D to finish)".cyan());

  let mut editor = Editor::<()>::new();

  editor.bind_sequence(KeyPress::Enter, Cmd::Insert(1, "\n".to_string()));
  editor.bind_sequence(KeyPress::Ctrl('D'), Cmd::AcceptLine);

  editor.readline_with_initial("> ", initial)
  .map(|x| Ok(x))
  .unwrap_or_else(|err| match err {
    ReadlineError::Eof => Ok(String::new()),
    ReadlineError::Interrupted => std::process::exit(130),
    _ => Err(RecallError::new("Unable to prompt user."))
  })
}

pub fn prompt_multiple_choice<T: Display>(text: T, hotkeys: &[char]) -> Result<char, RecallError> {
  let text = text.to_string();

  if text.len() > 0 {
    print!("{}: ", text.bright_white());
  }

  let mut editor = Editor::<()>::new();

  loop {
    let input = editor.readline("> ")
      .map(|x| Ok(x))
      .unwrap_or_else(|err| match err {
        ReadlineError::Eof => Ok(String::new()),
        ReadlineError::Interrupted => std::process::exit(130),
        _ => Err(RecallError::new("Unable to prompt user."))
      })?;

    let chars = input.chars().collect::<Vec<_>>();

    if chars.len() == 1 && hotkeys.contains(&chars[0]) {
      break Ok(chars[0].to_ascii_lowercase());
    }
  }
}

pub fn inline_code<T: Display>(code: T) -> impl Display {
  format!("`{}`", code).cyan()
}

pub fn progress_bar(progress: f64, width: u32) -> impl Display {
  let percent = (progress * 100.0).round() as u32;
  let completed = ((width as f64 * progress).round() as u32).min(width);
  let incomplete = width - completed;
  let mut result = String::new();

  result += "[";

  for _ in 0..completed {
    result += &"#".green().to_string();
  }

  for _ in 0..incomplete {
    result += &"-".bright_black().to_string();
  }

  result += "] ";
  result += &percent.to_string();
  result += "%";
  result
}

pub fn loop_cards(
  mut cards: VecDeque<(PathBuf, Card)>
) -> Result<Vec<(PathBuf, Card, bool)>, RecallError> {
  let total_count = cards.len();
  let mut result = Vec::new();
  let mut shown_again = HashSet::new();
  let mut first = true;

  while let Some((path, mut card)) = cards.pop_front() {
    let list = List::new(&path);
    if let None = list { continue; }

    let list = list.unwrap();
    let left_count = cards.len();
    let proficiency = card.proficiency();

    loop {
      if first {
        first = false;
      } else {
        println!();
      }

      println!(
        "{} - {}",
        format!(
          "{}/{}",
          total_count - left_count,
          total_count
        ).green(),
        proficiency.colorize(proficiency.to_string())
      );
      println!("List: {}", list.name());
      println!();
      println!("  {}", card.front);
      prompt_with_prefix("", "")?;
      println!("{} {}", "Back:".bright_white(), card.back);
      println!("{} {}", "Notes:".bright_white(), card.notes);
      println!();

      let choice = prompt_multiple_choice(
        format_args!(
          "show {}gain, {}dit notes, {}ext card - {}inish",
          "a".bold().cyan().underline(),
          "e".bold().cyan().underline(),
          "n".bold().cyan().underline(),
          "f".bold().cyan().underline()
        ),
        &['a', 'e', 'n', 'f']
      )?;

      match choice {
        'a' => {
          shown_again.insert((path.clone(), card.line_number));
          cards.push_back((path, card));

          break;
        },
        'n' => {
          let line_number = card.line_number;

          result.push((
            path.clone(),
            card,
            !shown_again.contains(&(path, line_number))
          ));

          break;
        },
        'f' => break,
        'e' => {
          let notes = prompt_multiline_with_initial("Notes", (&card.notes, ""))?;
          card.notes = notes;
        },
        _ => unreachable!()
      }
    }
  }

  Ok(result)
}
