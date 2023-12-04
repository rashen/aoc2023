const COLS: usize = 140;
const RANK: usize = COLS * COLS;

pub fn main() {
    let input = std::fs::read_to_string("input/day3.txt").expect("No input");
    println!("Part one: {}", part_one(&input, COLS));
    println!("Part two: {}", part_two(&input, COLS));
}

fn part_one(input: &str, size: usize) -> u32 {
    let input_grid = to_grid(input);

    let mut acc = 0;
    for i in (0..size).rev() {
        let mut line_acc = 0;
        let mut consecutive_number = 0;
        let mut adjacent_symbol = false;
        for j in (0..size).rev() {
            let base: u32 = 10;
            if let Some(digit) = input_grid[i][j].to_digit(base) {
                line_acc += digit * base.pow(consecutive_number);
                consecutive_number += 1;
                adjacent_symbol |= has_adjacent_symbol(&input_grid, i, j);
                if j > 0 {
                    continue;
                }
            }
            if adjacent_symbol {
                acc += line_acc;
            }
            line_acc = 0;
            consecutive_number = 0;
            adjacent_symbol = false;
        }
    }
    acc
}

fn to_grid(input: &str) -> [[char; 140]; 140] {
    let mut input_grid = [['.'; COLS]; COLS];
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            input_grid[i][j] = c;
        }
    }
    input_grid
}

fn has_adjacent_symbol(grid: &[[char; COLS]; COLS], i: usize, j: usize) -> bool {
    let min_i = if i > 0 { i - 1 } else { i };
    let min_j = if j > 0 { j - 1 } else { j };
    let max_i = (i + 1).min(COLS - 1);
    let max_j = (j + 1).min(COLS - 1);
    for i in min_i..=max_i {
        for j in min_j..=max_j {
            if grid[i][j] != '.' && grid[i][j].is_numeric() == false {
                return true;
            }
        }
    }
    false
}

fn get_index(i: usize, j: usize) -> usize {
    (i * COLS) + j
}

fn part_two(input: &str, columns: usize) -> u64 {
    let mut input_grid = ['.'; RANK];
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            input_grid[get_index(i, j)] = c;
        }
    }

    let mut acc = 0;
    let rows = columns;
    for i in 0..rows {
        for j in 0..columns {
            let index = get_index(i, j);
            if input_grid[index] == '*' {
                acc += calc_adjacent_numbers(&input_grid, index);
            }
        }
    }

    acc
}

fn calc_adjacent_numbers(grid: &[char; RANK], index: usize) -> u64 {
    let index = index as i32;
    let max_size = COLS as i32;
    let positions = [
        index - max_size - 1, // top left
        index - max_size,     // above
        index - max_size + 1, // top right
        index - 1,            // left
        index + 1,            // right,
        index + max_size - 1, // bottom left
        index + max_size,     // below
        index + max_size + 1, // bottom right
    ];
    let mut numbers = [false; 8];

    let max_index = (COLS * COLS) as i32;
    for (j, &p) in positions.iter().enumerate() {
        if p > 0 && p < max_index {
            numbers[j] = grid[p as usize].is_numeric();
        } else {
            numbers[j] = false;
        }
    }

    let numbers = filter_adjacent_numbers(numbers);
    if numbers.iter().filter(|&b| *b).count() != 2 {
        return 0;
    }

    let mut acc = 1;
    for i in 0..8 {
        if numbers[i] {
            acc *= read_number(positions[i] as usize, grid) as u64;
        }
    }

    acc
}

fn filter_adjacent_numbers(numbers: [bool; 8]) -> [bool; 8] {
    let mut output = numbers.clone();
    if numbers[1] {
        output[0] = false;
        output[2] = false;
    }
    if numbers[6] {
        output[5] = false;
        output[7] = false;
    }
    output
}

fn read_number(i: usize, grid: &[char; COLS * COLS]) -> u32 {
    let left_boundary = (i / COLS) * COLS; // inclusive
    let right_boundary = ((i + 1) / COLS) * COLS;
    let right_boundary = right_boundary.min(COLS * COLS); // exclusive

    // Find last index
    let mut index: i32 = i as i32;
    while index > right_boundary as i32 {
        if grid[index as usize + 1].is_numeric() {
            index += 1;
        } else {
            break;
        }
    }

    let mut acc = 0;
    let mut num_consecutive = 0;
    while index >= left_boundary as i32 && grid[index as usize].is_numeric() {
        acc += grid[index as usize].to_digit(10).unwrap() * 10_u32.pow(num_consecutive);
        num_consecutive += 1;
        index -= 1;
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT, 10), 4361);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT, 10), 467835);
    }
}
