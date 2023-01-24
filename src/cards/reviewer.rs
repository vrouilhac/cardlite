use colored::Color::TrueColor;
use colored::*;

use crate::cards::card::Card;
use crate::cards::card_algorithm::CardAlgorithm;
use crate::ds::Queue;

const GREY_COLOR: Color = TrueColor {
    r: 180,
    g: 180,
    b: 180,
};

const CHOICES_LIST: [&str; 6] = [
    "0 - Total blackout",
    "1 - Total blackout",
    "2 - Total blackout",
    "3 - Total blackout",
    "4 - Total blackout",
    "5 - Total blackout",
];

struct CardPrinter {}

impl CardPrinter {
    fn print_question(card: &Card) {
        println!("== Question =============================================");
        println!("{}", card.question());
        println!("=========================================================");
    }

    fn print_answer(card: &Card) {
        println!("== Answer ===============================================");
        println!("{}", card.answer());
        println!("=========================================================");
    }
}

struct IO {}

fn is_valid_choice(answer: &str) -> bool {
    let answer_integer = match answer.trim().parse::<u32>() {
        Ok(d) => d,
        Err(_) => 1000,
    };

    if answer_integer == 0
        || answer_integer == 1
        || answer_integer == 2
        || answer_integer == 3
        || answer_integer == 4
        || answer_integer == 5
    {
        return true;
    }

    false
}

impl IO {
    fn ask_press_to_continue() {
        println!("\n{}", "Press any key to continue".dimmed());
        std::io::stdin().read_line(&mut String::new()).unwrap();
    }

    fn ask_choice_from_list(choices_list: Vec<&str>) -> u32 {
        let mut answer = String::new();
        println!("\n{}", "How did you recall this answer ?".color(GREY_COLOR));

        for i in choices_list {
            println!("\t{}", i.color(GREY_COLOR));
        }

        loop {
            std::io::stdin().read_line(&mut answer).unwrap();

            if is_valid_choice(&answer) {
                break;
            }
        }

        answer.trim().parse::<u32>().unwrap()
    }

    fn clear_terminal() {
        // https://stackoverflow.com/questions/34837011/how-to-clear-the-terminal-screen-in-rust-after-a-new-line-is-printed
        // \x1B[2J clears the terminal
        // \x1B[1;1H places the cursor to first row
        print!("\x1B[2J\x1B[1;1H");
    }
}

#[allow(dead_code)]
pub struct Reviewer<T>
where
    T: CardAlgorithm,
{
    cards: Vec<Card>,
    algorithm: T,
}

impl<T> Reviewer<T>
where
    T: CardAlgorithm,
{
    #[allow(dead_code)]
    pub fn from(cards: Vec<Card>, algorithm: T) -> Reviewer<T> {
        Reviewer { cards, algorithm }
    }

    #[allow(dead_code)]
    pub fn get_cards(&self) -> Vec<Card> {
        // Here maybe a better understanding of lifetime would allow me to call get_cards and
        // tell the compiler to move cards because after it has been called
        // i do not want to update cards
        self.cards.clone()
    }

    #[allow(dead_code)]
    pub fn start(&mut self) -> Result<(), String> {
        let mut review_queue = Queue::from(self.cards.clone());
        let mut updated_cards: Vec<Card> = vec![];

        loop {
            if let Some(card) = review_queue.next() {
                IO::clear_terminal();
                CardPrinter::print_question(&card);
                IO::ask_press_to_continue();
                CardPrinter::print_answer(&card);
                let choice = IO::ask_choice_from_list(CHOICES_LIST.to_vec());

                if choice < 4 {
                    review_queue.append(card);
                } else {
                    let mut updated_card = card.clone();
                    let (next_n, next_i, next_ef) = self.algorithm.compute_score(
                        choice,
                        updated_card.success_recall_time(),
                        updated_card.interval(),
                        updated_card.easiness_factor(),
                    );

                    updated_card.update_score(next_ef, next_n, next_i);
                    updated_cards.push(updated_card);
                }
            } else {
                break;
            }
        }

        self.cards = updated_cards;

        // print the question
        // ask to enter a key to show answer
        // ask recall score
        // apply algorithm
        // save data or push to queue again
        // next card
        Ok(())
    }
}
