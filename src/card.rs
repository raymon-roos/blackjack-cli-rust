pub mod rank;
pub mod suit;

pub use crate::card::rank::*;
pub use crate::card::suit::Suit;

use core::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Card {
        Card { suit, rank }
    }

    pub fn value(&self) -> u8 {
        self.rank.value()
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.suit, self.rank)
    }
}

trait Scoreable {
    fn score() -> u8;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_card_can_show() {
        let card: Card = Card::new(Suit::Hearts, Rank::Face(rank::Face::Queen));

        assert_eq!("♥ Q", card.to_string());
    }
}
