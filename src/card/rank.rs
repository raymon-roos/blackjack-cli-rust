use core::fmt;
use std::{fmt::Display, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub enum Face {
    Ace,
    Jack,
    King,
    Queen,
}

impl Display for Face {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Face::Ace => 'A',
                Face::Jack => 'J',
                Face::King => 'K',
                Face::Queen => 'Q',
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RankError {
    InvalidString(String),
    InvalidNumber(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Rank {
    Number(u8),
    Face(Face),
}

impl Rank {
    pub fn value(&self) -> u8 {
        match self {
            Self::Number(value) => value.to_owned(),
            Self::Face(face) => match face {
                Face::Ace => 11,
                _ => 10,
            },
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Number(value) => value.to_string(),
                Self::Face(face) => face.to_string(),
            }
        )
    }
}

impl TryFrom<u8> for Rank {
    type Error = RankError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            u @ 2..=10 => Ok(Rank::Number(u)),
            u => Err(RankError::InvalidNumber(format!(
                "Invalid input, `{u}` is out of range for all variants of Rank"
            ))),
        }
    }
}

impl FromStr for Rank {
    type Err = RankError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(u) = s.parse::<u8>() {
            return Rank::try_from(u);
        }

        match s.to_lowercase().trim() {
            "two" => Ok(Self::Number(2)),
            "three" => Ok(Self::Number(3)),
            "four" => Ok(Self::Number(4)),
            "five" => Ok(Self::Number(5)),
            "six" => Ok(Self::Number(6)),
            "seven" => Ok(Self::Number(7)),
            "eight" => Ok(Self::Number(8)),
            "nine" => Ok(Self::Number(9)),
            "ten" => Ok(Self::Number(10)),
            "jack" | "j" => Ok(Self::Face(Face::Jack)),
            "queen" | "q" => Ok(Self::Face(Face::Queen)),
            "king" | "k" => Ok(Self::Face(Face::King)),
            "ace" | "a" => Ok(Self::Face(Face::Ace)),
            s => Err(RankError::InvalidString(format!(
                "Invalid input, `{s}` does not match any variant of Rank"
            ))),
        }
    }
}

#[cfg(test)]
mod rank_test {
    use super::*;

    #[test]
    fn test_can_convert() {
        let test_rank = "two".parse::<Rank>().unwrap();

        assert_eq!(test_rank, Rank::Number(2));
        assert_eq!(test_rank.value(), 2);

        assert!(Rank::try_from(11).is_err_and(|e| e
            == RankError::InvalidNumber(String::from(
                "Invalid input, `11` is out of range for all variants of Rank"
            ))));

        assert!("lol".parse::<Rank>().is_err_and(|e| e
            == RankError::InvalidString(String::from(
                "Invalid input, `lol` does not match any variant of Rank"
            ))));

        assert_eq!("Q".parse::<Rank>().unwrap(), Rank::Face(Face::Queen));
        assert_eq!("J".parse::<Rank>().unwrap(), Rank::Face(Face::Jack));
    }
}
