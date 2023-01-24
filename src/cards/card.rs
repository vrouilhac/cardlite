use crate::constants::{DEFAULT_EF, DEFAULT_I, DEFAULT_N};
use crate::date_manager::DateManager;

use serde::{Deserialize, Serialize};

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

    pub fn interval(&self) -> u32 {
        self.interval
    }

    pub fn success_recall_time(&self) -> u32 {
        self.success_recall_time
    }

    pub fn question(&self) -> String {
        self.question.clone()
    }

    pub fn easiness_factor(&self) -> f32 {
        self.easiness_factor
    }

    pub fn update_score(&mut self, next_ef: f32, next_n: u32, next_i: u32) {
        self.interval = next_i;
        self.easiness_factor = next_ef;
        self.success_recall_time = next_n;
        let next_review_date = update_next_review_date(next_i);
        self.next_review_day = next_review_date;
    }

    pub fn answer(&self) -> String {
        self.answer.clone()
    }
}

fn update_next_review_date(interval: u32) -> String {
    let mut today = DateManager::new();
    today.add(interval.into());
    today.date()
}
