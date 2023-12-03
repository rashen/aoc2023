use std::env;

mod day1;
mod day2;
mod day3;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Specify a day");
        return;
    }

    match args[1].as_str() {
        "1" => day1::main(),
        "2" => day2::main(),
        "3" => day3::main(),
        _ => {}
    };
}
