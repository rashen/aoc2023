use std::collections::VecDeque;

pub fn main() {
    let input = std::fs::read_to_string("input/day10").expect("No input");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[derive(Debug, PartialEq)]
enum MapNode {
    Instruction(u8),
    Distance(u32),
}

fn part_one(input: &str) -> u32 {
    let rows = input.lines().count();
    let columns = input.lines().nth(0).unwrap().len();
    let mut map: Vec<MapNode> = Vec::with_capacity(rows * columns);

    for line in input.lines() {
        for byte in line.bytes() {
            map.push(MapNode::Instruction(byte));
        }
    }

    let start = map
        .iter()
        .position(|c| c == &MapNode::Instruction(b'S'))
        .expect("No valid start found");

    let mut pos_to_visit = VecDeque::new();
    pos_to_visit.push_back((start, 0));
    while let Some((this_pos, dist)) = pos_to_visit.pop_front() {
        map[this_pos] = MapNode::Distance(dist);
        let dist = dist + 1;

        if this_pos > columns {
            let top = this_pos - columns;
            match map[top] {
                MapNode::Instruction(b'|' | b'7' | b'F') => pos_to_visit.push_back((top, dist)),
                _ => {}
            };
        }
        if this_pos < columns * (rows - 1) {
            let bottom = this_pos + columns;
            match map[bottom] {
                MapNode::Instruction(b'|' | b'L' | b'J') => pos_to_visit.push_back((bottom, dist)),
                _ => {}
            }
        }
        if this_pos % columns > 0 {
            let left = this_pos - 1;
            match map[left] {
                MapNode::Instruction(b'-' | b'F' | b'L') => pos_to_visit.push_back((left, dist)),
                _ => {}
            }
        }
        if this_pos % columns < columns {
            let right = this_pos + 1;
            match map[right] {
                MapNode::Instruction(b'-' | b'J' | b'7') => pos_to_visit.push_back((right, dist)),
                _ => {}
            }
        }
    }

    map.iter().fold(0, |acc, map_node| {
        if let MapNode::Distance(dist) = map_node {
            return acc.max(*dist);
        }
        acc
    })
}

fn part_two(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAP1: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(MAP1), 4);
    }
}
