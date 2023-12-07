use std::cmp::Ordering;

use crate::parsing::*;

pub fn main() {
    let input = std::fs::read_to_string("input/day7").expect("No input");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[allow(dead_code)] // False positive
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum CardLabel {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum HandType {
    None,
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

type CardHand = [CardLabel; 5];

#[derive(Debug)]
struct HandData {
    cards: CardHand,
    bid: i32,
    ty: HandType,
}

fn parse_line(line: &str) -> Option<(CardHand, i32)> {
    let (cards, bid) = split_once(line, ' ');
    let bid = bid.parse::<i32>().ok()?;
    let mut card_hand = [CardLabel::Two; 5];
    for (i, c) in cards.chars().enumerate() {
        if let Some(digit) = c.to_digit(10) {
            assert!(digit > 1 && digit < 10);
            unsafe {
                card_hand[i] = std::mem::transmute(digit as u8 - 2);
            }
        } else {
            let label = match c {
                'T' => CardLabel::Ten,
                'J' => CardLabel::Jack,
                'Q' => CardLabel::Queen,
                'K' => CardLabel::King,
                'A' => CardLabel::Ace,
                _ => return None,
            };
            card_hand[i] = label;
        }
    }
    Some((card_hand, bid))
}

fn get_hand_type(hand: &CardHand) -> HandType {
    let mut num_types = 1;
    let mut sorted_hand = hand.clone();
    sorted_hand.sort();
    for i in 1..hand.len() {
        if sorted_hand[i - 1] != sorted_hand[i] {
            num_types += 1;
        }
    }

    match num_types {
        1 => HandType::FiveOfAKind,
        2 => {
            let num_of_one_type = hand.iter().filter(|&c| c == &hand[0]).count();
            if num_of_one_type == 1 || num_of_one_type == 4 {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            }
        }
        3 => {
            let num_of_first_type = sorted_hand.iter().filter(|&c| c == &sorted_hand[0]).count();
            let num_of_second_type = sorted_hand.iter().filter(|&c| c == &sorted_hand[2]).count();
            let num_of_third_type = sorted_hand.iter().filter(|&c| c == &sorted_hand[4]).count();
            if num_of_first_type
                .max(num_of_second_type)
                .max(num_of_third_type)
                == 3
            {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPair
            }
        }
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => HandType::None,
    }
}

fn part_one(input: &str) -> u64 {
    let mut data = Vec::with_capacity(input.lines().count() - 1);
    for l in input.lines() {
        if let Some((cards, bid)) = parse_line(l) {
            data.push(HandData {
                cards,
                bid,
                ty: get_hand_type(&cards),
            })
        }
    }

    data.sort_by(|d1, d2| match d1.ty.cmp(&d2.ty) {
        Ordering::Equal => {
            for i in 0..d1.cards.len() {
                match d1.cards[i].cmp(&d2.cards[i]) {
                    Ordering::Equal => continue,
                    o => return o,
                }
            }
            Ordering::Equal
        }
        o => o,
    });

    let mut acc: u64 = 0;
    for i in 0..data.len() {
        let score = data[i].bid as u64 * (i + 1) as u64;
        acc += score
    }
    acc
}

fn part_two(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_parse_line() {
        use CardLabel::*;
        assert_eq!(
            parse_line(INPUT.lines().nth(0).unwrap()),
            Some(([Three, Two, Ten, Three, King], 765))
        )
    }

    #[test]
    fn test_get_hand_type() {
        let (card_hand, _) = parse_line(INPUT.lines().nth(0).unwrap()).unwrap();
        assert_eq!(get_hand_type(&card_hand), HandType::OnePair);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 6440);
    }
}
