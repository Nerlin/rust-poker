use poker::card::Card;
use poker::deck::Deck;

mod poker;

fn main() {
    let mut deck = Deck::create();
    deck.shuffle();

    let mut hand: [Card; 5] = [Card::new(); 5];

    for i in 0..5 {
        match deck.deal() {
            Some(card) => hand[i] = card,
            None => panic!("Not enough cards to deal.")
        }
    }

    for card in &hand {
        println!("{}", card);
    }
}