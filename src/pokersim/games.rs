use super::dealer::Dealer;
use super::player::*;


pub fn holdem_nl<T: HoldemPlayer>(dealer: &mut Dealer, players: Vec<&mut T>) {
    dealer.shuffle();
    for player in players {
        player.recieve_cards([dealer.next_card(), dealer.next_card()])
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn test_holdem_nl() {
        let mut dealer = Dealer::new();
        let mut player1 = Player::new("player1".to_string(), 100);
        let mut player2 = Player::new("player2".to_string(), 100);
        let players: Vec<&mut Player> = vec![&mut player1, &mut player2];
        holdem_nl(&mut dealer, players);
    }

}
