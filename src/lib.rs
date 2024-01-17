use crate::card::*;
use crate::player::*;

pub mod card;
pub mod player;

pub fn test() {
    let card: Card = Card::new(Suit::Hearts, Rank::Face(Face::Queen));
    println!("{}", card.to_string());
    dbg!(card);
}
