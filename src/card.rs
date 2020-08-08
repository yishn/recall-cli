use chrono::{offset::Utc, DateTime, Duration, TimeZone};
use crate::list::List;
use std::{path::PathBuf, fmt::Display};
use colored::{ColoredString, Colorize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Proficiency {
  Inactive,
  Apprentice,
  Guru,
  Master,
  Enlightened,
  Burned
}

impl Proficiency {
  pub fn colorize(&self, text: &str) -> ColoredString {
    match self {
      Proficiency::Inactive => text.bright_black(),
      Proficiency::Apprentice => text.bright_red(),
      Proficiency::Guru => text.bright_yellow(),
      Proficiency::Master => text.bright_cyan(),
      Proficiency::Enlightened => text.bright_blue(),
      Proficiency::Burned => text.bright_purple()
    }
  }
}

impl Display for Proficiency {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.pad(match self {
      Proficiency::Inactive => "Inactive",
      Proficiency::Apprentice => "Apprentice",
      Proficiency::Guru => "Guru",
      Proficiency::Master => "Master",
      Proficiency::Enlightened => "Enlightened",
      Proficiency::Burned => "Burned"
    })
  }
}

pub fn list_proficiencies() -> Vec<Proficiency> {
  vec![
    Proficiency::Inactive,
    Proficiency::Apprentice,
    Proficiency::Guru,
    Proficiency::Master,
    Proficiency::Enlightened,
    Proficiency::Burned
  ]
}

#[derive(Debug)]
pub struct Card {
  pub line_number: Option<usize>,
  pub front: String,
  pub back: String,
  pub notes: String,
  pub level: i8,
  pub due_time: Option<DateTime<Utc>>,
  pub correct_count: u32,
  pub total_count: u32,
  phantom: ()
}

impl Card {
  pub fn new(front: String, back: String, notes: String) -> Card {
    Card {
      line_number: None,
      front,
      back,
      notes,
      level: 0,
      due_time: None,
      correct_count: 0,
      total_count: 0,
      phantom: ()
    }
  }

  pub fn proficiency(&self) -> Proficiency {
    match self.level {
      x if x <= 0 => Proficiency::Inactive,
      x if x <= 4 => Proficiency::Apprentice,
      x if x <= 6 => Proficiency::Guru,
      x if x <= 7 => Proficiency::Master,
      x if x <= 8 => Proficiency::Enlightened,
      _ => Proficiency::Burned
    }
  }

  pub fn correctness(&self) -> Option<f64> {
    if self.total_count == 0 {
      None
    } else {
      Some(self.correct_count as f64 / self.total_count as f64)
    }
  }

  pub fn critical(&self) -> bool {
    self.proficiency() == Proficiency::Apprentice
    && self.total_count > 0
    && self.correctness() < Some(0.75)
  }

  pub fn is_due(&self) -> bool {
    self.is_due_in(Utc::now())
  }

  pub fn is_due_in<T: TimeZone>(&self, date_time: DateTime<T>) -> bool {
    self.due_time.map(|x| x >= date_time).unwrap_or(false)
  }

  pub fn advance_level(&mut self) -> &mut Card {
    if self.proficiency() != Proficiency::Inactive {
      self.level = 9.min(self.level + 1);
    }

    self
  }

  pub fn fallback_level(&mut self) -> &mut Card {
    if self.proficiency() != Proficiency::Inactive {
      self.level = 1.max(self.level - 2);
    }

    self
  }

  pub fn learn(&mut self) -> &mut Card {
    if self.proficiency() == Proficiency::Inactive {
      self.level = 1;
      self.due_time = Some(Utc::now() + Duration::hours(4));
    }

    self
  }

  pub fn study(&mut self, remembered: bool) -> &mut Card {
    if self.proficiency() != Proficiency::Inactive {
      if remembered {
        self.advance_level();
        self.correct_count += 1;
      } else {
        self.fallback_level();
      }

      self.total_count += 1;
      self.due_time = Some(Utc::now() + match self.level {
        x if x <= 0 => panic!(),
        x if x == 1 => Duration::hours(4),
        x if x == 2 => Duration::hours(8),
        x if x == 3 => Duration::days(1),
        x if x == 4 => Duration::days(3),
        x if x == 5 => Duration::days(7),
        x if x == 6 => Duration::days(14),
        x if x == 7 => Duration::days(30),
        _ => Duration::days(122)
      });
    }

    self
  }
}

pub fn get_cards<I: IntoIterator<Item = List>>(lists: I) -> impl Iterator<Item = (String, Card)> {
  lists.into_iter()
  .filter_map(|list| list.cards().ok().map(|cards| (list, cards)))
  .map(|(list, cards)| {
    let name = list.name().to_string();
    cards.map(move|card| (name.clone(), card))
  })
  .flatten()
}
