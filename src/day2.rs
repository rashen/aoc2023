pub fn main() {
    let input = std::fs::read_to_string("input/day2").expect("No input");
    println!("Part one: {}", part_one(&input));
}

#[derive(Debug, PartialEq)]
struct Game {
    id: i32,
    sets: Vec<CubeSet>,
}

#[derive(Debug, PartialEq)]
struct CubeSet {
    red: i32,
    green: i32,
    blue: i32,
}
impl CubeSet {
    pub fn new(r: i32, g: i32, b: i32) -> Self {
        Self {
            red: r,
            green: g,
            blue: b,
        }
    }
}

fn parse_input(input: &str) -> Vec<Game> {
    let mut output = vec![];
    for game in input.lines() {
        let split = game.split(':').collect::<Vec<&str>>();

        let game_id = split.first().unwrap().trim();
        let game_id = game_id
            .rsplit_once(' ')
            .map_or(0, |(_, digit)| digit.parse::<i32>().unwrap());

        let mut game = Game {
            id: game_id,
            sets: vec![],
        };

        let sets = split.last().unwrap().trim();
        for set in sets.split(';') {
            let set = set.trim();
            let mut cube_set = CubeSet::new(0, 0, 0);
            for val in set.split(',') {
                let val = val.trim();
                let digit = val.split(' ').nth(0);
                if let Some(digit) = digit.and_then(|s| s.parse::<i32>().ok()) {
                    if val.ends_with("red") {
                        cube_set.red = digit;
                    }
                    if val.ends_with("green") {
                        cube_set.green = digit;
                    }
                    if val.ends_with("blue") {
                        cube_set.blue = digit;
                    }
                }
            }
            game.sets.push(cube_set);
        }
        output.push(game);
    }
    output
}

fn part_one(input: &str) -> i32 {
    let input = parse_input(input);
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut acc = 0;
    'outer: for game in input {
        for set in game.sets {
            if set.red > max_red {
                continue 'outer;
            }
            if set.green > max_green {
                continue 'outer;
            }
            if set.blue > max_blue {
                continue 'outer;
            }
        }
        acc += game.id;
    }
    acc
}

fn part_two(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
   Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
   Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
   Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
   Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(INPUT).first().unwrap(),
            &Game {
                id: 1,
                sets: vec![
                    CubeSet::new(4, 0, 3),
                    CubeSet::new(1, 2, 6),
                    CubeSet::new(0, 2, 0)
                ]
            }
        );
        assert_eq!(
            parse_input(INPUT).get(2).unwrap(),
            &Game {
                id: 3,
                sets: vec![
                    CubeSet::new(20, 8, 6),
                    CubeSet::new(4, 13, 5),
                    CubeSet::new(1, 5, 0)
                ]
            }
        );
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 8);
    }
}
