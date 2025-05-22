use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn shuffle() -> Vec<i32> {
    let mut deck: Vec<i32> = (1..=52).collect();
    let mut rng = thread_rng();
    deck.shuffle(&mut rng);
    deck
}
