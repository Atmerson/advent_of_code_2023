use std::ops::Deref;

use itertools::Itertools;

use crate::custom_error::AocError;

#[derive(Debug, Clone, Copy)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

fn score_hand(
    hand: &str,
) -> (HandType, (u32, u32, u32, u32, u32)) {
    use HandType::*;

    let counts = hand.chars().counts();
    let values = counts.values().sorted().join("");
    let hand_type = match values.deref() {
        "5" => FiveOfAKind,
        "14" => FourOfAKind,
        "23" => FullHouse,
        "113" => ThreeOfAKind,
        "122" => TwoPair,
        "1112" => OnePair,
        "11111" => HighCard,
        value => panic!(
            "oh jeez encountered `{}`",
            value
        ),
    };
    let card_scores = hand
        .chars()
        .map(|card| match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            value => value.to_digit(10).unwrap(),
        })
        .collect_tuple()
        .unwrap();
    (hand_type, card_scores)
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let hands = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(" ").unwrap();
            (
                hand,
                bid.parse::<u32>().unwrap(),
                score_hand(hand),
            )
        })
        .sorted_by_key(|x| (x.2 .0 as u8, x.2 .1))
        .enumerate()
        .map(|(index, (_hand, bid, _))| {
            (index as u32 + 1) * bid
        })
        .sum::<u32>();
    Ok(hands.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!("6440", process(input)?);
        Ok(())
    }
}