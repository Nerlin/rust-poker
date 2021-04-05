use std::fmt::Display;
use std::cmp::Ordering;

#[derive(Copy, Clone)]
pub struct Card {
    pub suit: Suit,
    pub quantity: Quantity
}

impl Card {
    pub fn new() -> Card {
        Card { suit: Suit::Spades, quantity: Quantity::Ten }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.suit == other.suit && self.quantity == other.quantity
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.quantity, self.suit)
    }
}



#[derive(Copy, Clone, PartialEq)]
pub enum Suit {
    Spades,
    Clubs,
    Diamonds,
    Hearts
}

impl Suit {
    pub fn all() -> &'static [Suit] {
        &[
            Suit::Spades,
            Suit::Clubs,
            Suit::Diamonds,
            Suit::Hearts
        ]
    }
}


impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Suit::Spades => write!(f, "♣"),
            Suit::Clubs => write!(f, "♠"),
            Suit::Diamonds => write!(f, "♦"),
            Suit::Hearts => write!(f, "♥")
        }
    }
}


#[derive(Copy, Clone)]
pub enum Quantity {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14
}

impl Quantity {
    pub fn all() -> &'static [Quantity] {
        &[
            Quantity::Two,
            Quantity::Three,
            Quantity::Four,
            Quantity::Five,
            Quantity::Six,
            Quantity::Seven,
            Quantity::Eight,
            Quantity::Nine,
            Quantity::Ten,
            Quantity::Jack,
            Quantity::Queen,
            Quantity::King,
            Quantity::Ace
        ]
    }
}

impl Display for Quantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Quantity::Jack => write!(f, "J"),
            Quantity::Queen => write!(f, "Q"),
            Quantity::King => write!(f, "K"),
            Quantity::Ace => write!(f, "A"),
            _ => {
                let num = *self as u8;
                write!(f, "{}", num)
            }
        }
    }
}

impl PartialOrd for Quantity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let a = *self as u8;
        let b = *other as u8;
        a.partial_cmp(&b)
    }
}

impl PartialEq for Quantity {
    fn eq(&self, other: &Self) -> bool {
        let a = *self as u8;
        let b = *other as u8;
        a == b
    }
}