use pokersim::cards::dealer::Dealer;
use pokersim::cards::card_enums::Card;


fn main() {
    let mut dealer = Dealer::new();
    dealer.shuffle();
    let v = &dealer.deck;
    for card_int in v {
        println!("{}", Card::card_from_int(card_int))
    }
}
