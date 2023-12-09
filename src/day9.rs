pub fn main() {
    let input = std::fs::read_to_string("input/day9").expect("No input");
    println!("Part one: {}", part_one(&input));
    // println!("Part two: {}", part_two(&input));
}

fn part_one(input: &str) -> i32 {
    0
}

fn evaluate_line(vals: &[i32]) -> i32 {
    if vals.is_empty() {
        return 0;
    }

    if vals.iter().all(|&n| n == 0) {
        return 0;
    }

    let mut diffs = Vec::with_capacity(vals.len() - 1);
    for i in 1..vals.len() - 1 {
        diffs.push(vals[i] - vals[i - 1]);
    }

    let last_val = vals.last().unwrap() + evaluate_line(&diffs);
    println!("{:?}, {}", diffs, last_val);
    last_val
}

fn part_two(input: &str) -> i32 {
    0
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
}
