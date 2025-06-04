use rand::seq::SliceRandom;
use rand::rng;


#[derive(Debug)]
pub struct Dealer{
    card_idx: usize,
    pub deck: Vec<u8>,
}

impl Dealer {

    pub fn new() -> Dealer {
        let deck: Vec<u8> = (1..=52).collect();
        Self{ card_idx: 0, deck }
    }

    pub fn shuffle(&mut self) -> () {
        self.card_idx = 0;
        let mut rng = rng();
        self.deck.shuffle(&mut rng);
    }

    pub fn next_card(&mut self) -> u8 {
        if self.card_idx >= 51 {panic!("dealer is out of cards")}
        let card: u8 = self.deck[self.card_idx];
        self.card_idx += 1;
        return card
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_shuffle() { 
        let mut dealer = Dealer::new();
        for i in 1..51 {
            assert!(dealer.deck[i] < dealer.deck[i + 1])
        } 
        let init_deck = dealer.deck.to_vec();

        dealer.shuffle();
        assert_ne!(init_deck, dealer.deck)
        
    }

}
