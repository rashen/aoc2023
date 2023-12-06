use crate::parsing::*;

pub fn main() {
    let input = std::fs::read_to_string("input/day6").expect("No input");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &str) -> u64 {
    let mut times = vec![];
    let mut distances = vec![];
    for l in input.lines() {
        let (head, tail) = split_once(l, ':');
        match head {
            "Time" => {
                times = tail
                    .split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect()
            }
            "Distance" => {
                distances = tail
                    .split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect()
            }
            _ => {}
        }
    }

    let mut acc = 1_u64;
    for i in 0..times.len() {
        let (min, max) = get_int_bounds(distances[i] as f64, times[i] as f64);
        acc *= (max as i32 - min as i32) as u64;
    }
    acc
}

fn get_int_bounds(distance: f64, time: f64) -> (u64, u64) {
    // Quadratic solution to x(T-x) = d where T is total time, d is min distance
    let half_time = time / 2.0;
    let min = (half_time) - ((half_time).powf(2.0) - distance).sqrt();
    let max = (half_time) + ((half_time).powf(2.0) - distance).sqrt();

    let mut min = min as u64;
    let min_distance = min * (time as u64 - min);
    if min_distance == distance as u64 {
        min += 1;
    }
    (min, max as u64)
}

fn part_two(input: &str) -> u64 {
    let mut time = 0;
    let mut distance = 0;
    for l in input.lines() {
        let (head, tail) = split_once(l, ':');
        match head {
            "Time" => {
                time = tail
                    .split_whitespace()
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap();
            }
            "Distance" => {
                distance = tail
                    .split_whitespace()
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap();
            }
            _ => {}
        }
    }
    let (min, max) = get_int_bounds(distance as f64, time as f64);
    return (max - min) as u64;
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 288);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 71503);
    }
}
