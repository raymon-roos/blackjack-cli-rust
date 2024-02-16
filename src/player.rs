pub mod hand;

use self::hand::*;
use super::card::*;

#[derive(Debug)]
pub struct Player {
    name: String,
    hands: Vec<Hand>,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name: name.trim().to_string(),
            hands: vec![Hand::new(vec![])],
        }
    }

    pub fn take_card(&mut self, card: Card) {
        self.hands
            .first_mut()
            .expect("every player has at least one hand")
            .push(card);
    }

    pub fn show_hands(&self) -> String {
        let all_cards = self
            .hands
            .iter()
            .fold(format!("Player {} has:", self.name), |output, hand| {
                format!("{} [{}]", output, hand)
            });

        dbg!(&all_cards);

        all_cards
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_player_can_show() {
        let card: Card = Card::new(Suit::Hearts, Rank::Face(Face::Queen));

        let mut player = Player::new(String::from("test"));
        player.take_card(card);

        assert_eq!("Player test has: [♥ Q]", player.show_hands());
    }

    #[test]
    fn test_hand_can_take_card() {
        let mut hand = Hand::new(vec![]);
        hand.push(Card::new(Suit::Diamonds, Rank::Number(10)));
        hand.push(Card::new(Suit::Spades, Rank::Number(2)));

        assert_eq!("♦ 10 ♠ 2", hand.to_string());
        assert_eq!(12, hand.score());
    }
}
