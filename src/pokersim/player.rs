
#[derive(Default)]
pub struct Player {
    name: String,
    chips: u32,
    hand: Option<[u8; 2]>,
    // position counts from 1, contains [player_position, n_players]
    position: Option<[usize; 2]>,
    pot_contribution: u32,
}

impl Player {
    pub fn new(name: String, chips: u32) -> Player {
        Self{ name, chips, hand: None, position: None, pot_contribution: 0 }
    }
}

pub enum BlindType {
    Big,
    Little,
    Ante,
}

pub struct Blind {
    pub amount: u32,
    pub btype: BlindType,
}

pub enum Play {
    Bet(u32),
    Fold,
}


pub trait HoldemPlayer {
    fn recieve_cards(&mut self, cards: [u8; 2]) -> ();
    // blind also handles antes
    fn blind(&mut self, blind: Blind) -> u32;
    fn show(&self) -> [u8; 2];
    fn best_hand(&self, shared_cards: &Vec<u8>) -> [u8; 5];
    fn play(&mut self, shared_cards: &Vec<u8>, min_call: u32, max_bet: u32) -> Play;
    fn bet(&mut self, shared_cards: &Vec<u8>, min_call: u32, max_bet: u32) -> u32;
    fn fold(&mut self) -> ();
    fn assign_position(&mut self, player_position: usize, n_players: usize) -> ();
    fn end_round(&mut self, winnings: Option<u32>) -> ();
}

impl HoldemPlayer for Player {
    fn recieve_cards(&mut self, cards: [u8; 2]) {
        self.hand = Some(cards);
    }
    fn blind(&mut self, blind: Blind) -> u32 {
        if self.chips < blind.amount {panic!("Player {} has lost", self.name)}
        self.chips -= blind.amount;
        return blind.amount
    }
    fn bet(&mut self, _shared_cards: &Vec<u8>, min_call: u32, _max_bet: u32) -> u32 {

        // bet of 0 == check
        self.pot_contribution += min_call;
        self.chips -= min_call;
        return min_call;
    }
    fn show(&self) -> [u8; 2] {
        match self.hand {
            Some(hand) => return hand,
            None => panic!("No Cards"),
        }
    }
    fn play(&mut self, shared_cards: &Vec<u8>, min_call: u32, max_bet: u32) -> Play {
        if self.hand.is_none() {panic!("Player {} can't play, player has no cards", self.name)}
        if shared_cards.len() == 5 {
            self.fold();
            return Play::Fold;
        } else {
            return Play::Bet(self.bet(shared_cards, min_call, max_bet))
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
    fn assign_position(&mut self, player_position: usize, n_players: usize) -> () {
        self.position = Some([player_position, n_players]);
    }
    fn end_round(&mut self, winnings: Option<u32>) -> () {
        self.pot_contribution = 0;
        self.position = None;
        self.hand = None;

        match winnings {
            Some(val) => self.chips += val,
            None => (),
        };
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
        let _play1 = player.play(&shared_cards, 0, 100);
        assert_eq!(player.chips, 9);

        shared_cards.push(5);
        let _play2 = player.play(&shared_cards, 0, 100);
        assert_eq!(player.chips, 8);

        shared_cards.push(6);
        let _play3 = player.play(&shared_cards, 0, 100);
        assert_eq!(player.chips, 8);
        assert!(player.hand.is_none());
    }

}
