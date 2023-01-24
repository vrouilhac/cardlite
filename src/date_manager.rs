use chrono::{DateTime, Datelike, Duration, Utc};

#[derive(Debug)]
pub struct DateManager {
    date: String,
}

impl DateManager {
    pub fn date(&self) -> String {
        self.date.clone()
    }

    pub fn new() -> DateManager {
        let now: DateTime<Utc> = DateTime::from(Utc::now());
        let timestamp = now.to_rfc3339();

        DateManager { date: timestamp }
    }

    pub fn from(date: String) -> DateManager {
        DateManager { date }
    }

    pub fn add(&mut self, days: i64) {
        let date = DateTime::parse_from_rfc3339(&self.date).unwrap();
        let next_date = date + Duration::days(days);
        self.date = next_date.to_rfc3339();
    }

    pub fn is_date(&self, date: DateManager) -> bool {
        // todo and replace is_today in get_today_cards
        true
    }

    pub fn is_today(&self) -> bool {
        let date = DateTime::parse_from_rfc3339(&self.date).unwrap();
        let today: DateTime<Utc> = DateTime::from(Utc::now());

        if date.year() <= today.year() && date.month() <= today.month() && date.day() <= today.day()
        {
            return true;
        }

        false
    }
}
