mod cards;
mod constants;
mod date_manager;
mod ds;
mod enums;

use cards::{
    card::Card,
    card_algorithm::{CardAlgorithm, CardAlgorithmSM2},
    card_reader::CardReader,
    reviewer::Reviewer,
};

// use cards::card::Card;
// use ds::Queue;
// use enums::ReviewScore;

// use serde::{Deserialize, Serialize};
// use serde_json::{Result, Value};
// use chrono::prelude::*;
// use chrono::Days;
// use std::fs;
// use std::path::PathBuf;

fn main() {
    let today_cards: Vec<Card> = CardReader::get_today_cards();
    let algorithm = CardAlgorithmSM2::new();
    let mut reviewer: Reviewer<CardAlgorithmSM2> =
        Reviewer::<CardAlgorithmSM2>::from(today_cards, algorithm);
    reviewer.start().unwrap();
    CardReader::save_today_cards(reviewer.get_cards()).unwrap();
}
