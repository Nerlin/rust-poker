use crate::poker::card::{Card, Suit, Quantity};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Deck {
    pub cards: Vec<Card>,
}


impl Deck {
    pub fn create() -> Deck {
        let mut cards: Vec<Card> = Vec::with_capacity(52);

        for suit in Suit::all() {
            for quantity in Quantity::all() {
                cards.push(Card {
                    suit: *suit,
                    quantity: *quantity
                })
            }
        }

        Deck {
            cards
        }
    }

    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
}
