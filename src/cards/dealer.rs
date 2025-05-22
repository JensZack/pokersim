use rand::seq::SliceRandom;
use rand::rng;


#[derive(Debug)]
pub struct Dealer{
    deck: Vec<u8>
}

impl Dealer {

    pub fn shuffle() -> Vec<i32> {
        let mut deck: Vec<i32> = (1..=52).collect();
        let mut rng = rng();
        deck.shuffle(&mut rng);
        deck
    }

}
