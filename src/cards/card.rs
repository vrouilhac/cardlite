use crate::constants::{DEFAULT_EF, DEFAULT_I, DEFAULT_N};
use crate::enums::ReviewScore;

use serde::{Deserialize, Serialize};

use chrono::prelude::*;
use chrono::Days;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Card {
    #[serde(rename = "i")]
    interval: u32,

    #[serde(rename = "n")]
    success_recall_time: u32,

    #[serde(rename = "ef")]
    easiness_factor: f32,

    #[serde(rename = "p")]
    path: String,

    #[serde(rename = "r")]
    next_review_day: String,

    #[serde(rename = "q")]
    question: String,

    #[serde(rename = "a")]
    answer: String,
}

impl Card {
    pub fn new() -> Card {
        Card {
            interval: DEFAULT_I,
            success_recall_time: DEFAULT_N,
            path: String::from("./there"),
            easiness_factor: DEFAULT_EF,
            next_review_day: String::from("2023-01-16"),
            question: String::from("Question"),
            answer: String::from("Answer"),
        }
    }

    // not get see (https://rust-lang.github.io/api-guidelines/naming.html#c-getter)
    pub fn next_review_date(&self) -> String {
        self.next_review_day.clone()
    }

    pub fn compute_review_score(&mut self, score: &ReviewScore) {
        let answer = score.get_value();
        let mut days_to_add: i32 = 0;

        if answer >= 3 {
            days_to_add = match self.success_recall_time {
                0 => 1,
                1 => 6,
                _ => (self.interval as f32 * self.easiness_factor).round() as i32,
            };
            self.interval = days_to_add as u32;
            self.success_recall_time = self.success_recall_time + 1;
        } else {
            self.success_recall_time = 0;
            self.interval = 1;
        }

        self.easiness_factor = self.easiness_factor
            + (0.1 - (5 - answer) as f32 * (0.08 + (5 - answer) as f32 * 0.02));
        self.next_review_day = compute_next_review_day(&self.next_review_day, days_to_add);

        if self.easiness_factor < 1.3 {
            self.easiness_factor = 1.3;
        }
    }
}

// Optimize this
fn compute_next_review_day(review_day: &String, days_to_add: i32) -> String {
    let re = review_day.split("-").collect::<Vec<&str>>();
    let year = match re[0].parse::<i32>() {
        Ok(r) => r,
        Err(_) => panic!("Failed to parse number"),
    };
    let month = match re[1].parse::<u32>() {
        Ok(r) => r,
        Err(_) => panic!("Failed to parse number"),
    };
    let day = match re[2].parse::<u32>() {
        Ok(r) => r,
        Err(_) => panic!("Failed to parse number"),
    };
    let date = match NaiveDate::from_ymd_opt(year, month, day) {
        Some(x) => x,
        None => panic!("Problem formatting date"),
    };
    let new_date = match date.checked_add_days(Days::new(days_to_add as u64)) {
        Some(r) => r,
        None => panic!("Failed to add date"),
    };

    new_date.to_string()
}
