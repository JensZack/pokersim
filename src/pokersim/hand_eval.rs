
// hand ranking
// royal flush - only 4 combos (4 masks): 80000
// straight flush - HC tie break (36 masks): 70000
// 4 OAK - HC of 4 TB (count each card type): 60000
// full house - HC of 3 TB (count each card type): 50000 + HC * 100 + LC
// flush - HC TB (count each suit type): 40000
// straight - HC TB (count each card type): 30000
// 3 OAK (count each card type): 20000
// 2 OAK (count each card type): 10000 (2 2oak doesn't beat 3oak)
// high card 100-1300
//
// Process ideas:
// masks for each type of hand? masks need to have an iter ability

pub fn score_hand(player_cards: &[u8; 2], shared_cards: &[u8; 5]) -> f64 {
    let mut all_cards: [u8; 7] = [0; 7]; 
    let (one, two) = all_cards.split_at_mut(2);
    one.copy_from_slice(player_cards);
    two.copy_from_slice(shared_cards);

    let mut suit_count: [u32; 4] = [0; 4];
    let mut card_count: [u32; 13] = [0; 13];
    for card in all_cards.iter() {
        let val: usize = (((card - 1) % 13)).try_into().unwrap(); 
        let suit: usize = (((card - 1) / 13)).try_into().unwrap(); 

        card_count[val] += 1;
        suit_count[suit] += 1;
    };


    // flush and straight check
    let is_flush: bool = *suit_count.iter().max().unwrap() >= 5;
    let mut is_straight: bool = false;

    let mut counter = 0;
    let mut straight_high_card: f64 = 0.;
    for i in 0..13 {
        if card_count[i] >= 1 {
            counter += 1;
            if counter >= 5 {
                is_straight = true;
                straight_high_card = (i + 1) as f64;
            }
        }
        else {
            counter = 0;
        };
    };

    if is_straight & is_flush {
        // mask checks for straight and royal flush
        // return if either mask check passes
        let mut one_hot_cards: [u32; 52] = [0; 52];
        for card in all_cards {one_hot_cards[(card as usize) - 1] = 1;}
        for card_num in (5..14).rev() {
            for suit in 0..4 {
                let end_idx: usize = card_num + suit * 13;
                let mask_sum: u32 = one_hot_cards[end_idx-5..end_idx].iter().sum::<u32>();

                if mask_sum == 5 {
                    if card_num == 13 {
                        // royal flush
                        return 80000.
                    }
                    else {
                        let hc = card_num as f64;
                        return 70000. + hc * 100.;
                    }
                }
            }
        }
    };

    if is_flush {
        // find hc
        let mut one_hot_cards: [u32; 52] = [0; 52];
        for card in all_cards {one_hot_cards[(card - 1) as usize] = 1;}
        for suit in 0..4 {
            let start_idx: usize = (suit) * 13;
            let masked_cards: [u32; 13] = one_hot_cards[start_idx..start_idx+13].try_into().unwrap();
            
            // if flush set hc
            if masked_cards.iter().sum::<u32>() >= 5 {
                for card_num in (5..13).rev() {
                    if masked_cards[card_num] == 1 { 
                        let hc: f64 = card_num as f64;
                        return 40000. + hc * 100.;
                    }
                }
            }
        }
    };
    
    if is_straight {
        return 30000. + straight_high_card * 100.;
    }

    let mut oak4: bool = false;
    let mut fh: bool = false;
    let mut oak3: bool = false;
    let mut oak2: bool = false;
    let mut oak22: bool = false;
    let mut hc: f64 = 0.;
    let mut lc: f64 = 0.; // lc only for full house or 2 doubles
    let mut abshc: f64 = 0.;

    for i in (0..13).rev() {
        if card_count[i] == 4 {
            oak4 = true;
            hc = i as f64;
            break
        }
        if card_count[i] == 3 {
            if oak2 {
                // already found 2 oak -> full house
                lc = hc;
                hc = i as f64;
                fh = true;
                break;
            }
            else if oak3 {
                lc = i as f64;
                fh = true;
                break
            }
            else {
                oak3 = true;
                hc = i as f64;
            }
        }
        if card_count[i] == 2 {
            if oak3 {
                lc = i as f64;
                fh = true;
                break;
            }
            else if oak2 & !oak22 {
                lc = i as f64;
                oak22 = true;
            }
            else {
                oak2 = true;
                hc = i as f64;
            }
        }
        if (abshc == 0.) & (card_count[i] >= 1) {
            abshc = i as f64;
        }
    };

    if oak4 {
        return 60000. + hc * 100. + abshc * 0.01;
    }

    if fh {
        return 50000. + hc * 100. + lc;
    }

    if oak3 {
        return 20000. + hc * 100. + abshc * 0.01;
    }

    if oak2 {
        return 10000. + hc * 100. + lc + abshc * 0.01;
    }

    return abshc * 0.01;
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pokersim::card_enums::Card;

    enum Result {
        P1WINS,
        P2WINS,
        TIE,
    }

    fn run_hand_comp_test(shared_cards: &[u8; 5], player_cards_1: &[u8; 2], player_cards_2: &[u8; 2], result: Result){
        let score_1 = score_hand(player_cards_1, shared_cards);
        let score_2 = score_hand(player_cards_2, shared_cards);
        println!("Score 1: {}, Score 2: {}", score_1, score_2);
        match result {
            Result::P1WINS => assert!(score_1 > score_2),
            Result::P2WINS => assert!(score_1 < score_2),
            Result::TIE => assert!(score_1 == score_2),
        };
    }

    fn test_cards(shared_cards: &[Card; 5], player_cards_1: &[Card; 2], player_cards_2: &[Card; 2], result: Result) {
        let shared_cards_int: [u8; 5] = shared_cards.iter().map(|x| x.to_int()).collect::<Vec<u8>>().try_into().unwrap();
        let p1_int: [u8; 2] = player_cards_1.iter().map(|x| x.to_int()).collect::<Vec<u8>>().try_into().unwrap();
        let p2_int: [u8; 2] = player_cards_2.iter().map(|x| x.to_int()).collect::<Vec<u8>>().try_into().unwrap();

        run_hand_comp_test(&shared_cards_int, &p1_int, &p2_int, result);
    }

    #[test]
    pub fn round_1 () {
        // flush vs straight
        let shared_cards: [Card; 5] = [
            Card::card_from_name("spade", "two"),
            Card::card_from_name("spade", "three"),
            Card::card_from_name("heart", "four"),
            Card::card_from_name("spade", "king"),
            Card::card_from_name("club", "king"),
        ];

        // has flush
        let player_cards_1: [Card; 2] = [
            Card::card_from_name("spade", "five"),
            Card::card_from_name("spade", "seven"),
        ];

        // has straight
        let player_cards_2: [Card; 2] = [
            Card::card_from_name("club", "five"),
            Card::card_from_name("heart", "six"),
        ];

        test_cards(&shared_cards, &player_cards_1, &player_cards_2, Result::P1WINS);
    }

    #[test]
    pub fn round_2 () {
        // straight vs straight
        let shared_cards: [Card; 5] = [
            Card::card_from_name("spade", "two"),
            Card::card_from_name("spade", "five"),
            Card::card_from_name("heart", "four"),
            Card::card_from_name("spade", "eight"),
            Card::card_from_name("club", "king"),
        ];

        // has flush
        let player_cards_1: [Card; 2] = [
            Card::card_from_name("club", "six"),
            Card::card_from_name("spade", "seven"),
        ];

        // has straight
        let player_cards_2: [Card; 2] = [
            Card::card_from_name("spade", "three"),
            Card::card_from_name("heart", "six"),
        ];

        test_cards(&shared_cards, &player_cards_1, &player_cards_2, Result::P1WINS);
    }

    #[test]
    pub fn round_3 () {
        // flush vs flush
        let shared_cards: [Card; 5] = [
            Card::card_from_name("spade", "two"),
            Card::card_from_name("spade", "five"),
            Card::card_from_name("spade", "jack"),
            Card::card_from_name("club", "king"),
            Card::card_from_name("spade", "seven"),
        ];

        // has flush wins w/HC
        let player_cards_1: [Card; 2] = [
            Card::card_from_name("heart", "ace"),
            Card::card_from_name("spade", "eight"),
        ];

        // has flush, lower HC
        let player_cards_2: [Card; 2] = [
            Card::card_from_name("spade", "three"),
            Card::card_from_name("spade", "queen"),
        ];
        
        // has flush, lower HC
        let player_cards_3: [Card; 2] = [
            Card::card_from_name("spade", "four"),
            Card::card_from_name("heart", "queen"),
        ];

        test_cards(&shared_cards, &player_cards_1, &player_cards_2, Result::P2WINS);
        test_cards(&shared_cards, &player_cards_1, &player_cards_3, Result::TIE);
    }

    #[test]
    pub fn round_4 () {
        // royal, straight flush, flush

        let shared_cards: [Card; 5] = [
            Card::card_from_name("club", "queen"),
            Card::card_from_name("club", "ten"),
            Card::card_from_name("club", "jack"),
            Card::card_from_name("heart", "king"),
            Card::card_from_name("spade", "seven"),
        ];

        // royal flush
        let player_cards_1: [Card; 2] = [
            Card::card_from_name("club", "ace"),
            Card::card_from_name("club", "king"),
        ];

        // straight flush
        let player_cards_2: [Card; 2] = [
            Card::card_from_name("club", "nine"),
            Card::card_from_name("club", "eight"),
        ];

        // flush
        let player_cards_3: [Card; 2] = [
            Card::card_from_name("club", "two"),
            Card::card_from_name("club", "four"),
        ];

        test_cards(&shared_cards, &player_cards_1, &player_cards_2, Result::P1WINS);
        test_cards(&shared_cards, &player_cards_2, &player_cards_3, Result::P1WINS);
    }

    #[test]
    pub fn round_5 () {
        // compare straight flush HC

        let shared_cards: [Card; 5] = [
            Card::card_from_name("diamond", "eight"),
            Card::card_from_name("diamond", "ten"),
            Card::card_from_name("diamond", "nine"),
            Card::card_from_name("heart", "king"),
            Card::card_from_name("spade", "seven"),
        ];

        let player_cards_1: [Card; 2] = [
            Card::card_from_name("diamond", "queen"),
            Card::card_from_name("diamond", "jack"),
        ];

        let player_cards_2: [Card; 2] = [
            Card::card_from_name("diamond", "seven"),
            Card::card_from_name("diamond", "six"),
        ];

        test_cards(&shared_cards, &player_cards_1, &player_cards_2, Result::P1WINS);
    }

    #[test]
    pub fn round_6 () {
        // some pair comparisons

        let shared_cards: [Card; 5] = [
            Card::card_from_name("diamond", "two"),
            Card::card_from_name("club", "three"),
            Card::card_from_name("club", "four"),
            Card::card_from_name("heart", "king"),
            Card::card_from_name("spade", "ace"),
        ];

        // one pair twos
        let player_cards_1: [Card; 2] = [
            Card::card_from_name("diamond", "ten"),
            Card::card_from_name("club", "two"),
        ];

        // one pair fours
        let player_cards_2: [Card; 2] = [
            Card::card_from_name("diamond", "ten"),
            Card::card_from_name("diamond", "four"),
        ];

        // one pair aces
        let player_cards_3: [Card; 2] = [
            Card::card_from_name("diamond", "ace"),
            Card::card_from_name("diamond", "six"),
        ];

        // three of a kind threes
        let player_cards_4: [Card; 2] = [
            Card::card_from_name("diamond", "three"),
            Card::card_from_name("heart", "three"),
        ];
        
        // only hc
        let player_cards_5: [Card; 2] = [
            Card::card_from_name("diamond", "seven"),
            Card::card_from_name("heart", "six"),
        ];

        test_cards(&shared_cards, &player_cards_1, &player_cards_2, Result::P2WINS);
        test_cards(&shared_cards, &player_cards_2, &player_cards_3, Result::P2WINS);
        test_cards(&shared_cards, &player_cards_3, &player_cards_4, Result::P2WINS);
        test_cards(&shared_cards, &player_cards_1, &player_cards_5, Result::P1WINS);
    }
}
