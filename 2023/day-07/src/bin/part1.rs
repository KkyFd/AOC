use day_07::Hand;
use itertools::Itertools;
#[allow(dead_code)]
fn camel_card_winnings(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let mut separated = line.split_whitespace();
            let hand = TryInto::<[u8; 5]>::try_into(separated.next().unwrap().as_bytes())
                .unwrap()
                .map(|c| match c {
                    b'2'..=b'9' => c - b'0',
                    b'T' => 10,
                    b'J' => 11,
                    b'Q' => 12,
                    b'K' => 13,
                    b'A' => 14,
                    _ => unreachable!(),
                });
            let bid = separated.next().unwrap().parse::<u64>().unwrap();
            let handtype = Hand::hand_type(&hand);
            Hand::new(hand, handtype, bid)
        }) //don't care
        .sorted_unstable_by(|a, b| {
            a.handtype.cmp(&b.handtype).then(
                a.hand[0].cmp(&b.hand[0]).then(
                    a.hand[1].cmp(&b.hand[1]).then(
                        a.hand[2]
                            .cmp(&b.hand[2])
                            .then(a.hand[3].cmp(&b.hand[3]))
                            .then(a.hand[4].cmp(&b.hand[4])),
                    ),
                ),
            )
        })
        .enumerate()
        .map(|(i, hand)| (i as u64 + 1) * hand.bid)
        .sum()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yeah() {
        let test = camel_card_winnings(
            "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483",
        );
        assert_eq!(test, 6440);
    }
}
