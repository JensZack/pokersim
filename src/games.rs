use crate::cards::dealer::Dealer;
use crate::players::player;


pub fn holdem_nl(dealer: &mut Dealer, _players: Vec<&mut player::Player>) {
    dealer.shuffle()
}


#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn test_holdem_nl() {
        let mut dealer = Dealer::new();
        let mut player1 = player::Player::new("player1".to_string(), 100);
        let mut player2 = player::Player::new("player2".to_string(), 100);
        let players: Vec<&mut player::Player> = vec![&mut player1, &mut player2];
        holdem_nl(&mut dealer, players);
    }

}
