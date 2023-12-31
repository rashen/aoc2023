use std::env;

mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod parsing;

fn main() {
    let entry_points = [
        day1::main,
        day2::main,
        day3::main,
        day4::main,
        day5::main,
        day6::main,
        day7::main,
        day8::main,
        day9::main,
        day10::main,
    ];

    let args: Vec<String> = env::args().collect();
    let mut index = entry_points.len() - 1;

    if args.len() > 1 {
        if let Ok(day) = args[1].as_str().parse::<usize>() {
            index = day - 1;
        }
    } else {
        println!("No day specified, running latest");
    };

    entry_points[index]()
}
