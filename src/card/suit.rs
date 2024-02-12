#[derive(Debug)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    pub const VALUES: [Self; 4] = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];

    /// In poker-style games, the symbol represinting the suit of a card is called a "pip"
    pub fn pip(&self) -> char {
        match self {
            Suit::Clubs => '♣',
            Suit::Diamonds => '♦',
            Suit::Hearts => '♥',
            Suit::Spades => '♠',
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_has_correct_pips() {
        assert_eq!('♣', Suit::Clubs.pip());
        assert_eq!('♦', Suit::Diamonds.pip());
        assert_eq!('♥', Suit::Hearts.pip());
        assert_eq!('♠', Suit::Spades.pip());
    }
}
