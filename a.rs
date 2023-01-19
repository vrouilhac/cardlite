use serde::{Deserialize, Serialize};
// use serde_json::{Result, Value};
use chrono::prelude::*;
use chrono::{Days, Duration};
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
struct Queue<T> {
    array: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Queue<T> {
        Queue::<T> { array: vec![] }
    }

    fn append(&mut self, element: T) {
        let mut new_array: Vec<T> = vec![element];
        new_array.append(&mut self.array);
        self.array = new_array;
    }

    fn next(&mut self) -> Option<T> {
        let next = self.array.pop();
        next
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CardsJson {
    cards: Vec<CardMetadata>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CardMetadata {
    id: String,
    path: String,
    i: i32,
    review_day: String,
    ef: f32,
    n: i32,
}

struct Config {
    path: String,
}

impl Config {
    fn get_cards_config(&self) -> CardsJson {
        let path = PathBuf::from(&self.path);
        let config_file_content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => panic!("Failed to read config file {:?}", e),
        };
        let config_parsed = serde_json::from_str(&config_file_content);

        let config = match config_parsed {
            Ok(result) => result,
            Err(e) => panic!("Failed to parse the json file {}", e),
        };

        config
    }
}

fn main() {
    let path = String::from("./config.json");
    let config = Config { path };
    let cards_config = config.get_cards_config();
    let (mut today_cards, mut other_cards) = get_today_cards(cards_config);
    match review_cards(&mut today_cards) {
        Ok(_e) => (),
        Err(_e) => panic!("Error"),
    };
    today_cards.append(&mut other_cards);
    save(&CardsJson { cards: today_cards }, config.path);
}

fn save(cards_config: &CardsJson, path: String) {
    let json = match serde_json::to_string_pretty(cards_config) {
        Ok(j) => j,
        Err(e) => panic!("{}", e),
    };
    match fs::write(path, json) {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    };
}

fn print_options() {
    println!("How well did you remembered ?");
    println!("0 - Total blackout");
    println!("1 - Incorrect but felt familiar");
    println!("2 - Incorrect but seemed easy to remember afterward");
    println!("3 - Correct answer with significant effort");
    println!("4 - Correce answer after hesitation");
    println!("5 - Correct answer with perfect recall");
}

fn ask_for_answer(answer: &str) {
    println!("Enter any key to show answer");
    get_answer();
    println!("---");
    println!("{}", answer);
    println!("---");
}

fn get_answer() -> String {
    let mut a = String::new();
    let _answer = std::io::stdin().read_line(&mut a).unwrap();
    a
}

fn is_valid_answer(answer: &String) -> bool {
    println!("{}", answer);
    if answer.contains("0")
        || answer.contains("1")
        || answer.contains("2")
        || answer.contains("3")
        || answer.contains("4")
        || answer.contains("5")
    {
        return true;
    }

    false
}

fn review_cards(cards: &mut Vec<CardMetadata>) -> std::io::Result<()> {
    // add a queue
    for mut i in cards {
        let card_content = match fs::read_to_string(&i.path) {
            Ok(content) => content,
            Err(e) => panic!("Failed to read card content {}", e),
        };
        let card_content = card_content.split("---").collect::<Vec<&str>>();
        println!("---");
        println!("{}", card_content[0]);
        println!("---");
        ask_for_answer(&card_content[1]);
        print_options();
        let mut answer = get_answer();

        while !is_valid_answer(&answer) {
            println!("Select a valid option");
            answer = get_answer();
        }

        compute_recall_metadata(&mut i, answer);
    }

    Ok(())
}

fn compute_recall_metadata(card: &mut CardMetadata, answer: String) {
    let parsed_answer = match answer.trim().parse::<i8>() {
        Ok(n) => n,
        Err(e) => panic!("{}", e),
    };
    let mut days_to_add: i32 = 0;

    if parsed_answer >= 3 {
        if card.n == 0 {
            days_to_add = 1;
        } else if card.n == 1 {
            days_to_add = 6;
        } else {
            days_to_add = (card.i as f32 * card.ef).round() as i32;
        }

        card.n = card.n + 1;
    } else {
        card.n = 0;
        card.i = 1;
    }

    card.ef =
        card.ef + (0.1 - (5 - parsed_answer) as f32 * (0.08 + (5 - parsed_answer) as f32 * 0.02));
    card.review_day = compute_next_review_day(&card.review_day, days_to_add);

    if card.ef < 1.3 {
        card.ef = 1.3;
    }
}

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

fn get_today_cards(cards_config: CardsJson) -> (Vec<CardMetadata>, Vec<CardMetadata>) {
    let mut today_cards: Vec<CardMetadata> = vec![];
    let mut other_cards: Vec<CardMetadata> = vec![];

    for i in cards_config.cards {
        if need_review_card(&i) {
            today_cards.push(i);
        } else {
            other_cards.push(i);
        }
    }

    (today_cards, other_cards)
}

fn need_review_card(card: &CardMetadata) -> bool {
    let today_date = Local::now();
    let card_date = card.review_day.split("-").collect::<Vec<&str>>();
    let same_year = today_date.year().to_string() == card_date[0];
    let today_month = format_date(today_date.month());
    let same_month = today_month == card_date[1];
    let today_day = format_date(today_date.day());
    let same_day = today_day == card_date[2];

    if same_year && same_month && same_day {
        return true;
    }

    false
}

fn format_date(number: u32) -> String {
    if number < 10 {
        return format!("0{}", number.to_string());
    }

    number.to_string()
}
