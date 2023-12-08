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

fn get_hand_type<T>(hand: [T; NUM_CARDS]) -> HandType
where
    T: PartialEq + PartialOrd + Ord + Clone,
{
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
            data.push(Hand {
                cards,
                bid,
                ty: get_hand_type(cards),
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

#[allow(dead_code)] // False positive
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum CardLabelWithJoker {
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
    Queen,
    King,
    Ace,
}
impl From<CardLabel> for CardLabelWithJoker {
    fn from(value: CardLabel) -> Self {
        if value == CardLabel::Jack {
            return CardLabelWithJoker::Joker;
        }
        unsafe {
            let as_int: u8 = std::mem::transmute(value);
            if value < CardLabel::Queen {
                return std::mem::transmute(as_int + 1);
            }
            std::mem::transmute(as_int)
        }
    }
}
impl Into<CardLabel> for CardLabelWithJoker {
    fn into(self) -> CardLabel {
        if self == CardLabelWithJoker::Joker {
            return CardLabel::Jack;
        }
        unsafe {
            let as_int: u8 = std::mem::transmute(self);
            if self < CardLabelWithJoker::Queen {
                return std::mem::transmute(as_int - 1);
            }
            std::mem::transmute(as_int)
        }
    }
}

struct HandWithJoker {
    cards: [CardLabelWithJoker; NUM_CARDS],
    bid: i32,
    highest_type: HandType,
}

fn part_two(input: &str) -> u64 {
    let mut hands = Vec::with_capacity(input.lines().count() - 1);
    for l in input.lines() {
        if let Some((cards, bid)) = parse_line(l) {
            let mut cards_with_joker = [CardLabelWithJoker::Joker; NUM_CARDS];
            for i in 0..cards.len() {
                cards_with_joker[i] = cards[i].into()
            }
            hands.push(HandWithJoker {
                cards: cards_with_joker,
                bid,
                highest_type: HandType::None,
            });
        }
    }

    // Get the highest numer for each card
    for i in 0..hands.len() {
        if hands[i].cards.contains(&CardLabelWithJoker::Joker) {
            unsafe {
                let max_index: u8 = std::mem::transmute(CardLabel::Ace);
                for j in 0..=max_index {
                    let joker_replacement: CardLabel = std::mem::transmute(j);
                    let mut hand = [CardLabel::Two; NUM_CARDS];
                    for k in 0..NUM_CARDS {
                        if hands[i].cards[k] == CardLabelWithJoker::Joker {
                            hand[k] = joker_replacement;
                        } else {
                            hand[k] = hands[i].cards[k].into();
                        }
                    }

                    let score = get_hand_type(hand);
                    hands[i].highest_type = hands[i].highest_type.max(score);
                }
            }
        } else {
            // No joker present
            hands[i].highest_type = get_hand_type(hands[i].cards);
        }
    }

    hands.sort_by(|d1, d2| match d1.highest_type.cmp(&d2.highest_type) {
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
    for i in 0..hands.len() {
        let score = hands[i].bid as u64 * (i + 1) as u64;
        acc += score
    }
    acc
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
    fn test_card_label_into_from() {
        unsafe {
            let max_val: u8 = std::mem::transmute(CardLabel::Ace);
            for i in 0..=max_val {
                let a: CardLabel = std::mem::transmute(i);
                let b: CardLabelWithJoker = a.into();
                let c: CardLabel = b.into();
                assert_eq!(a, c);
            }
        }
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 5905);
    }
}
