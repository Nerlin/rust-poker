use std::thread::sleep;
use std::time::Duration;

use crate::poker::card::Hand;
use poker::card::Card;
use poker::deck::Deck;
use poker::rules::get_rules;

mod poker;

fn main() {
    loop {
        let mut deck = Deck::create();
        deck.shuffle();

        let mut hand: Hand = [Card::new(); 5];

        for i in 0..5 {
            match deck.deal() {
                Some(card) => hand[i] = card,
                None => panic!("Not enough cards to deal."),
            }
        }

        for card in &hand {
            print!("{} ", card);
        }
        println!("");

        let rules = get_rules();
        for rule in rules {
            match rule(&hand) {
                Some(combination) => {
                    println!("{}", combination);
                    break;
                }
                None => {
                    continue;
                }
            }
        }

        println!("");
        sleep(Duration::from_secs(1))
    }
}

#[cfg(test)]
mod tests {
    use crate::poker::card::Card;
    use crate::poker::card::Hand;
    use crate::poker::card::Quantity;
    use crate::poker::card::Suit;
    use crate::poker::rules::check_duplicates;
    use crate::poker::rules::check_flush;
    use crate::poker::rules::check_flush_royal;
    use crate::poker::rules::check_straight;
    use crate::poker::rules::Combination;

    #[test]
    fn test_straight() {
        let hand: Hand = [
            Card {
                quantity: Quantity::Jack,
                suit: Suit::Diamonds,
            },
            Card {
                quantity: Quantity::Eight,
                suit: Suit::Clubs,
            },
            Card {
                quantity: Quantity::Ten,
                suit: Suit::Diamonds,
            },
            Card {
                quantity: Quantity::Seven,
                suit: Suit::Diamonds,
            },
            Card {
                quantity: Quantity::Nine,
                suit: Suit::Hearts,
            },
        ];

        match check_straight(&hand) {
            Some(combination) => assert_eq!(
                combination,
                Combination {
                    name: "Straight",
                    cards: hand.iter().collect()
                }
            ),
            None => panic!("Did not receive straight."),
        }
    }

    #[test]
    fn test_straight_from_ace_to_five() {
        let hand: Hand = [
            Card {
                quantity: Quantity::Ace,
                suit: Suit::Hearts,
            },
            Card {
                quantity: Quantity::Two,
                suit: Suit::Spades,
            },
            Card {
                quantity: Quantity::Three,
                suit: Suit::Hearts,
            },
            Card {
                quantity: Quantity::Four,
                suit: Suit::Spades,
            },
            Card {
                quantity: Quantity::Five,
                suit: Suit::Spades,
            },
        ];

        match check_straight(&hand) {
            Some(combination) => assert_eq!(
                combination,
                Combination {
                    name: "Straight",
                    cards: hand.iter().collect()
                }
            ),
            None => panic!("Did not receive straight."),
        }
    }

    #[test]
    fn test_straight_from_ten_to_ace() {
        let hand: Hand = [
            Card {
                quantity: Quantity::Ten,
                suit: Suit::Spades,
            },
            Card {
                quantity: Quantity::Jack,
                suit: Suit::Diamonds,
            },
            Card {
                quantity: Quantity::Queen,
                suit: Suit::Diamonds,
            },
            Card {
                quantity: Quantity::King,
                suit: Suit::Diamonds,
            },
            Card {
                quantity: Quantity::Ace,
                suit: Suit::Hearts,
            },
        ];

        match check_straight(&hand) {
            Some(combination) => assert_eq!(
                combination,
                Combination {
                    name: "Straight",
                    cards: hand.iter().collect()
                }
            ),
            None => panic!("Did not receive straight."),
        }
    }

    #[test]
    fn test_straight_fail() {
        let hand: Hand = [
            Card {
                quantity: Quantity::Five,
                suit: Suit::Hearts,
            },
            Card {
                quantity: Quantity::Seven,
                suit: Suit::Hearts,
            },
            Card {
                quantity: Quantity::Eight,
                suit: Suit::Hearts,
            },
            Card {
                quantity: Quantity::Nine,
                suit: Suit::Hearts,
            },
            Card {
                quantity: Quantity::Ten,
                suit: Suit::Hearts,
            },
        ];

        match check_straight(&hand) {
            Some(_) => panic!("Did not expect straight."),
            None => {}
        }
    }

