use std::fmt;
use std::str::FromStr;

extern crate clap;
use clap::{Arg, App};
extern crate itertools;
use itertools::Itertools;

pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl fmt::Display for Rank {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Rank::Two => write!(f, "2"),
      Rank::Three => write!(f, "3"),
      Rank::Four => write!(f, "4"),
      Rank::Five => write!(f, "5"),
      Rank::Six => write!(f, "6"),
      Rank::Seven => write!(f, "7"),
      Rank::Eight => write!(f, "8"),
      Rank::Nine => write!(f, "9"),
      Rank::Ten => write!(f, "T"),
      Rank::Jack => write!(f, "J"),
      Rank::Queen => write!(f, "Q"),
      Rank::King => write!(f, "K"),
      Rank::Ace => write!(f, "A"),
    }
  }
}

impl FromStr for Rank {
  
  type Err = ();
  
  fn from_str(input: &str) -> Result<Rank, Self::Err> {
    match input {
      "2" => Ok(Rank::Two),
      "3" => Ok(Rank::Three),
      "4" => Ok(Rank::Four),
      "5" => Ok(Rank::Five),
      "6" => Ok(Rank::Six),
      "7" => Ok(Rank::Seven),
      "8" => Ok(Rank::Eight),
      "9" => Ok(Rank::Nine),
      "T" => Ok(Rank::Ten),
      "J" => Ok(Rank::Jack),
      "Q" => Ok(Rank::Queen),
      "K" => Ok(Rank::King),
      "A" => Ok(Rank::Ace),
       _  => Err(()),
    }
  }
}

pub enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

impl fmt::Display for Suit {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Suit::Spade => write!(f, "s"),
      Suit::Heart => write!(f, "h"),
      Suit::Diamond => write!(f, "d"),
      Suit::Club => write!(f, "c"),
    }
  }
}

impl FromStr for Suit {
  
  type Err = ();
  
  fn from_str(input: &str) -> Result<Suit, Self::Err> {
    match input {
      "s" => Ok(Suit::Spade),
      "h" => Ok(Suit::Heart),
      "d" => Ok(Suit::Diamond),
      "c" => Ok(Suit::Club),
       _  => Err(()),
    }
  }
}

struct Card {
  rank: Rank,
  suit: Suit,
}

impl Card {
  pub fn new(rank: Rank, suit: Suit) -> Card {
    Card{ rank, suit }
  }
}

impl fmt::Display for Card {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "<Card: {}{}", self.rank.to_string(), self.suit.to_string())
  }
}

// implement from_string for Card

fn main() {
    let matches = App::new("Omaha Odds Calculator (ooc-rs)")
                     .version("0.1")
                     .about("A calculator for odds for Omaha Hi poker")
                     .arg(Arg::with_name("hole_cards")
                         .help("Start cards (player specific)")
                         .required(true)
                         .index(1))
                     .arg(Arg::with_name("flop")
                         .help("The 3-card flop")
                         .required(false)
                         .index(2))
                     .arg(Arg::with_name("turn")
                         .help("The turn")
                         .required(false)
                         .index(3))
                     .arg(Arg::with_name("river")
                         .help("The river")
                         .required(false)
                         .index(4))
                     .get_matches();
    let hole_card_str = matches.value_of("hole_cards").unwrap();
    println!("Hand: {}", hole_card_str);
}

