use chrono::{offset::Utc, DateTime, Duration};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Proficiency {
  Inactive,
  Apprentice,
  Guru,
  Master,
  Enlightened,
  Burned
}

#[derive(Debug)]
pub struct Card {
  pub front: String,
  pub back: String,
  pub notes: String,
  pub level: i8,
  pub last_study_time: Option<DateTime<Utc>>,
  pub correct_count: u32,
  pub total_count: u32,
  phantom: ()
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
      _ => Duration::days(122),
    };

    self.last_study_time.map(|x| x + duration)
  }

  pub fn correctness(&self) -> f64 {
    if self.total_count == 0 {
      1.0
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
