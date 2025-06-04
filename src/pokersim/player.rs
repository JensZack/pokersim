
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
    fn best_hand(&self, shared_cards: &Vec<u8>) -> [u8; 5];
    fn play(&mut self, shared_cards: &Vec<u8>) -> Option<u32>;
    fn bet(&mut self, shared_cards: &Vec<u8>) -> u32;
    fn fold(&mut self) -> ();
}

impl HoldemPlayer for Player {
    fn recieve_cards(&mut self, cards: [u8; 2]) {
        self.hand = Some(cards);
    }
    fn bet(&mut self, _shared_cards: &Vec<u8>) -> u32 {
        // bet of 0 == check
        if self.chips > 1 {
            self.chips -= 1;
            1
        } else {
            0
        }
    }
    fn show(&self) -> [u8; 2] {
        match self.hand {
            Some(hand) => return hand,
            None => panic!("No Cards"),
        }
        
    }
    fn play(&mut self, shared_cards: &Vec<u8>) -> Option<u32> {
        if self.hand.is_none() {panic!("Player {} can't play, player has no cards", self.name)}
        if shared_cards.len() == 5 {
            self.fold();
            return None
        } else {
            return Some(self.bet(shared_cards))
        }
    }
    fn fold(&mut self) -> () {
        self.hand = None;
    }
    fn best_hand(&self, shared_cards: &Vec<u8>) -> [u8; 5] {
        match self.hand {
            Some(hand) => return [hand[0], hand[1], shared_cards[0], shared_cards[1], shared_cards[2]],
            None => panic!("Player {} has no cards", self.name)
        }
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn test_recieve_cards() {
        let mut player = Player::new("test".to_string(), 10);
        let cards: [u8; 2] = [10, 15];
        player.recieve_cards(cards)
    }

    pub fn test_play() {
        let mut player = Player::new("test".to_string(), 10);
        let cards: [u8; 2] = [20, 30];
        player.recieve_cards(cards);

        let mut shared_cards: Vec<u8> = [2, 3, 4].to_vec();
        let _play1 = player.play(&shared_cards);
        assert_eq!(player.chips, 9);

        shared_cards.push(5);
        let _play2 = player.play(&shared_cards);
        assert_eq!(player.chips, 8);

        shared_cards.push(6);
        let _play3 = player.play(&shared_cards);
        assert_eq!(player.chips, 8);
        assert!(player.hand.is_none());
    }

}
