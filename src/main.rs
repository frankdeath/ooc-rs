use std::fmt;

extern crate clap;
use clap::{Arg, App};
extern crate itertools;
use itertools::Itertools;
extern crate strum_macros;
use strum_macros::{EnumIter,EnumString};


#[derive(Debug, PartialEq, EnumString, EnumIter)]
pub enum Rank {
    #[strum(serialize = "two", serialize = "2")]
    Two,
    #[strum(serialize = "three", serialize = "3")]
    Three,
    #[strum(serialize = "four", serialize = "4")]
    Four,
    #[strum(serialize = "five", serialize = "5")]
    Five,
    #[strum(serialize = "six", serialize = "6")]
    Six,
    #[strum(serialize = "seven", serialize = "7")]
    Seven,
    #[strum(serialize = "eight", serialize = "8")]
    Eight,
    #[strum(serialize = "nine", serialize = "9")]
    Nine,
    #[strum(serialize = "ten", serialize = "T")]
    Ten,
    #[strum(serialize = "jack", serialize = "J")]
    Jack,
    #[strum(serialize = "queen", serialize = "Q")]
    Queen,
    #[strum(serialize = "king", serialize = "K")]
    King,
    #[strum(serialize = "ace", serialize = "A")]
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

#[derive(Debug, EnumString)]
pub enum Suit {
    #[strum(serialize = "spade", serialize = "s")]
    Spade,
    #[strum(serialize = "heart", serialize = "h")]
    Heart,
    #[strum(serialize = "diamond", serialize = "d")]
    Diamond,
    #[strum(serialize = "club", serialize = "c")]
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

#[derive(Debug)]
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
    for chunk in &hole_card_str.chars().chunks(2)
    {
        let mut card_str = String::from("");
        for c in chunk {
            //print!("{}\n", c);
            card_str.push(c);
        }
        println!("{}", card_str);
    }
}

