use serde::{Serialize, Deserialize};
use serde_json;
use zmq;
// TODO: protobufs would be better for python/rust interop

pub trait SendMessage {
    fn send();
}

pub trait RecvMessage {
    fn recieve(msg: &str) -> Self;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Plays {
    BET,
    FOLD,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerHand {
    cards: [u8; 2],
    player_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Status {
    OK,
    ERROR,
}

impl Status {
    pub fn check_status(&self, err_msg: &str) {
        match self {
            Status::OK => (),
            Status::ERROR => panic!("{}", err_msg),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Turn {
    HOLE=0,
    FLOP=1,
    TURN=2,
    RIVER=3,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "message_type", content = "message")]
pub enum Message {

    // pokersim sends ReadyForGameMessage and recieves StartGameMessage to initialize players
    // Send
    ReadyForGameMessage,

    // Message for Table
    // Recv
    StartGameMessage { players: Vec<String>, chips: Vec<u32> },

    // pokersim sends RoundInitMessage and recieves a StatusMessage
    // Message for Table
    // Send
    RoundInitMessage { ante: u32, blinds: [u32; 2], n_players: usize, position: usize },

    // pokersim sends a TurnMessage and recieves a PlayMessage
    // Message for a given player
    // Send
    TurnMessage { 
        current_players: Vec<usize>, 
        current_position: usize, 
        current_pot: u32, 
        min_bet: u32, 
        max_bet: u32,
        shared_cards: Vec<u8>, 
        turn: Turn 
    },

    // Message for a given player
    // Recv
    PlayMessage { play_type: Plays, bet_amount: u32},
    
    // pokersim sends an EndOfRoundMesssge to send information about
    // Message for a given player
    // Send
    EndOfRoundMessage { winnings: Vec<i32> },
    
    // Messages for Table
    // Recv
    StatusMessage { status: Status },

    // pokersim sends message when game is over (one player wins)
    // Send
    EndOfGameMessage,
}

pub struct ZmqSocketReply {
    socket: zmq::Socket,
}

impl ZmqSocketReply {

    pub fn new(socket: zmq::Socket) -> ZmqSocketReply {
        return ZmqSocketReply { socket: socket }
    }

    pub fn send_recv_message(&self, msg: Message, header_msg: &str) -> Message {
        // send message and recieve response
        let msg_str = serde_json::to_string(&msg).unwrap();
        self.socket.send_multipart([header_msg, &msg_str], 0).unwrap();
        let mut recv_multi = self.socket.recv_multipart(0).unwrap();
        let response: Vec<u8> = recv_multi.pop().unwrap();
        let rep_msg: Message = serde_json::from_str(&String::from_utf8(response).unwrap()).unwrap();
        return rep_msg;
    }
}

pub fn zmq_init () -> ZmqSocketReply {
    // This app is running as request/response, so start with a request to initialize a game
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::REQ).unwrap();
    socket.bind("tcp://127.0.0.1:1212").unwrap();
    
    return ZmqSocketReply::new(socket);
}


#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    pub fn print_all_messages() {
        let current_players: Vec<usize> = vec![1, 2, 3];
        let current_position: usize = 2;
        let min_bet: u32 = 100;
        let max_bet: u32 = 200;
        let shared_cards: Vec<u8> = vec![1, 7, 3, 50];
        let current_pot: u32 = 10000;
        let msg = Message::TurnMessage{ current_players, current_position, current_pot, min_bet, max_bet, shared_cards, turn: Turn::HOLE };

        let ser_msg = serde_json::to_string(&msg).unwrap();
        println!("{}", ser_msg);

        let players: Vec<String> = vec!["player_1".to_string(), "player_2".to_string()];
        let chips: Vec<u32> = vec![1000, 1000];
        let msg = Message::StartGameMessage{ players, chips };
        let ser_msg = serde_json::to_string(&msg).unwrap();
        println!("{}", ser_msg);
    }

    #[test]
    #[serial]
    pub fn test_zmq_init() {
        let msg = Message::ReadyForGameMessage;
        let header: &str = "test_header";
        let zmq_socket = zmq_init();
        let reply: Message = zmq_socket.send_recv_message(msg, header);
        println!("{:?}", reply);
    }

}
