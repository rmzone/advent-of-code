use common::custom_error::AocError;
use itertools::Itertools;
use nom::character::complete::one_of;
use nom::multi::many1;
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, u32},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Clone, Copy)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 1, // Now are wild
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
    score: (u32, (u32, u32, u32, u32, u32)),
}

impl Hand {
    fn new(cards: Vec<char>, bid: u32) -> Hand {
        let mut cardss: Vec<Card> = vec![];
        for c in cards {
            let y: Card = match c {
                'A' => Card::Ace,
                'K' => Card::King,
                'Q' => Card::Queen,
                'J' => Card::Jack,
                'T' => Card::Ten,
                '9' => Card::Nine,
                '8' => Card::Eight,
                '7' => Card::Seven,
                '6' => Card::Six,
                '5' => Card::Five,
                '4' => Card::Four,
                '3' => Card::Three,
                '2' => Card::Two,
                _ => {
                    panic!()
                }
            };

            cardss.push(y);
        }

        assert_eq!(cardss.len(), 5);
        let scores = score_hand(&cardss);

        Hand {
            cards: cardss,
            bid,
            score: scores,
        }
    }
}

fn score_hand(cards: &Vec<Card>) -> (u32, (u32, u32, u32, u32, u32)) {
    use HandType::*;

    let counts = cards.iter().counts();
    let values = counts.values().sorted().join("");
    let mut hand_type = match values.deref() {
        "5" => FiveOfAKind,
        "14" => FourOfAKind,
        "23" => FullHouse,
        "113" => ThreeOfAKind,
        "122" => TwoPair,
        "1112" => OnePair,
        "11111" => HighCard,
        value => panic!("should never happen. Encountered `{}`", value),
    };

    // see if there are any Jokers, so this will upgrade the HandType
    let j_count = cards.iter().filter(|card| **card == Card::Jack).count();

    // brute force all the special cases. check for a better method
    if j_count == 1 {
        if hand_type == HighCard {
            hand_type = OnePair;
        } else if hand_type == OnePair {
            hand_type = ThreeOfAKind;
        } else if hand_type == TwoPair {
            hand_type = FullHouse;
        } else if hand_type == ThreeOfAKind {
            hand_type = FourOfAKind;
        } else if hand_type == FourOfAKind {
            hand_type = FiveOfAKind;
        }
    } else if j_count == 2 {
        if hand_type == OnePair {
            hand_type = ThreeOfAKind;
        } else if hand_type == TwoPair {
            hand_type = FourOfAKind;
        } else if hand_type == FullHouse {
            hand_type = FiveOfAKind;
        }
    } else if j_count == 3 {
        if hand_type == ThreeOfAKind {
            hand_type = FourOfAKind;
        } else if hand_type == FullHouse {
            hand_type = FiveOfAKind;
        }
    } else if j_count == 4 && hand_type == FourOfAKind {
        hand_type = FiveOfAKind;
    }

    let card_scores = cards
        .iter()
        .map(|card| *card as u32)
        .collect_tuple()
        .unwrap();
    (hand_type as u32, card_scores)
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, hand) = separated_pair(many1(one_of("AKQJT123456789")), tag(" "), u32)(input)?;

    Ok((input, Hand::new(hand.0, hand.1)))
}

fn parse_hands(input: &str) -> IResult<&str, Vec<Hand>> {
    let (input, hands) = separated_list1(line_ending, parse_hand)(input)?;
    Ok((input, hands))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, hands) = parse_hands(input).unwrap();
    let result = hands
        .iter()
        .sorted_by_key(|u| u.score)
        .enumerate()
        .map(|(index, hand)| (index as u32 + 1) * hand.bid)
        .sum::<u32>();

    Ok(result.to_string())
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
        assert_eq!("5905", process(input)?);
        Ok(())
    }
}
