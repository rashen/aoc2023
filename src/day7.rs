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
    Joker,
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

const NUM_CARDS: usize = 5;
type Cards = [CardLabel; NUM_CARDS];

#[derive(Debug)]
struct Hand {
    cards: Cards,
    bid: i32,
    ty: HandType,
}

fn parse_line(line: &str) -> Option<(Cards, i32)> {
    let (cards, bid) = split_once(line, ' ');
    let bid = bid.parse::<i32>().ok()?;
    let mut card_hand = [CardLabel::Two; NUM_CARDS];
    for (i, c) in cards.chars().enumerate() {
        if let Some(digit) = c.to_digit(10) {
            assert!(digit > 1 && digit < 10);
            unsafe {
                card_hand[i] = std::mem::transmute(digit as u8 - 1);
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

fn get_hand_type(cards: Cards) -> HandType {
    let mut num_types = 1;
    let mut sorted_cards = cards.clone();
    sorted_cards.sort();
    for i in 1..cards.len() {
        if sorted_cards[i - 1] != sorted_cards[i] {
            num_types += 1;
        }
    }

    match num_types {
        1 => HandType::FiveOfAKind,
        2 => {
            let num_of_one_type = cards.iter().filter(|&c| c == &cards[0]).count();
            if num_of_one_type == 1 || num_of_one_type == 4 {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            }
        }
        3 => {
            let num_of_first_type = sorted_cards
                .iter()
                .filter(|&c| c == &sorted_cards[0])
                .count();
            let num_of_second_type = sorted_cards
                .iter()
                .filter(|&c| c == &sorted_cards[2])
                .count();
            let num_of_third_type = sorted_cards
                .iter()
                .filter(|&c| c == &sorted_cards[4])
                .count();
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
            data.push(Hand {
                cards,
                bid,
                ty: get_hand_type(cards),
            })
        }
    }

    sort_hands_by_rank(&mut data);
    calculate_score(data)
}

fn part_two(input: &str) -> u64 {
    let mut hands = Vec::with_capacity(input.lines().count() - 1);
    for l in input.lines() {
        if let Some((mut cards, bid)) = parse_line(l) {
            for i in 0..cards.len() {
                if cards[i] == CardLabel::Jack {
                    cards[i] = CardLabel::Joker;
                }
            }
            hands.push(Hand {
                cards,
                bid,
                ty: HandType::None,
            });
        }
    }

    // Get the highest type for each card
    for i in 0..hands.len() {
        if hands[i].cards.contains(&CardLabel::Joker) {
            unsafe {
                let max_index: u8 = std::mem::transmute(CardLabel::Ace);
                for j in 0..=max_index {
                    let joker_replacement: CardLabel = std::mem::transmute(j);
                    let mut hand = [CardLabel::Two; NUM_CARDS];
                    for k in 0..NUM_CARDS {
                        if hands[i].cards[k] == CardLabel::Joker {
                            hand[k] = joker_replacement;
                        } else {
                            hand[k] = hands[i].cards[k].into();
                        }
                    }

                    let ty = get_hand_type(hand);
                    hands[i].ty = hands[i].ty.max(ty);
                }
            }
        } else {
            // No joker present
            hands[i].ty = get_hand_type(hands[i].cards);
        }
    }

    sort_hands_by_rank(&mut hands);
    calculate_score(hands)
}

fn calculate_score(hands: Vec<Hand>) -> u64 {
    let mut acc: u64 = 0;
    for i in 0..hands.len() {
        let score = hands[i].bid as u64 * (i + 1) as u64;
        acc += score
    }
    acc
}

fn sort_hands_by_rank(hands: &mut Vec<Hand>) {
    hands.sort_by(|d1, d2| match d1.ty.cmp(&d2.ty) {
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
        assert_eq!(get_hand_type(card_hand), HandType::OnePair);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 6440);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 5905);
    }
}