    #[test]
    fn test_flush() {
        let hand: Hand = [
            Card {
                quantity: Quantity::Ace,
                suit: Suit::Spades,
            },
            Card {
                quantity: Quantity::Three,
                suit: Suit::Spades,
            },
            Card {
                quantity: Quantity::Eight,
                suit: Suit::Spades,
            },
            Card {
                quantity: Quantity::Seven,
                suit: Suit::Spades,
            },
            Card {
                quantity: Quantity::Queen,
                suit: Suit::Spades,
            },
        ];

        match check_flush(&hand) {
            Some(combination) => assert_eq!(
                combination,
                Combination {
                    name: "Flush",
                    cards: hand.iter().collect()
                }
            ),
            None => panic!("Did not receive flush."),
        }
    }

    #[test]
    fn test_flush_fail() {
        let hand: Hand = [
            Card {
                quantity: Quantity::Ace,
                suit: Suit::Spades,
            },
            Card {
                quantity: Quantity::Three,
                suit: Suit::Hearts,
            },
            Card {
                quantity: Quantity::Eight,
                suit: Suit::Spades,
            },
            Card {
                quantity: Quantity::Seven,
                suit: Suit::Spades,
            },
            Card {
                quantity: Quantity::Queen,
                suit: Suit::Spades,
            },
        ];

        match check_flush(&hand) {
            Some(_) => panic!("Did not expect flush."),
            None => {}
        }
    }

    #[test]
    fn test_flush_royal() {
        let hand: Hand = [
            Card {
                quantity: Quantity::Ten,
                suit: Suit::Spades,
            },
            Card {
                quantity: Quantity::Jack,
                suit: Suit::Spades,
            },
            Card {
                quantity: Quantity::Queen,
                suit: Suit::Spades,
            },
            Card {
                quantity: Quantity::King,
                suit: Suit::Spades,
            },
            Card {
                quantity: Quantity::Ace,
                suit: Suit::Spades,
            },
        ];

        match check_flush_royal(&hand) {
            Some(combination) => assert_eq!(
                combination,
                Combination {
                    name: "Flush Royal",
                    cards: hand.iter().collect()
                }
            ),
            None => panic!("Did not receive flush royal."),
        }
    }

    #[test]
    fn test_flush_royal_returns_flush() {
        let hand: Hand = [
            Card {
                quantity: Quantity::Nine,
                suit: Suit::Spades,
            },
            Card {
                quantity: Quantity::Jack,
                suit: Suit::Spades,
            },
            Card {
                quantity: Quantity::Queen,
                suit: Suit::Spades,
            },
            Card {
                quantity: Quantity::King,
                suit: Suit::Spades,
            },
            Card {
                quantity: Quantity::Ace,
                suit: Suit::Spades,
            },
        ];

        match check_flush_royal(&hand) {
            Some(combination) => assert_eq!(
                combination,
                Combination {
                    name: "Flush",
                    cards: hand.iter().collect()
                }
            ),
            None => panic!("Did not receive flush."),
        }
    }

    #[test]
    fn test_flush_royal_returns_straight() {
        let hand: Hand = [
            Card {
                quantity: Quantity::Ten,
                suit: Suit::Spades,
            },
            Card {
                quantity: Quantity::Jack,
                suit: Suit::Spades,
            },
            Card {
                quantity: Quantity::Queen,
                suit: Suit::Spades,
            },
            Card {
                quantity: Quantity::King,
                suit: Suit::Hearts,
            },
            Card {
                quantity: Quantity::Ace,
                suit: Suit::Spades,
            },
        ];

        match check_flush_royal(&hand) {
            Some(combination) => assert_eq!(
                combination,
                Combination {
                    name: "Straight",
                    cards: hand.iter().collect()
                }
            ),
            None => panic!("Did not receive straight."),
        }
    }

    #[test]
    fn test_flush_royal_fail() {
        let hand: Hand = [
            Card {
                quantity: Quantity::Ace,
                suit: Suit::Diamonds,
            },
            Card {
                quantity: Quantity::Jack,
                suit: Suit::Spades,
            },
            Card {
                quantity: Quantity::Queen,
                suit: Suit::Spades,
            },
            Card {
                quantity: Quantity::King,
                suit: Suit::Hearts,
            },
            Card {
                quantity: Quantity::Ace,
                suit: Suit::Spades,
            },
        ];

        match check_flush_royal(&hand) {
            Some(combination) => panic!("Did not expect a combination. {}", combination),
            None => {}
        }
    }

    #[test]
    fn test_pair() {
        let hand: Hand = [
            Card {
                quantity: Quantity::Ace,
                suit: Suit::Diamonds,
            },
            Card {
                quantity: Quantity::Jack,
                suit: Suit::Spades,
            },
            Card {
                quantity: Quantity::Queen,
                suit: Suit::Spades,
            },
            Card {
                quantity: Quantity::King,
                suit: Suit::Hearts,
            },
            Card {
                quantity: Quantity::Ace,
                suit: Suit::Spades,
            },
        ];

        match check_duplicates(&hand) {
            Some(combination) => assert_eq!(
                combination,
                Combination {
                    name: "Pair",
                    cards: vec![
                        &Card {
                            quantity: Quantity::Ace,
                            suit: Suit::Diamonds,
                        },
                        &Card {
                            quantity: Quantity::Ace,
                            suit: Suit::Spades,
                        }
                    ]
                }
            ),
            None => panic!("Did not receive a pair."),
        }
    }

