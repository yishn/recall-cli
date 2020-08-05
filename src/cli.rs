use std::fmt::Display;
use colored::Colorize;
use rustyline::{error::ReadlineError, Editor, KeyPress, Cmd};

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

pub fn prompt<T: Display>(text: T) -> String {
  println!("{}:", text.to_string().bright_white());

  let mut editor = Editor::<()>::new();

  editor.readline("> ")
  .unwrap_or_else(|err| match err {
    ReadlineError::Eof => String::new(),
    ReadlineError::Interrupted => std::process::exit(130),
    err => panic!(err)
  })
}

pub fn prompt_multiline<T: Display>(text: T) -> String {
  println!("{}: {}", text.to_string().bright_white(), "(Press ^D to finish)".cyan());

  let mut editor = Editor::<()>::new();

  editor.bind_sequence(KeyPress::Enter, Cmd::Insert(1, "\n".to_string()));
  editor.bind_sequence(KeyPress::Ctrl('D'), Cmd::AcceptLine);

  editor.readline("> ")
  .unwrap_or_else(|err| match err {
    ReadlineError::Eof => String::new(),
    ReadlineError::Interrupted => std::process::exit(130),
    err => panic!(err)
  })
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
