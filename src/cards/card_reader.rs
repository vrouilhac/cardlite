use std::fs;
use std::path::PathBuf;

use crate::cards::card::Card;
use chrono::{DateTime, Utc};

enum CardReaderError {
    ConfigRead,
}

#[allow(dead_code)]
pub struct CardReader {}

impl CardReader {
    #[allow(dead_code)]
    pub fn get_today_cards() -> Vec<Card> {
        let today_date = get_today_date();
        let cards = match read_cards_config() {
            Ok(cards) => cards,
            Err(_) => panic!("Failed to read config"),
        };
        let today_cards = filter_cards_by_date(&cards, today_date);
        // filter today card by review date
        // return today cards
        vec![]
    }

    #[allow(dead_code)]
    pub fn save_today_cards(_cards: Vec<Card>) -> Result<(), String> {
        Ok(())
    }
}

fn get_today_date() -> DateTime<Utc> {
    let today = Utc::now();
    today
}

fn read_cards_config() -> Result<Vec<Card>, CardReaderError> {
    let path = PathBuf::from("./config.json");
    let file_content = fs::read_to_string(path).unwrap(); // Opti
    let config_parsed = serde_json::from_str::<Vec<Card>>(&file_content);

    match config_parsed {
        Ok(cards) => Ok(cards),
        Err(_) => Err(CardReaderError::ConfigRead),
    }
}

fn filter_cards_by_date(cards: &Vec<Card>, date: DateTime<Utc>) -> Vec<Card> {
    let filtered_cards: Vec<Card> = vec![];

    for card in cards {
        // let card_date = Utc::from(card.next_review_date());
        // get card date
        // compare dates and append to filtered_cards accordingly
        // filtered_cards.append(card);
    }

    filtered_cards
}
