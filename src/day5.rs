use sscanf::sscanf;

pub fn main() {
    let input = std::fs::read_to_string("input/day5").expect("No input");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[derive(Debug)]
struct Map {
    dst: u32,
    src: u32,
    range: u32,
}
impl Map {
    fn from_str(input: &str) -> Option<Self> {
        if let Ok((dst, src, range)) = sscanf!(input, "{} {} {}", u32, u32, u32) {
            return Some(Map { dst, src, range });
        }
        None
    }
}

fn split_once<'a>(input: &'a str, pat: char) -> (&'a str, &'a str) {
    let mid = input.find(pat).unwrap_or(input.len());
    let (head, tail) = input.split_at(mid);
    (&head[..mid], &tail[1..])
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<Vec<Map>>) {
    let mut lines = input.lines();
    let (_, tail) = split_once(lines.next().unwrap(), ':');
    let seeds = tail
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut output = vec![vec![]];
    while let Some(l) = lines.next() {
        if l.ends_with("map:") {
            let mut maps = vec![];
            while let Some(map) = lines.next().and_then(|l| Map::from_str(l)) {
                maps.push(map);
            }
            output.push(maps);
        }
    }
    (seeds, output)
}

fn part_one(input: &str) -> u64 {
    let (seeds, maps) = parse_input(input);
    let mut locations = vec![];
    for s in seeds {
        locations.push(propagate_seed(s, &maps));
    }
    locations.sort_by(|a, b| a.cmp(b));
    locations[0]
}

fn propagate_seed(seed: u32, maps: &[Vec<Map>]) -> u64 {
    let mut location = seed as u64;
    'layers: for layer in maps.iter() {
        for map in layer.iter() {
            let lower_bound = map.src as u64;
            let upper_bound = lower_bound + map.range as u64;
            if location >= lower_bound && location < upper_bound {
                let offset = location - lower_bound;
                location = map.dst as u64 + offset;
                continue 'layers;
            }
        }
    }
    location
}

fn part_two(input: &str) -> u64 {
    let (seeds, maps) = parse_input(input);
    let mut seed_ranges = vec![];
    for (i, start) in seeds.iter().enumerate().step_by(2) {
        seed_ranges.push((*start, seeds[i + 1]));
    }

    for i in 0..u32::MAX {
        let mut location = i as u64;
        'layers: for layer in maps.iter().rev() {
            for map in layer.iter().rev() {
                let lower_bound = map.dst as u64;
                let upper_bound = lower_bound + map.range as u64;
                if location >= lower_bound && location < upper_bound {
                    let offset = location - lower_bound;
                    location = map.src as u64 + offset;
                    continue 'layers;
                }
            }
        }
        // Compare to seed ranges
        for (start, range) in seed_ranges.iter() {
            let start = *start as u64;
            let end = start + *range as u64;
            if location >= start as u64 && location < end {
                return i as u64;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 35);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 46);
    }
}
