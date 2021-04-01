use std::fmt;
use std::str::FromStr;

extern crate clap;
use clap::{Arg, App};
extern crate itertools;
use itertools::Itertools;
extern crate strum_macros;
use strum_macros::{EnumString};


#[derive(Debug, PartialEq, EnumString)]
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
    write!(f, "<Card: {}{}>", self.rank.to_string(), self.suit.to_string())
  }
}


#[derive(Debug)]
struct OmahaHand {
  hole_cards: Vec<Card>,
  board_cards: Vec<Card>,
}


fn string_to_cards(s: String) -> Result<Vec<Card>, strum::ParseError> {
    let mut cards: Vec<Card> = Vec::new();
    for chunk in &s.chars().chunks(2)
    {
        let vec = chunk.collect::<Vec<char>>();
        /* ['A', 'd] */
        //println!("{:?}", vec);
        let r = Rank::from_str(&String::from(vec[0])).expect("Invalid rank");
        let s = Suit::from_str(&String::from(vec[1])).expect("Invalid suit");
        let card = Card::new(r, s);
        /* Card { rank: Ace, suit: Diamond } */
        //println!("{:?}", card);
        /* <Card: Ad> */
        //println!("{}", card.to_string());
        cards.push(card);
    }

    Ok(cards)
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
    if hole_card_str.len() != 8
    {
      println!("Error: four hole cards are required");
      return ();
    }
    let hole_cards = string_to_cards(hole_card_str.to_string()).unwrap();
    println!("Hand: {:?}", hole_cards);
}

