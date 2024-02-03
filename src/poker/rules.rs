use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Display,
};

use super::card::{Hand, Quantity};
use crate::Card;

pub type Rule = fn(&Hand) -> Option<Combination>;

pub fn check_flush_royal(hand: &Hand) -> Option<Combination> {
    let straight = check_straight(hand);
    let flush = check_flush(hand);

    match (straight, flush) {
        (Some(_), Some(_)) => {
            let mut sorted: Hand = hand.clone();
            sorted.sort();

            let mut start = 10;
            for card in sorted {
                if card.quantity as u8 != start {
                    return Some(Combination {
                        name: "Straight Flush",
                        cards: hand.iter().collect(),
                    });
                }
                start += 1;
            }
            return Some(Combination {
                name: "Flush Royal",
                cards: hand.iter().collect(),
            });
        }
        (Some(straight), None) => Some(straight),
        (None, Some(flush)) => Some(flush),
        (None, None) => None,
    }
}

pub fn check_straight(hand: &Hand) -> Option<Combination> {
    let mut sorted: Hand = hand.clone();
    sorted.sort();

    // This handles an edge case when ace can be used
    // to make a straight combination from ace to five:
    // ace is always considered as the highest card,
    // therefore this only needs to check the first four cards in the hand.
    let len = if sorted[0].quantity == Quantity::Two && sorted[4].quantity == Quantity::Ace {
        4
    } else {
        5
    };

    for i in 1..len {
        let q1 = sorted[i].quantity as u8;
        let q2 = sorted[i - 1].quantity as u8;
        if q1 != q2 + 1 {
            return None;
        }
    }

    return Some(Combination {
        name: "Straight",
        cards: hand.iter().collect(),
    });
}

pub fn check_flush(hand: &Hand) -> Option<Combination> {
    let suit = hand[0].suit;
    for card in hand {
        if card.suit != suit {
            return None;
        }
    }
    Some(Combination {
        name: "Flush",
        cards: hand.iter().collect(),
    })
}

pub fn check_duplicates(hand: &Hand) -> Option<Combination> {
    let duplicates = find_duplicates(hand);

    let duplicate_count = duplicates.len();
    if duplicate_count == 2 {
        return Some(Combination {
            name: "Two pairs",
            cards: duplicates.into_iter().flatten().collect(),
        });
    } else if duplicate_count == 1 {
        let cards = &duplicates[0];
        return match cards.len() {
            2 => Some(Combination {
                name: "Pair",
                cards: cards.to_vec(),
            }),
            3 => Some(Combination {
                name: "Three of a kind",
                cards: cards.to_vec(),
            }),
            4 => Some(Combination {
                name: "Four of a kind",
                cards: cards.to_vec(),
            }),
            _ => None,
        };
    }
    None
}

pub fn check_high(hand: &Hand) -> Option<Combination> {
    let mut high = &hand[0];
    for i in 1..5 {
        if hand[i].quantity > high.quantity {
            high = &hand[i];
        }
    }
    return Some(Combination {
        name: "High card",
        cards: vec![high],
    });
}

fn find_duplicates(hand: &Hand) -> Vec<Vec<&Card>> {
    let mut result = HashMap::new();

    for card in hand {
        match result.entry(card.quantity) {
            Entry::Occupied(mut entry) => {
                let duplicates: &mut Vec<&Card> = entry.get_mut();
                duplicates.push(card);
            }
            Entry::Vacant(entry) => {
                let mut duplicates: Vec<&Card> = Vec::new();
                duplicates.push(card);
                entry.insert(duplicates);
            }
        }
    }

    result
        .into_values()
        .filter(|duplicate| duplicate.len() > 1)
        .collect()
}

#[derive(Debug, PartialEq, Eq)]
pub struct Combination<'a> {
    pub name: &'static str,
    pub cards: Vec<&'a Card>,
}

impl Display for Combination<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(self.name)?;
        fmt.write_str(": ")?;
        for card in self.cards.iter() {
            card.fmt(fmt)?;
            fmt.write_str(" ")?;
        }
        Ok(())
    }
}

pub fn get_rules() -> Vec<Rule> {
    vec![check_flush_royal, check_duplicates, check_high]
}
