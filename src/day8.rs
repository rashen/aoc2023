use core::fmt::Debug;
use sscanf::sscanf;
use std::{fmt::Display, ptr};

pub fn main() {
    let input = std::fs::read_to_string("input/day8").expect("No input");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[derive(Debug)]
struct Node {
    name: [u8; 3],
    left: *const Node,
    right: *const Node,
}
impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = std::str::from_utf8(&self.name).unwrap();
        write!(f, "{}", text)
    }
}

fn part_one(input: &str) -> usize {
    let instructions = input.lines().nth(0).unwrap().as_bytes();
    let map = build_map(input);

    // Follow the instructions
    let mut num_steps = 0;
    let start_pos = map.iter().position(|n| n.name == [b'A'; 3]).unwrap();
    let mut position = ptr::addr_of!(map[start_pos]);
    loop {
        unsafe {
            let index = num_steps % instructions.len();
            let this_node: Node = ptr::read(position);
            if this_node.name == [b'Z'; 3] {
                break;
            }
            num_steps += 1;
            match instructions[index] {
                b'L' => position = this_node.left,
                b'R' => position = this_node.right,
                _ => panic!("faulty instruction"),
            }
        }
    }

    num_steps
}

fn build_map(input: &str) -> Vec<Node> {
    // Fill map with locations
    let lines = input.lines();
    let mut map = Vec::with_capacity(lines.clone().count());
    for l in lines.clone() {
        if let Ok((name, _, _)) = sscanf!(l, "{} = ({}, {})", &str, &str, &str) {
            map.push(Node {
                name: name.get(0..3).unwrap().as_bytes().try_into().unwrap(),
                left: ptr::null(),
                right: ptr::null(),
            })
        }
    }

    // Fill map with addresses
    let mut i = 0;
    for l in lines.clone() {
        if let Ok((_, left, right)) = sscanf!(l, "{} = ({}, {})", &str, &str, &str) {
            let left: [u8; 3] = left.get(0..3).unwrap().as_bytes().try_into().unwrap();
            let right: [u8; 3] = right.get(0..3).unwrap().as_bytes().try_into().unwrap();

            let p_left = map.iter().position(|n| n.name == left).unwrap();
            let p_right = map.iter().position(|n| n.name == right).unwrap();
            map[i].left = ptr::addr_of!(map[p_left]);
            map[i].right = ptr::addr_of!(map[p_right]);

            i += 1;
        }
    }
    map
}

fn part_two(input: &str) -> u64 {
    let instructions = input.lines().nth(0).unwrap().as_bytes();
    let map = build_map(input);

    let mut positions = vec![];
    for node in map.iter() {
        if node.name[2] == b'A' {
            positions.push(ptr::addr_of!(*node));
        }
    }

    let mut num_steps = 0;
    let mut num_steps_to_finish = vec![];
    loop {
        let instruction_index = num_steps % instructions.len();

        unsafe {
            let mut j = 0;
            while j < positions.len() {
                if ptr::read(positions[j]).name[2] == b'Z' {
                    positions.swap_remove(j);
                    num_steps_to_finish.push(num_steps);
                } else {
                    j += 1;
                }
            }

            if positions.is_empty() {
                break;
            }

            for j in 0..positions.len() {
                let this_node: Node = ptr::read(positions[j]);

                match instructions[instruction_index] {
                    b'L' => positions[j] = this_node.left,
                    b'R' => positions[j] = this_node.right,
                    _ => panic!("faulty instruction"),
                }
            }
            num_steps += 1;
        }
    }

    let mut lcm = num_steps_to_finish[0] as u64;
    for i in 1..num_steps_to_finish.len() {
        lcm = find_lcm(lcm, num_steps_to_finish[i] as u64);
    }

    lcm
}

fn find_gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    find_gcd(b, a % b)
}

fn find_lcm(a: u64, b: u64) -> u64 {
    if a | b == 0 {
        return 0;
    }
    a * (b / find_gcd(a, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const INPUT2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const INPUT3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT1), 2);
        assert_eq!(part_one(INPUT2), 6);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(find_gcd(0, 0), 0);
        assert_eq!(find_gcd(10, 4), 2);
        assert_eq!(find_gcd(13, 2), 1);
        assert_eq!(find_gcd(24, 32), 8);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(find_lcm(0, 0), 0);
        assert_eq!(find_lcm(5, 2), 10);
        assert_eq!(find_lcm(2, 5), 10);
        assert_eq!(find_lcm(4, 6), 12);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT3), 6);
    }
}
