pub mod hand;

use self::hand::*;

#[derive(Debug)]
pub struct Player {
    name: String,
    hands: Vec<Hand>,
    score: u8,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            hands: vec![Hand::new(vec![])],
            score: 0,
        }
    }

    pub fn show_hands(&self) {
        let hands: Vec<String> = self.hands.iter().map(|hand| hand.show()).collect();

        for hand in self.hands.iter() {
            println!("Player {} hand #{}: {}", self.name, hand.show());
        }
    }
}

#[cfg(test)]
mod test {
    use crate::card::*;

    use super::*;

    #[test]
    fn test_player_can_show() {
        let card: Card = Card::new(Suit::Hearts, Rank::Face(Face::Queen));
        let player = Player::new(String::from("test"));

        assert_eq!("♥ Q", player.show_hand());
    }

    #[test]
    fn test_hand_can_show() {
        let card1: Card = Card::new(Suit::Hearts, Rank::Face(rank::Face::Queen));
        let card2: Card = Card::new(Suit::Spades, Rank::Face(rank::Face::Ace));

        let hand = Hand::new(vec![card1, card2]);

        assert_eq!("♥ Q ♠ A", hand.show());
    }

    #[test]
    fn test_hand_can_take_card() {
        let mut hand = Hand::new(vec![]);
        hand.push(Card::new(Suit::Diamonds, Rank::Number(10)));
        hand.push(Card::new(Suit::Spades, Rank::Number(2)));

        assert_eq!("♦ 10 ♠ 2", hand.show());
        assert_eq!(12, hand.score());
    }
}
