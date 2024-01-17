## This is a simplified version of Blackjack, written in Rust

### Why does this exist

Because I enjoyed doing this in PHP, and I enjoyed learning about Rust, so I wanted try
making something I'm familiar with, in a language I'm not yet experienced at.

### What it does

So far, nothing but run tests, as I slowly work on figuring out how to structure a Rust
project, and implementing features. This project is far from a priority, so it won't
progress very quickly.

A big difference I already noticed is, unlike in PHP, Rust function arguments can have
only one type, meaning I'll have to create additional functions if I want similar behaviour
for multiple types. 

#### Planned features

- Adding up to 7 players.
- Players take turns drawing cards (hitting or standing).
- Placing bets, and winning or losing “money”.
- Player are presented their score and gains/losses at the end of the game.
- Splitting hands.
- Doubling down.
- Ace counting as either 1 or 11.

#### Features it doesn't have

- Multiplayer over a network
- Placing “insurance” bets
- Surrendering

### How to use it

Run `cargo run` from the root of this project, and follow the on-screen
instructions. 

### Design considerations 

So far, I'm just exploring how to work with Rust. Apart from being inspired by my PHP
implementation, I haven't made any cohesive design decisions yet. 
