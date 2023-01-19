mod cards;

use cards::card::Card;

use serde::{Deserialize, Serialize};
// use serde_json::{Result, Value};
use chrono::prelude::*;
use chrono::{Days, Duration};
use std::fs;
use std::path::PathBuf;

const DEFAULT_EF: f32 = 2.5;
const DEFAULT_N: u32 = 0;
const DEFAULT_I: u32 = 0;

#[derive(Debug)]
struct Queue<T> {
    array: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Queue<T> {
        Queue::<T> { array: vec![] }
    }

    fn from(elements: Vec<T>) -> Queue<T> {
        Queue { array: elements }
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

    fn is_empty(&self) -> bool {
        if self.array.len() == 0 {
            true
        } else {
            false
        }
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

enum ReviewScore {
    ZERO,
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
}

impl ReviewScore {
    fn get_value(&self) -> u32 {
        match &self {
            ReviewScore::ZERO => 0,
            ReviewScore::ONE => 1,
            ReviewScore::TWO => 2,
            ReviewScore::THREE => 3,
            ReviewScore::FOUR => 4,
            ReviewScore::FIVE => 5,
        }
    }
}

// remove
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Card {
    #[serde(rename = "i")]
    interval: u32,

    #[serde(rename = "n")]
    success_recall_time: u32,

    #[serde(rename = "ef")]
    easiness_factor: f32,

    #[serde(rename = "p")]
    path: String,

    #[serde(rename = "r")]
    review_day: String,
}

impl Card {
    fn new() -> Card {
        Card {
            interval: DEFAULT_I,
            success_recall_time: DEFAULT_N,
            path: String::from("./there"),
            easiness_factor: DEFAULT_EF,
            review_day: String::from("2023-01-16"),
        }
    }

    fn compute_review_score(&mut self, score: &ReviewScore) {
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
        self.review_day = compute_next_review_day(&self.review_day, days_to_add);

        if self.easiness_factor < 1.3 {
            self.easiness_factor = 1.3;
        }
    }
}

#[derive(Debug)]
struct Reviewer {
    cards: Vec<Card>,
    review_cards: Queue<Card>,
}

struct CardReader {
    question: String,
    answer: String,
}

fn clear_terminal() {
    print!("\x1B[2J");
}

impl CardReader {
    fn from(path: String) -> Result<CardReader, String> {
        let path = PathBuf::from(path);
        let card_content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(_) => panic!("There was an error reading card content"),
        };
        let card_split = card_content.split("---").collect::<Vec<&str>>();
        Ok(CardReader {
            question: String::from(card_split[0]),
            answer: String::from(card_split[1]),
        })
    }

    fn ask_for_answer(&self) -> String {
        let mut a = String::new();
        let _answer = std::io::stdin().read_line(&mut a).unwrap();
        a
    }

    fn show_question(&self) {
        // clear_terminal();
        println!("-- Question ---");
        println!("{}", self.question.trim());
        println!("----");
    }

    fn show_answer(&self) {
        println!("-- Answer ---");
        println!("{}", self.answer.trim());
        println!("----");
    }

    fn ask_recall_score(&self) -> ReviewScore {
        print_options();
        let mut valid = false;
        let mut final_answer: ReviewScore = ReviewScore::ZERO;

        while !valid {
            let answer = self.ask_for_answer();
            let number_answer = match answer.trim().parse::<u32>() {
                Ok(r) => r,
                Err(_) => continue,
            };

            let value = match number_answer {
                0 => Some(ReviewScore::ZERO),
                1 => Some(ReviewScore::ONE),
                2 => Some(ReviewScore::TWO),
                3 => Some(ReviewScore::THREE),
                4 => Some(ReviewScore::FOUR),
                5 => Some(ReviewScore::FIVE),
                _ => None,
            };

            match value {
                Some(x) => {
                    valid = true;
                    final_answer = x;
                }
                None => (),
            }
        }

        final_answer
    }
}

impl Reviewer {
    fn start(&mut self) {
        println!("{:?}", self.review_cards);
        while !self.review_cards.is_empty() {
            let mut next_card = self.review_cards.next().unwrap();
            let card_content = match CardReader::from(next_card.path.clone()) {
                Ok(card_content) => card_content,
                Err(_) => panic!("Can't read card content"),
            };
            card_content.show_question();
            card_content.ask_for_answer();
            card_content.show_answer();
            let recall_answer = card_content.ask_recall_score();
            println!("{:?}", next_card);
            next_card.compute_review_score(&recall_answer);
            println!("{:?}", next_card);

            if recall_answer.get_value() < 4 {
                self.review_cards.append(next_card);
            }
            // read the card file
            // show the card quesion
            // ask for whatever input
            // show the card answer
            // ask for recall score
            // compute card next data and remove/keep it in queue
        }
    }

    fn from(cards: Vec<Card>) -> Reviewer {
        Reviewer {
            cards: vec![],
            review_cards: Queue::from(cards),
        }
    }
}

fn main() {
    let card1 = Card::new();
    let mut reviewer = Reviewer::from(vec![card1]);
    reviewer.start();

    // Read available card
    // Select todays cards and add them to the queue
    // Go through the queue until it's empty
    // Compute the score for each cards
    // Save the new metadata
    // Exit

    // let path = String::from("./config.json");
    // let config = Config { path };
    // let cards_config = config.get_cards_config();
    // let (mut today_cards, mut other_cards) = get_today_cards(cards_config);
    // match review_cards(&mut today_cards) {
    //     Ok(_e) => (),
    //     Err(_e) => panic!("Error"),
    // };
    // today_cards.append(&mut other_cards);
    // save(&CardsJson { cards: today_cards }, config.path);
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
