use super::dealer::Dealer;
use super::player::*;


#[derive(Debug)]
struct Round {
    bets: Vec<u32>,
    current_player_idx: usize,
    n_players: usize,
    initial_bets_complete: bool,
    n_plays: usize,
    current_players: Vec<usize>,
}

impl Round {
    pub fn new(n_players: usize, initial_bets: Option<Vec<u32>>) -> Round {
        if n_players < 2 {panic!("Can't play a Round with less than 2 players")}
        let current_players: Vec<usize> = (1..n_players).collect();
        match initial_bets {
            Some(bets) => Round{ bets, current_player_idx: 0, n_players, initial_bets_complete: false, n_plays: 0, current_players },
            None => Round{ bets: vec![0; n_players], current_player_idx: 0, n_players, initial_bets_complete: false, n_plays:0, current_players }
        }
        
    }
    fn next_player(&mut self, move_index: bool) {
        if self.current_players.is_empty() {panic!("Can't find next player, all have folded")}

        self.n_plays += 1;
        if self.n_plays == self.n_players {self.initial_bets_complete = true;}

        if move_index {self.current_player_idx += 1;}
    }

    pub fn next_player_idx(&self) -> usize {
        return self.current_players[self.current_player_idx];
    }

    fn handle_fold(&mut self) {
        // remove current player from current players,
        // next_player doesn't need to be called because removing current player
        // moves the current_player_idx to the next player
        self.current_players.remove(self.current_player_idx);
        self.next_player(false);
    }

    fn handle_bet(&mut self, val: u32) {
        let player_idx: usize = self.current_players[self.current_player_idx];
        self.bets[player_idx] += val;
        self.next_player(true);
    }

    fn round_ended(&self) -> bool {
        // The round has been completed if:
        //  - all bets are equal and initial round is completed
        //  - one player is left

        if self.current_players.len() == 1 {return true};

        let first = &self.current_players[0];
        if self.initial_bets_complete & self.current_players.iter().all(|x| x == first){
            return true
        }
        else {
            return false
        }
    }

    pub fn next_play(&mut self, play: Play) -> bool {
        // return true if round continues after bet
        // return false if the round has ended

        match play {
            Play::Fold => self.handle_fold(),
            Play::Bet(val) => self.handle_bet(val),
        }

        return !self.round_ended();
    }

    pub fn reset_round(&mut self) {
        // reset n_players to reflect the number of players who didn't fold last betting round
        self.n_players = self.current_players.len();
        self.current_player_idx = 0;
        
    }

    pub fn min_next_bet(&self) -> u32 {
        let player_idx: usize = self.current_players[self.current_player_idx];
        return self.bets[player_idx] - self.bets.iter().max().unwrap();
    }

    pub fn pot_total(&self) -> u32 { 
        return self.bets.iter().sum();
    }

    pub fn remaining_players(&self) -> Vec<usize> {
        return self.current_players.iter().copied().collect();
    }

    pub fn one_remaining_player(&self) -> Option<usize> {
        match self.current_players.len() {
            1 => Some(self.current_players[0]),
            _ => None,
        }
    }
}

fn play_holdem_round<T: HoldemPlayer>(players: &mut Vec<&mut T>, round: &mut Round, shared_cards: &Vec<u8>) {
    // failsafe, panic if 1000 plays are made in a round.
    let max_plays: usize = 1_000;
    let mut round_ended: bool = false;

    for _ in 1..max_plays {
        if round_ended {return};

        let min_bet = round.min_next_bet();
        let player_idx = round.next_player_idx();
        let play = players[player_idx].play(shared_cards, min_bet);

        round.next_play(play);
        round_ended = round.round_ended();
    }
    panic!("max plays of {} reached in a single holdem round", max_plays);
}


pub fn holdem_nl<T: HoldemPlayer>(dealer: &mut Dealer, players: &mut Vec<&mut T>, blinds: [u32; 2], ante: u32) {
    if blinds[0] > blinds[1] {panic!("Blinds must be passed in [Little, Big]")}

    dealer.shuffle();

    let n_players = players.len();
    let mut player_position: usize = 1;

    for player in players.iter_mut() {
        player.recieve_cards([dealer.next_card(), dealer.next_card()]);
        player.assign_position(player_position, n_players);
        player.blind(Blind{ amount: ante, btype: BlindType::Ante });
        player_position += 1;
    }

    players[1].blind(Blind{ amount: blinds[0], btype: BlindType::Little });
    players[2].blind(Blind{ amount: blinds[1], btype: BlindType::Big });

    let mut pre_bets: Vec<u32> = vec![ante; n_players];
    pre_bets[1] += blinds[0];
    pre_bets[2] += blinds[1];

    let mut round = Round::new(players.len(), Some(pre_bets));

    let mut shared_cards: Vec<u8> = vec![];

    // Play a betting round with hole cards
    play_holdem_round(players, &mut round, &shared_cards);
    if round.one_remaining_player().is_some() {}

    // Flop
    for _ in 1..3 { shared_cards.push(dealer.next_card()); }
    play_holdem_round(players, &mut round, &shared_cards);

    // Turn
    shared_cards.push(dealer.next_card());
    play_holdem_round(players, &mut round, &shared_cards);
    
    // River
    shared_cards.push(dealer.next_card());
    play_holdem_round(players, &mut round, &shared_cards);

}


#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn test_holdem_nl() {
        let mut dealer = Dealer::new();
        let mut player1 = Player::new("player1".to_string(), 100);
        let mut player2 = Player::new("player2".to_string(), 100);
        let mut players: Vec<&mut Player> = vec![&mut player1, &mut player2];
        let blinds: [u32; 2] = [100, 200];
        holdem_nl(&mut dealer, &mut players, blinds, 0);
    }

}
