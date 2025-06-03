use rand::seq::SliceRandom;
use rand::rng;


#[derive(Debug)]
pub struct Dealer{
    pub deck: Vec<u8>
}

impl Dealer {

    pub fn new() -> Dealer {
        let deck: Vec<u8> = (1..=52).collect();
        Self{ deck }
    }

    pub fn shuffle(&mut self) -> () {
        let mut rng = rng();
        self.deck.shuffle(&mut rng);
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
