pub fn main() {
    let input = std::fs::read_to_string("input/day4.txt").expect("No input");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &str) -> i32 {
    let mut acc: i32 = 0;

    for line in input.lines() {
        let split = line.split(":");
        let cards = split.last().unwrap();
        let mut split = cards.split("|");
        let winning_cards = split.nth(0).unwrap().split_whitespace();
        let elfs_cards = split.last().unwrap().split_whitespace();

        let winning = winning_cards
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let elfs_cards = elfs_cards
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut line_acc = 0;
        for c in elfs_cards.iter() {
            if winning.contains(c) {
                line_acc = 1.max(2 * line_acc);
            }
        }
        acc += line_acc;
    }
    acc
}

fn split_once<'a>(input: &'a str, pat: char) -> (&'a str, &'a str) {
    let mid = input.find(pat).unwrap_or(input.len());
    let (head, tail) = input.split_at(mid);
    (&head[..mid], &tail[1..])
}

fn part_two(input: &str) -> i32 {
    let num_cards = input.lines().count();
    let mut num_copies = vec![1; num_cards];

    for (i, line) in input.lines().enumerate() {
        let (_, tail) = split_once(line, ':');
        let (head, tail) = split_once(tail, '|');
        let winning = head
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let elfs_cards = tail
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let cards_won = winning.iter().filter(|c| elfs_cards.contains(c)).count();
        for j in 1..=cards_won {
            num_copies[i + j] += num_copies[i];
        }
    }
    num_copies.iter().fold(0, |acc, n| acc + *n as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 13);
    }

    #[test]
    fn test_split() {
        assert_eq!(split_once("Card 1 | numbers", '|'), ("Card 1 ", " numbers"));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 30);
    }
}
