use super::coms::*;
use std::fmt;


#[derive(Default)]
pub struct Player<'a> {
    name: String,
    chips: u32,
    hand: Option<[u8; 2]>,
    // position counts from 1, contains [player_position, n_players]
    position: Option<[usize; 2]>,
    pot_contrib: u32,
    zmq_reply_socket: Option<&'a ZmqSocketReply>,
}

impl fmt::Debug for Player<'_> {    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
         .field("name", &self.name)
         .field("chips", &self.chips)
         .field("hand", &self.hand)
         .field("position", &self.position)
         .field("pot_contribution", &self.pot_contrib)
         .finish()
    }

}

impl<'a> Player<'a> {
    pub fn new(name: String, chips: u32, zmq_reply_socket: Option<&'a ZmqSocketReply>) -> Player<'a> {
        Self{ name, chips, hand: None, position: None, pot_contrib: 0, zmq_reply_socket: zmq_reply_socket }
    }

    pub fn msg_header(&self) -> String {
        return self.name.to_string();
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
    fn play(&mut self, current_players: &Vec<usize>, current_position: usize, current_pot: u32, min_bet: u32, shared_cards: &Vec<u8>, turn: &Turn) -> Play;
    fn bet(&mut self, shared_cards: &Vec<u8>, min_call: u32) -> u32;
    fn fold(&mut self) -> ();
    fn assign_position(&mut self, player_position: usize, n_players: usize) -> ();
    fn end_round(&mut self, winnings: Option<u32>) -> ();
    fn pot_contribution(&self) -> u32;
}

impl<'a> HoldemPlayer for Player<'a> {
    fn recieve_cards(&mut self, cards: [u8; 2]) {
        self.hand = Some(cards);
    }
    fn blind(&mut self, blind: Blind) -> u32 {
        if self.chips < blind.amount {panic!("Player {} has lost", self.name)}
        self.chips -= blind.amount;
        return blind.amount
    }
    fn bet(&mut self, _shared_cards: &Vec<u8>, min_call: u32) -> u32 {

        // bet of 0 == check
        self.pot_contrib += min_call;
        self.chips -= min_call;
        return min_call;
    }
    fn show(&self) -> [u8; 2] {
        match self.hand {
            Some(hand) => return hand,
            None => panic!("No Cards"),
        }
    }
    fn play(
        &mut self,
        current_players: &Vec<usize>,
        current_position: usize,
        current_pot: u32,
        min_bet: u32,
        shared_cards: &Vec<u8>,
        turn: &Turn,
    ) -> Play {
        // TODO: This play function assumes that the server is also keeping track of player state
        //      Investigate moving to stateless play call and maybe RPC?
        if self.hand.is_none() {panic!("Player {} can't play, player has no cards", self.name)}

        let play: Message;
        match &self.zmq_reply_socket {
            Some(conn) => {
                let msg = Message::TurnMessage { 
                    current_players: current_players.clone(),
                    current_position: current_position,
                    current_pot: current_pot,
                    min_bet: min_bet,
                    max_bet: self.chips,
                    shared_cards: shared_cards.clone(),
                    turn: turn.clone()
                };
                play = conn.send_recv_message(msg, &self.msg_header());
            },
            None => panic!("Can't play without zmq connection")
        }
        match play {
            Message::PlayMessage{play_type, bet_amount} => {
                match play_type {
                    Plays::FOLD => {return Play::Fold},
                    Plays::BET => {return Play::Bet(bet_amount)},
                };
            },
            _ => panic!("Expected a PlayMessage in Player.play()")
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
        self.pot_contrib = 0;
        self.position = None;
        self.hand = None;

        match winnings {
            Some(val) => self.chips += val,
            None => (),
        };
    }
    fn pot_contribution(&self) -> u32 {
        return self.pot_contrib;
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn test_recieve_cards() {
        let zmq_conn = zmq_init();
        let mut player = Player::new("test".to_string(), 10, Some(&zmq_conn));
        let cards: [u8; 2] = [10, 15];
        player.recieve_cards(cards)
    }

    pub fn test_play() {
        let zmq_conn = zmq_init();
        let mut player = Player::new("test".to_string(), 10, Some(&zmq_conn));
        let cards: [u8; 2] = [20, 30];
        player.recieve_cards(cards);

        let mut shared_cards: Vec<u8> = [2, 3, 4].to_vec();
        let mut players: Vec<usize> = vec![1, 2, 3];
        let current_position: usize = 2;
        let current_pot: u32 = 10_000;
        let min_bet: u32 = 1_000;
        let turn = Turn::FLOP;
        let _play1 = player.play(&players, current_position, current_pot, min_bet, &shared_cards, &turn);
        assert_eq!(player.chips, 9);

        shared_cards.push(5);
        let _play2 = player.play(&players, current_position, current_pot, min_bet, &shared_cards, &turn);
        assert_eq!(player.chips, 8);

        shared_cards.push(6);
        let _play3 = player.play(&players, current_position, current_pot, min_bet, &shared_cards, &turn);
        assert_eq!(player.chips, 8);
        assert!(player.hand.is_none());
    }

    pub fn test_zmq_coms() {
        let zmq_conn = zmq_init();
        let player1 = Player::new("test_player1".to_string(), 10_000, Some(&zmq_conn));
        let player2 = Player::new("test_player2".to_string(), 10_000, Some(&zmq_conn));
    }

}
