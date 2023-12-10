use std::collections::VecDeque;

pub fn main() {
    let input = std::fs::read_to_string("input/day10").expect("No input");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[derive(Debug, PartialEq)]
struct MapNode {
    val: u8,
    dist: i32,
    interior: bool,
}

fn part_one(input: &str) -> i32 {
    let rows = input.lines().count();
    let columns = input.lines().nth(0).unwrap().len();
    let mut map = create_map(rows, columns, input);

    find_main_loop(&mut map, columns, rows);

    map.iter()
        .fold(0, |acc, map_node| acc.max(map_node.dist as i32))
}

fn find_main_loop(map: &mut Vec<MapNode>, columns: usize, rows: usize) {
    let start = map
        .iter()
        .position(|node| node.val == b'S')
        .expect("No valid start found");

    let mut pos_to_visit = VecDeque::new();
    pos_to_visit.push_back((start, 0));
    while let Some((this_pos, dist)) = pos_to_visit.pop_front() {
        if map[this_pos].dist > 0 {
            // This node has already been handled
            continue;
        }

        map[this_pos].dist = dist;
        let dist = dist + 1;

        if this_pos > columns {
            let top = this_pos - columns;
            match map[top].val {
                b'|' | b'7' | b'F' => pos_to_visit.push_back((top, dist)),
                _ => {}
            };
        }
        if this_pos < columns * (rows - 1) {
            let bottom = this_pos + columns;
            match map[bottom].val {
                b'|' | b'L' | b'J' => pos_to_visit.push_back((bottom, dist)),
                _ => {}
            }
        }
        if this_pos % columns > 0 {
            let left = this_pos - 1;
            match map[left].val {
                b'-' | b'F' | b'L' => pos_to_visit.push_back((left, dist)),
                _ => {}
            }
        }
        if this_pos % columns < columns {
            let right = this_pos + 1;
            match map[right].val {
                b'-' | b'J' | b'7' => pos_to_visit.push_back((right, dist)),
                _ => {}
            }
        }
    }
}

fn create_map(rows: usize, columns: usize, input: &str) -> Vec<MapNode> {
    let mut map: Vec<MapNode> = Vec::with_capacity(rows * columns);
    for line in input.lines() {
        for byte in line.bytes() {
            map.push(MapNode {
                val: byte,
                dist: 0,
                interior: false,
            });
        }
    }
    map
}

fn part_two(input: &str) -> i32 {
    let rows = input.lines().count();
    let columns = input.lines().nth(0).unwrap().len();
    let mut map = create_map(rows, columns, input);

    find_main_loop(&mut map, columns, rows);

    // Replace the S
    replace_start(&mut map, columns, rows);

    let mut interior_points = 0;
    for i in 0..columns {
        let mut interior = false;
        for j in 0..rows {
            let index = columns * i + j;
            if map[index].dist > 0 {
                match map[index].val {
                    b'-' | b'F' | b'L' | b'J' | b'7' => {
                        // Check if this is part of the main loop
                        if map[index].val > 0 {
                            println!("Crossing boundary at {i}, {j}");
                            interior = !interior;
                        }
                    }
                    _ => {}
                };
            } else if interior {
                println!("Adding {} at {i}, {j} as interior", map[index].val as char);
                interior_points += 1;
            }
        }
    }

    interior_points
}

fn replace_start(map: &mut [MapNode], columns: usize, rows: usize) {
    if let Some(i) = map.iter().position(|n| n.val == b'S') {
        let mut top = false;
        let mut bottom = false;
        let mut left = false;
        let mut right = false;
        if i > columns {
            top = matches!(map[i - columns].val, b'|' | b'7' | b'F');
        }
        if i < columns * (rows - 1) {
            bottom = matches!(map[i - columns].val, b'|' | b'J' | b'L');
        }
        if i % columns > 0 {
            left = matches!(map[i - columns].val, b'-' | b'F' | b'L');
        }
        if i % columns < columns {
            right = matches!(map[i - columns].val, b'-' | b'J' | b'7');
        }

        if top {
            if left {
                map[i].val = b'L';
            }
            if right {
                map[i].val = b'J';
            }
            if bottom {
                map[i].val = b'|';
            }
        }
        if left {
            if right {
                map[i].val = b'-';
            }
            if bottom {
                map[i].val = b'7';
            }
        }
        if bottom && right {
            map[i].val = b'F';
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAP1: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

    const MAP2: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const MAP3: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    const MAP4: &str = "...........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(MAP1), 4);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(MAP1), 1);
        assert_eq!(part_two(MAP2), 4);
        assert_eq!(part_two(MAP3), 10);
        assert_eq!(part_two(MAP4), 4);
    }
}
