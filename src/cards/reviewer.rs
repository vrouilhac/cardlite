use crate::cards::card::Card;
use crate::cards::card_algorithm::CardAlgorithm;

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
    pub fn start(&self) -> Result<(), String> {
        Ok(())
    }
}
