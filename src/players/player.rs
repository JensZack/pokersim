
#[derive(Default)]
pub struct Player {
    name: String,
    chips: i32,
    hand: Option<[u8; 2]>,
}

impl Player {
    pub fn new(name: String, chips: i32) -> Player {
        Self{ name, chips, hand: None }
    }
}

pub trait HoldemPlayer {
    fn recieve_cards(&mut self, cards: [u8; 2]) -> ();
    fn show(&self) -> [u8; 2];
    // fn best_hand(&self, shared_cards: &Vec<u8>) -> &Vec<u8>;
    fn play(&self, shared_cards: &Vec<u8>) -> Option<i32>;
    fn bet(&self, shared_cards: &Vec<u8>) -> i32;
    fn fold(&self, shared_cards: &Vec<u8>) -> ();
}

impl HoldemPlayer for Player {
    fn recieve_cards(&mut self, cards: [u8; 2]) {
        self.hand = Some(cards);
    }
    fn bet(&self, shared_cards: &Vec<u8>) -> i32 {
        return 1
    }
    fn show(&self) -> [u8; 2] {
        match self.hand {
            Some(hand) => println!("{}, {}", hand[0], hand[1]),
            None => println!("No Cards"),
        }
        return [1, 2];
        
    }
    fn play(&self, shared_cards: &Vec<u8>) -> Option<i32> {
        if shared_cards.len() == 5 {
            return None
        } else {
            return Some(1)
        }
    }
    fn fold(&self, shared_cards: &Vec<u8>) -> () {
        
    }
    // fn best_hand(&self, shared_cards: &Vec<u8>) -> &Vec<u8> {
    //     
    // }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn test_recieve_cards() {
        let mut player = Player::new("test".to_string(), 10);
        let cards: [u8; 2] = [10, 15];
        player.recieve_cards(cards)
    }

}
