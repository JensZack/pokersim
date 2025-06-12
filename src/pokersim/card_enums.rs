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
    pub fn card_from_name(suit: &str, value: &str) -> Card {
        let card_suit: CardSuit = match suit.to_lowercase().as_str() {
            "spade" => CardSuit::SPADE,
            "heart" => CardSuit::HEART,
            "diamond" => CardSuit::DIAMOND,
            "club" => CardSuit::CLUB,
            _ => panic!("no CardSuit: {}", suit) 
        };

        let card_value: CardValue = match value.to_lowercase().as_str() {
            "two" => CardValue::TWO,
            "three" => CardValue::THREE,
            "four" => CardValue::FOUR,
            "five" => CardValue::FIVE,
            "six" => CardValue::SIX,
            "seven" => CardValue::SEVEN,
            "eight" => CardValue::EIGHT,
            "nine" => CardValue::NINE,
            "ten" => CardValue::TEN,
            "jack" => CardValue::JACK,
            "queen" => CardValue::QUEEN,
            "king" => CardValue::KING,
            "ace" => CardValue::ACE,
            _ => panic!("no CardValue: {}", value) 
        };

        Card{ card_value, card_suit }
    }

    pub fn card_from_int(val: &u8) -> Card {
        let v1: u8 = ((val - 1) % 13) + 1; 
        let v2: u8 = ((val - 1) / 13) + 1; 
        let card_value = CardValue::try_from(v1).unwrap();
        let card_suit = CardSuit::try_from(v2).unwrap();
        Card{ card_value, card_suit }
    }

    pub fn to_int(&self) -> u8 {
        let card_val: u8;
        match self.card_value {
            CardValue::TWO => card_val = 1,
            CardValue::THREE => card_val = 2,
            CardValue::FOUR => card_val = 3,
            CardValue::FIVE => card_val = 4,
            CardValue::SIX => card_val = 5,
            CardValue::SEVEN => card_val = 6,
            CardValue::EIGHT => card_val = 7,
            CardValue::NINE => card_val = 8,
            CardValue::TEN => card_val = 9,
            CardValue::JACK => card_val = 10,
            CardValue::QUEEN => card_val = 11,
            CardValue::KING => card_val = 12,
            CardValue::ACE => card_val = 13,
        };
        
        let suit_val: u8;
        match self.card_suit {
            CardSuit::SPADE => suit_val = 1,
            CardSuit::HEART => suit_val = 2,
            CardSuit::DIAMOND => suit_val = 3,
            CardSuit::CLUB => suit_val = 4,
        }
        return card_val + ((suit_val - 1) * 13)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} of {:?}S", self.card_value, self.card_suit) // Write the point's coordinates
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
