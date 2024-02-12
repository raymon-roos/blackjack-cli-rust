use crate::card::*;

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>,
    score: u8,
}

impl Hand {
    pub fn new(cards: Vec<Card>) -> Hand {
        let score = cards.iter().fold(0, |score, card| score + card.value());

        Hand { cards, score }
    }

    pub fn push(&mut self, card: Card) {
        self.score += card.value();
        self.cards.push(card);
    }

    pub fn score(&self) -> u8 {
        self.score
    }

    pub fn show(&self) -> String {
        let cards: Vec<String> = self.cards.iter().map(|card| card.to_string()).collect();
        let output = cards
            .into_iter()
            .reduce(|output, card_string| format!("{} {}", output, card_string));

        output.expect("Cant have hands without cards")
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
