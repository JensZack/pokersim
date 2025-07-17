use crate::pokersim::coms::*;
use crate::pokersim::games::holdem_nl;
use crate::pokersim::dealer::Dealer;
use crate::pokersim::player::*;


pub fn run_pokersim() {
    let zmq_conn = zmq_init();
    
    let msg = Message::ReadyForGameMessage;
    let header: &str = "";
    let rply = zmq_conn.send_recv_message(msg, header);
    println!("{:?}", rply);

    let player_names: Vec<String>;
    let player_chips: Vec<u32>;

    match rply {
        Message::StartGameMessage { players, chips } => {
        player_names = players;
        player_chips = chips;
    },
        _ => panic!("Expected a StartGameMessage, recieved {:?}", rply)
    }
    let mut dealer = Dealer::new();

    let mut players: Vec<Player> = vec![];
    for idx in 0..player_names.len() {
        let player = Player::new(player_names[idx].clone(), player_chips[idx], Some(&zmq_conn));
        players.push(player);
    }
    let blinds: [u32; 2] = [0, 0];
    let ante: u32 = 10;

    while players.len() > 1 {
        holdem_nl(&mut dealer, &mut players, blinds, ante, Some(&zmq_conn));
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    pub fn test_run_pokersim() {
        run_pokersim();
    }
}
