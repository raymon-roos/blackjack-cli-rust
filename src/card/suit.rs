#[derive(Debug)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    pub const VALUES: [Self; 4] = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];

    pub fn get_pip(&self) -> char {
        match self {
            Suit::Clubs => '♣',
            Suit::Diamonds => '♦',
            Suit::Hearts => '♥',
            Suit::Spades => '♠',
        }
    }
}

#[cfg(test)]
mod suit_test {
    use super::*;

    #[test]
    fn test_has_correct_pips() {
        assert_eq!('♣', Suit::Clubs.get_pip());
        assert_eq!('♦', Suit::Diamonds.get_pip());
        assert_eq!('♥', Suit::Hearts.get_pip());
        assert_eq!('♠', Suit::Spades.get_pip());
    }
}
