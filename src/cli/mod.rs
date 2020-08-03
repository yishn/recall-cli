use std::fmt::Display;
use colored::Colorize;
use std::io::{stdin, stdout, Write, BufRead};

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
  println!("{}:", text);
  print!("{}", "> ".bright_black());
  stdout().flush().unwrap();

  let mut result = String::new();
  stdin().read_line(&mut result).unwrap();

  result.trim_end().to_string()
}

pub fn prompt_multiline<T: Display>(text: T) -> String {
  println!("{}: {}", text, "(Press ^D and enter to finish)".cyan());
  print!("{}", "> ".bright_black());
  stdout().flush().unwrap();

  let mut buf = Vec::new();
  let stdin = stdin();
  let mut handle = stdin.lock();

  // Read until EOT (^D)
  handle.read_until(4, &mut buf).unwrap();
  // Don't let residue bleed into next prompt
  handle.read_line(&mut String::new()).unwrap();
  // Remove EOT character
  buf.pop();

  String::from_utf8(buf).unwrap().trim_end().to_string()
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
