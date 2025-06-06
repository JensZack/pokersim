use super::dealer::Dealer;
use super::player::*;


pub fn holdem_nl<T: HoldemPlayer>(dealer: &mut Dealer, players: &mut Vec<&mut T>, blinds: [u32; 2], ante: Option<u32>) {
    dealer.shuffle();
    let n_players = players.len();
    let player_position: usize = 1;
    for player in players.iter_mut() {
        player.recieve_cards([dealer.next_card(), dealer.next_card()]);
        player.assign_position(player_position, n_players);
        if let Some(val) = ante {
            player.blind(Blind{ amount: val, btype: BlindType::Ante });
        }
    }
    players[1].blind(Blind{ amount: blinds[1], btype: BlindType::Big });
    players[2].blind(Blind{ amount: blinds[0], btype: BlindType::Little });
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
        holdem_nl(&mut dealer, &mut players, blinds, None);
    }

}
