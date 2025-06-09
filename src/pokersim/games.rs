use super::dealer::Dealer;
use super::player::*;


#[derive(Debug)]
struct Round {
    bets: Vec<Option<u32>>,
    player_idx: usize,
    n_players: usize,
    initial_round_complete: bool,
}

impl Round {
    pub fn new(n_players: usize, initial_bets: Option<Vec<Option<u32>>>) -> Round {
        if n_players < 2 {panic!("Can't play a Round with less than 2 players")}
        match initial_bets {
            Some(bets) => Round{ bets, player_idx: 0, n_players, initial_round_complete: false },
            None => Round{ bets: vec![Some(0); n_players], player_idx: 0, n_players, initial_round_complete: false }
        }
        
    }
    fn next_player(&mut self) {
        if self.player_idx == self.n_players {self.initial_round_complete = true};
        self.player_idx = self.player_idx + 1 % self.n_players;
    }

    pub fn next_player_idx(&self) -> usize {
        return self.player_idx;
    }

    pub fn next_bet(&mut self, bet: u32) -> bool {
        // return true if round continues after bet
        // return false if the round has ended

        self.bets[self.player_idx] += bet;
        self.next_player();

        let mut round_complete = false;
        if self.initial_round_complete {
            let first = &self.bets[0];
            round_complete = self.bets.iter().all(|x| x == first);
        }
        return round_complete;
    }

    pub fn min_next_bet(&self) -> u32 {
        return self.bets[self.player_idx] - self.bets.iter().max().unwrap();
    }
}

fn play_holdem_round<T: HoldemPlayer>(players: &mut Vec<&mut T>, shared_cards: &Vec<u8>, pre_bets: Option<Vec<u32>>) {
    // failsafe, panic if 1000 plays are made in a round.
    let max_plays: usize = 1_000;
    let mut round = Round::new(players.len(), pre_bets);
    let mut round_ended: bool = false;
    for x in 1..max_plays {
        if round_ended {return};
        
    }
    panic!("max plays of {} reached in a single holdem round", max_plays);
}


pub fn holdem_nl<T: HoldemPlayer>(dealer: &mut Dealer, players: &mut Vec<&mut T>, blinds: [u32; 2], ante: u32) {
    if blinds[0] > blinds[1] {panic!("Blinds must be passed in [Little, Big]")}

    dealer.shuffle();

    let mut min_individual_pot_contribution = 0;
    let n_players = players.len();
    let player_position: usize = 1;

    for player in players.iter_mut() {
        player.recieve_cards([dealer.next_card(), dealer.next_card()]);
        player.assign_position(player_position, n_players);
        player.blind(Blind{ amount: ante, btype: BlindType::Ante });
    }

    players[1].blind(Blind{ amount: blinds[0], btype: BlindType::Little });
    players[2].blind(Blind{ amount: blinds[1], btype: BlindType::Big });

    min_individual_pot_contribution += ante + blinds[1];

    // players have to match big blind to stay in after hole cards are dealt
    //
    // I need a way to handle the round ending once all players are even 
    // I also need to handle the minimum a player needs to stay in based on other players

    let mut shared_cards: Vec<u8> = vec![];
    for _ in 1..3 { shared_cards.push(dealer.next_card()); }
    let round_on: bool = true;

    while round_on {
        player.play(&shared_cards);
    }
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
