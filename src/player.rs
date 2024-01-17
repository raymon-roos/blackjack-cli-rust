use crate::card::rank::*;
use crate::card::suit::Suit;

#[derive(Debug)]
pub struct Player {
    name: String,
    hand: Vec<Rank>,
    score: u8,
}

impl Player {
    pub fn new(suit: Suit, rank: Rank) -> Card {
        Card { suit, rank }
    }
}

impl ToString for Card {
    fn to_string(&self) -> String {
        format!("{} {}", self.suit.get_pip(), self.rank.get_value())
    }
}

#[cfg(test)]
mod card_test {
    use super::*;

    #[test]
    fn test_card_can_show() {
        let card: Card = Card::new(Suit::Hearts, Rank::Face(rank::Face::Queen));

        assert_eq!("♥ 10", card.to_string());
    }
}
