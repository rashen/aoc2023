use std::ops::Add;

pub fn main() {
    let input = std::fs::read_to_string("input/day21").expect("No input");
    println!("Part one: {}", part_one(&input, 64));
    // println!("Part two: {}", part_two(&input));
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Pos {
    r: i32,
    c: i32,
}
impl Pos {
    fn new(r: i32, c: i32) -> Self {
        Self { r, c }
    }
}
impl Add for Pos {
    type Output = Pos;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            c: self.c + rhs.c,
        }
    }
}

fn part_one(input: &str, steps: usize) -> i32 {
    let n_rows = input.lines().count();
    let n_cols = input.lines().nth(0).unwrap().chars().count();

    let mut start_pos = Pos { r: 0, c: 0 };
    let mut map: Vec<char> = Vec::with_capacity(input.chars().count());
    for (row, l) in input.lines().enumerate() {
        for (col, c) in l.chars().enumerate() {
            map.push(c);
            if c == 'S' {
                start_pos = Pos {
                    r: row as i32,
                    c: col as i32,
                };
            }
        }
    }

    let mut maps = [map.clone(), map.clone()];
    // Populate the starting pos of the first map
    maps[0][get_index(start_pos, n_cols)] = 'O';

    for i in 0..steps {
        let read_map_index = i % 2;
        let write_map_index = (i + 1) % 2;
        maps[write_map_index] = map.clone();
        for j in 0..map.len() {
            if maps[read_map_index][j] == 'O' {
                let pos = get_pos(j, n_cols);

                let neighbors = [
                    Pos::new(-1, 0),
                    Pos::new(1, 0),
                    Pos::new(0, -1),
                    Pos::new(0, 1),
                ];
                for k in 0..neighbors.len() {
                    let neighbor_pos = pos + neighbors[k];
                    if pos.r > 0
                        && pos.r < (n_rows as i32 - 1)
                        && pos.c > 0
                        && pos.c < (n_cols as i32 - 1)
                    {
                        let neighbor_index = get_index(neighbor_pos, n_cols);
                        if maps[write_map_index][neighbor_index] != '#' {
                            maps[write_map_index][neighbor_index] = 'O';
                        }
                    }
                }
            }
        }
    }

    let last_map = &maps[steps % 2];
    last_map.iter().filter(|&c| *c == 'O').count() as i32
}

fn get_index(pos: Pos, row_length: usize) -> usize {
    pos.r as usize * row_length + pos.c as usize
}

fn get_pos(i: usize, row_length: usize) -> Pos {
    Pos {
        r: (i / row_length) as i32,
        c: (i % row_length) as i32,
    }
}

fn part_two(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&INPUT, 6), 16);
    }

    #[test]
    fn test_get_index() {
        let index = get_index(Pos { r: 5, c: 5 }, 11);
        assert_eq!(index, 60);
        let pos = get_pos(index, 11);
        assert_eq!(pos, Pos { r: 5, c: 5 });
    }
}
