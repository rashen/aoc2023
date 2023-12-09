pub fn main() {
    let input = std::fs::read_to_string("input/day9").expect("No input");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &str) -> i32 {
    let mut history = vec![vec![]];
    for l in input.lines() {
        let mut this_line = vec![];
        for digit in l.split_whitespace() {
            this_line.push(digit.parse::<i32>().unwrap());
        }
        history.push(this_line);
    }

    let mut acc = 0;
    for h in history {
        acc += evaluate_line(&h);
    }

    acc
}

fn evaluate_line(vals: &[i32]) -> i32 {
    if vals.is_empty() {
        return 0;
    }

    if vals.iter().all(|&n| n == 0) {
        return 0;
    }

    let mut diffs = Vec::with_capacity(vals.len() - 1);
    for i in 1..vals.len() {
        diffs.push(vals[i] - vals[i - 1]);
    }

    vals.last().unwrap() + evaluate_line(&diffs)
}

fn evaluate_line_two_sided(vals: &[i32]) -> (i32, i32) {
    if vals.is_empty() {
        return (0, 0);
    }

    if vals.iter().all(|&n| n == 0) {
        return (0, 0);
    }

    let mut diffs = Vec::with_capacity(vals.len() - 1);
    for i in 1..vals.len() {
        diffs.push(vals[i] - vals[i - 1]);
    }

    let next_line = evaluate_line_two_sided(&diffs);
    let first_val = vals.first().unwrap() - next_line.0;
    let last_val = vals.last().unwrap() + next_line.1;

    (first_val, last_val)
}

fn part_two(input: &str) -> i32 {
    let mut history = vec![vec![]];
    for l in input.lines() {
        let mut this_line = vec![];
        for digit in l.split_whitespace() {
            this_line.push(digit.parse::<i32>().unwrap());
        }
        history.push(this_line);
    }

    let mut acc = 0;
    for h in history {
        let (left, _) = evaluate_line_two_sided(&h);
        acc += left;
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 114);
    }

    #[test]
    fn test_evaluate_line() {
        assert_eq!(evaluate_line(&[0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(evaluate_line(&[1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(evaluate_line(&[10, 13, 16, 21, 30, 45]), 68);
    }

    #[test]
    fn test_evaluate_line_two_sided() {
        assert_eq!(evaluate_line_two_sided(&[0, 3, 6, 9, 12, 15]), (-3, 18));
        assert_eq!(evaluate_line_two_sided(&[1, 3, 6, 10, 15, 21]), (0, 28));
        assert_eq!(evaluate_line_two_sided(&[10, 13, 16, 21, 30, 45]), (5, 68));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 2);
    }
}
