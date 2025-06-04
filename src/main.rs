mod cards;


fn main() {
    let mut dealer = cards::dealer::Dealer::new();
    dealer.shuffle();
    let v = &dealer.deck;
    for card_int in v {
        println!("{}", cards::card_enums::Card::card_from_int(card_int))
    }
}