    #[test]
    fn test_two_pairs() {
        let hand: Hand = [
            Card {
                quantity: Quantity::Two,
                suit: Suit::Hearts,
            },
            Card {
                quantity: Quantity::Seven,
                suit: Suit::Hearts,
            },
            Card {
                quantity: Quantity::Seven,
                suit: Suit::Spades,
            },
            Card {
                quantity: Quantity::Ace,
                suit: Suit::Diamonds,
            },
            Card {
                quantity: Quantity::Two,
                suit: Suit::Spades,
            },
        ];

        match check_duplicates(&hand) {
            Some(combination) => assert_eq!(
                combination,
                Combination {
                    name: "Two pairs",
                    cards: vec![
                        &Card {
                            quantity: Quantity::Seven,
                            suit: Suit::Hearts,
                        },
                        &Card {
                            quantity: Quantity::Seven,
                            suit: Suit::Spades,
                        },
                        &Card {
                            quantity: Quantity::Two,
                            suit: Suit::Hearts,
                        },
                        &Card {
                            quantity: Quantity::Two,
                            suit: Suit::Spades,
                        },
                    ]
                }
            ),
            None => panic!("Did not receive a pair."),
        }
    }

    #[test]
    fn test_three_of_a_kind() {
        let hand: Hand = [
            Card {
                quantity: Quantity::Six,
                suit: Suit::Diamonds,
            },
            Card {
                quantity: Quantity::Jack,
                suit: Suit::Clubs,
            },
            Card {
                quantity: Quantity::Six,
                suit: Suit::Spades,
            },
            Card {
                quantity: Quantity::Six,
                suit: Suit::Clubs,
            },
            Card {
                quantity: Quantity::Ace,
                suit: Suit::Spades,
            },
        ];

        match check_duplicates(&hand) {
            Some(combination) => assert_eq!(
                combination,
                Combination {
                    name: "Three of a kind",
                    cards: vec![
                        &Card {
                            quantity: Quantity::Six,
                            suit: Suit::Diamonds,
                        },
                        &Card {
                            quantity: Quantity::Six,
                            suit: Suit::Spades,
                        },
                        &Card {
                            quantity: Quantity::Six,
                            suit: Suit::Clubs,
                        }
                    ]
                }
            ),
            None => panic!("Did not receive a pair."),
        }
    }

    #[test]
    fn test_four_of_a_kind() {
        let hand: Hand = [
            Card {
                quantity: Quantity::Four,
                suit: Suit::Diamonds,
            },
            Card {
                quantity: Quantity::Four,
                suit: Suit::Clubs,
            },
            Card {
                quantity: Quantity::Six,
                suit: Suit::Spades,
            },
            Card {
                quantity: Quantity::Four,
                suit: Suit::Hearts,
            },
            Card {
                quantity: Quantity::Four,
                suit: Suit::Spades,
            },
        ];

        match check_duplicates(&hand) {
            Some(combination) => assert_eq!(
                combination,
                Combination {
                    name: "Four of a kind",
                    cards: vec![
                        &Card {
                            quantity: Quantity::Four,
                            suit: Suit::Diamonds,
                        },
                        &Card {
                            quantity: Quantity::Four,
                            suit: Suit::Clubs,
                        },
                        &Card {
                            quantity: Quantity::Four,
                            suit: Suit::Hearts,
                        },
                        &Card {
                            quantity: Quantity::Four,
                            suit: Suit::Spades,
                        }
                    ]
                }
            ),
            None => panic!("Did not receive a pair."),
        }
    }

    #[test]
    fn test_no_duplicates() {
        let hand: Hand = [
            Card {
                quantity: Quantity::Ace,
                suit: Suit::Hearts,
            },
            Card {
                quantity: Quantity::Five,
                suit: Suit::Spades,
            },
            Card {
                quantity: Quantity::Seven,
                suit: Suit::Clubs,
            },
            Card {
                quantity: Quantity::Four,
                suit: Suit::Clubs,
            },
            Card {
                quantity: Quantity::Ten,
                suit: Suit::Spades,
            },
        ];

        match check_duplicates(&hand) {
            Some(combination) => panic!("Did not expect a combination: {}", combination),
            None => {}
        }
    }
}
