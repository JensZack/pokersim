use std::convert::TryFrom;
use std::fmt;


#[derive(Debug)]
enum CardValue {
    TWO = 1,
    THREE = 2,
    FOUR = 3,
    FIVE = 4,
    SIX = 5,
    SEVEN = 6,
    EIGHT = 7,
    NINE = 8,
    TEN = 9,
    JACK = 10,
    QUEEN = 11,
    KING = 12,
    ACE = 13,
}

#[derive(Debug)]
enum CardSuit {
    SPADE = 1,
    HEART = 2,
    DIAMOND = 3,
    CLUB = 4,
}

pub struct Card {
    card_value: CardValue, 
    card_suit: CardSuit
}

impl Card {
    pub fn card_from_int(val: &u8) -> Card {
        let card_value = CardValue::try_from(val.rem_euclid(13) + 1).unwrap();
        let card_suit = CardSuit::try_from(val % 4 + 1).unwrap();
        Card{ card_value, card_suit}
    }

}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} of {:?}s", self.card_value, self.card_suit) // Write the point's coordinates
    }
}

impl TryFrom<u8> for CardValue {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == CardValue::TWO as u8 => Ok(CardValue::TWO),
            x if x == CardValue::THREE as u8 => Ok(CardValue::THREE),
            x if x == CardValue::FOUR as u8 => Ok(CardValue::FOUR),
            x if x == CardValue::FIVE as u8 => Ok(CardValue::FIVE),
            x if x == CardValue::SIX as u8 => Ok(CardValue::SIX),
            x if x == CardValue::SEVEN as u8 => Ok(CardValue::SEVEN),
            x if x == CardValue::EIGHT as u8 => Ok(CardValue::EIGHT),
            x if x == CardValue::NINE as u8 => Ok(CardValue::NINE),
            x if x == CardValue::TEN as u8 => Ok(CardValue::TEN),
            x if x == CardValue::JACK as u8 => Ok(CardValue::JACK),
            x if x == CardValue::QUEEN as u8 => Ok(CardValue::QUEEN),
            x if x == CardValue::KING as u8 => Ok(CardValue::KING),
            x if x == CardValue::ACE as u8 => Ok(CardValue::ACE),
            _ => Err(()),
        }
    }
}


impl TryFrom<u8> for CardSuit {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == CardSuit::SPADE as u8 => Ok(CardSuit::SPADE),
            x if x == CardSuit::CLUB as u8 => Ok(CardSuit::CLUB),
            x if x == CardSuit::DIAMOND as u8 => Ok(CardSuit::DIAMOND),
            x if x == CardSuit::HEART as u8 => Ok(CardSuit::HEART),
            _ => Err(()),
        }
    }
}
