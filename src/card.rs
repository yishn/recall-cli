use chrono::{offset::Utc, DateTime, Duration};

#[derive(Debug, Eq, PartialEq)]
pub enum Proficiency {
  Inactive,
  Apprentice,
  Guru,
  Master,
  Enlightened,
  Burned
}

pub struct Card {
  front: String,
  back: String,
  notes: String,
  level: i8,
  last_study_time: Option<DateTime<Utc>>,
  correct_count: u32,
  total_count: u32,
}

impl Card {
  pub fn new(front: String, back: String, notes: String) -> Card {
    Card {
      front,
      back,
      notes,
      level: 0,
      last_study_time: None,
      correct_count: 0,
      total_count: 0
    }
  }

  pub fn data(&self) -> (&str, &str, &str) {
    (&self.front, &self.back, &self.notes)
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

  pub fn last_study_time(&self) -> Option<DateTime<Utc>> {
    self.last_study_time
  }

  pub fn due_time(&self) -> Option<DateTime<Utc>> {
    let duration = match self.level {
      x if x <= 0 => return None,
      x if x == 1 => Duration::hours(4),
      x if x == 2 => Duration::hours(8),
      x if x == 3 => Duration::days(1),
      x if x == 4 => Duration::days(3),
      x if x == 5 => Duration::days(7),
      x if x == 6 => Duration::days(14),
      x if x == 7 => Duration::days(30),
      x if x == 8 => Duration::days(122),
      _ => Duration::days(122),
    };

    self.last_study_time.map(|x| x + duration)
  }

  pub fn correctness(&self) -> f64 {
    if self.total_count == 0 {
      0.0
    } else {
      self.correct_count as f64 / self.total_count as f64
    }
  }

  pub fn critical(&self) -> bool {
    self.proficiency() == Proficiency::Apprentice
    && self.total_count > 0
    && self.correctness() < 0.75
  }

  pub fn is_due(&self) -> bool {
    self.due_time().map(|x| x >= Utc::now()).unwrap_or(false)
  }

  pub fn set_front(&mut self, front: &str) -> &mut Card {
    self.front = front.to_string();
    self
  }

  pub fn set_back(&mut self, back: &str) -> &mut Card {
    self.back = back.to_string();
    self
  }

  pub fn set_notes(&mut self, notes: &str) -> &mut Card {
    self.notes = notes.to_string();
    self
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
      self.last_study_time = Some(Utc::now());
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
      self.last_study_time = Some(Utc::now());
    }
    self
  }
}
