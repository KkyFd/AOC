use std::collections::HashMap;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hand {
    pub hand: [u8; 5],
    pub handtype: HandType,
    pub bid: u64,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum HandType {
    FiveofaKind = 6,
    FourofaKind = 5,
    FullHouse = 4,
    ThreeofaKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}
impl Hand {
    pub fn new(hand: [u8; 5], handtype: HandType, bid: u64) -> Self {
        Self {
            hand,
            handtype,
            bid,
        }
    }
    pub fn hand_type(input: &[u8]) -> HandType {
        let mut card_counts: HashMap<u8, usize> = HashMap::new();

        for card in input {
            card_counts
                .entry(*card)
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
        match [
            card_counts.values().filter(|t| **t == 1).count(),
            card_counts.values().filter(|t| **t == 2).count(),
            card_counts.values().filter(|t| **t == 3).count(),
            card_counts.values().filter(|t| **t == 4).count(),
            card_counts.values().filter(|t| **t == 5).count(),
        ] {
            [_, _, _, _, 1] => HandType::FiveofaKind,
            [_, _, _, 1, _] => HandType::FourofaKind,
            [_, 1, 1, _, _] => HandType::FullHouse,
            [2, _, 1, _, _] => HandType::ThreeofaKind,
            [_, 2, _, _, _] => HandType::TwoPair,
            [3, 1, _, _, _] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
    pub fn hand_type_jokers(input: &[u8]) -> HandType {
        let mut card_counts: HashMap<u8, usize> = HashMap::new();
        for card in input {
            card_counts
                .entry(*card)
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
        let jokers = card_counts.remove(&1).unwrap_or(0);
        //for that cheeky JJJJJ that i spent 1 hour+ trying to figure out what was wrong
        let max_joker = match card_counts.values().max() {
            Some(max) => max + jokers,
            None => return HandType::FiveofaKind,
        };
        let pairs = card_counts.values().filter(|t| **t == 2).count();
        let one_offs = card_counts.values().filter(|t| **t == 1).count();
        match [one_offs, pairs, max_joker] {
            [_, _, 5] => HandType::FiveofaKind,
            [_, _, 4] => HandType::FourofaKind,
            [0, _, 3] => HandType::FullHouse,
            [3, _, 3] | [2, _, 3] => HandType::ThreeofaKind,
            [1, 2, _] => HandType::TwoPair,
            [_, _, 2] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}
