use std::env;

mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "1" {
        day1::main();
    }
}
