fn main() {
    let input = std::fs::read_to_string("src/input").expect("No input");
    println!("First puzzle: {}", first_puzzle(&input));
    println!("Second puzzle: {}", second_puzzle(&input));
}

fn first_puzzle(input: &str) -> u32 {
    let mut acc = 0;
    let base = 10;
    for line in input.lines().into_iter() {
        let digits = line
            .chars()
            .filter_map(|c| c.to_digit(base))
            .collect::<Vec<u32>>();
        if digits.len() > 0 {
            acc += digits.first().unwrap() * 10 + digits.last().unwrap();
        }
    }
    acc
}

fn second_puzzle(input: &str) -> u32 {
    let mut acc = 0;
    for line in input.lines() {
        let found_numbers = parse_single_line(line);
        acc += found_numbers.first().unwrap() * 10 + found_numbers.last().unwrap();
    }
    acc
}

fn parse_single_line(line: &str) -> Vec<u32> {
    let numbers_as_string = [
        "one", "two", "six", "nine", "four", "five", "seven", "eight", "three",
    ];
    let numbers_as_digits = [1, 2, 6, 9, 4, 5, 7, 8, 3];
    let mut found_numbers = vec![];
    let mut i = 0;
    'outer: while i < line.len() {
        if let Some(c) = line.chars().nth(i) {
            let base = 10;
            if let Some(digit) = c.to_digit(base) {
                found_numbers.push(digit);
                i += 1;
                continue 'outer;
            }
        }
        // Sliding window
        let window_size = 5.min(line.len() - i);
        let last_index = i + window_size;
        if let Some(window) = line.get(i..last_index) {
            for (index, n) in numbers_as_string.iter().enumerate() {
                if window.contains(n) {
                    found_numbers.push(numbers_as_digits[index]);
                    i += 2; // Taking two steps ensure that we don't register same digit twice, but can still catch overlapping digits
                    continue 'outer;
                }
            }
        }
        i += 1;
    }
    found_numbers.dedup();
    found_numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIRST_INPUT: &str = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";

    const SECOND_INPUT: &str = "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen
    eighthree
    sevenine
    oneight";

    #[test]
    fn test_first_input() {
        assert_eq!(first_puzzle(FIRST_INPUT), 142);
    }

    #[test]
    fn test_second_input() {
        assert_eq!(parse_single_line("xtwone3four"), vec![2, 1, 3, 4]);
        assert_eq!(parse_single_line("4nineeightseven2"), vec![4, 9, 8, 7, 2]);
        assert_eq!(parse_single_line("oneight"), vec![1, 8]);
        assert_eq!(parse_single_line("eighthree"), vec![8, 3]);
        assert_eq!(parse_single_line("zoneight234"), vec![1, 8, 2, 3, 4]);
        assert_eq!(
            parse_single_line("three2fiveonexrllxsvfive"),
            vec![3, 2, 5, 1, 5]
        );
        assert_eq!(second_puzzle(SECOND_INPUT), 281 + 83 + 79 + 18);
    }
}
