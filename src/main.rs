use pokersim::pokersim::dealer;
use pokersim::pokersim::card_enums;


fn main() {
    let mut dealer = dealer::Dealer::new();
    // dealer.shuffle();
    let v = &dealer.deck;
    for card_int in v {
        println!("{}", card_enums::Card::card_from_int(card_int))
    }
}
