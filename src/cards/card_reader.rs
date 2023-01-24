use std::fs;
use std::path::PathBuf;

use crate::cards::card::Card;
use crate::date_manager::DateManager;

enum CardReaderError {
    ConfigRead,
}

#[allow(dead_code)]
pub struct CardReader {}

impl CardReader {
    #[allow(dead_code)]
    pub fn get_today_cards() -> Vec<Card> {
        let today_date = DateManager::new();
        let cards = match read_cards_config() {
            Ok(cards) => cards,
            Err(_) => panic!("Failed to read config"),
        };

        // See function message
        let today_cards = filter_cards_by_date(&cards, today_date);

        today_cards
    }

    #[allow(dead_code)]
    pub fn save_today_cards(today_cards: Vec<Card>) -> Result<(), String> {
        let mut cards = match read_cards_config() {
            Ok(cards) => cards,
            Err(_) => panic!("Failed to save cards"),
        };

        for card in today_cards {
            for i in 0..cards.len() {
                if cards[i].question() == card.question() {
                    cards.remove(i);
                    cards.push(card.clone());
                }
            }
        }

        let path = PathBuf::from("./config.json");
        let config_stringified = serde_json::to_string_pretty(&cards).unwrap();
        match fs::write(path, config_stringified) {
            Ok(_) => Ok(()),
            Err(_) => Err(String::from("Failed to save Cards")),
        }
    }
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

// Currently this function filters by today date only
fn filter_cards_by_date(cards: &Vec<Card>, _date: DateManager) -> Vec<Card> {
    let mut filtered_cards: Vec<Card> = vec![];

    for card in cards {
        let card_date = DateManager::from(card.next_review_date());

        if card_date.is_today() {
            filtered_cards.push(card.clone());
        }
    }

    filtered_cards
}
