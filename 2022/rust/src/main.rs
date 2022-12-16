mod days;
use std::{env, fmt::Display, fs};

// https://adventofcode.com/2022/day/<DAY>

// For first challenge: cargo run -- 1
// For second challenge: cargo run -- 2

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: cargo run -- <day number> <puzzle part>");
        return;
    }

    let day: i32 = args[1].parse().expect("Expected a day number");
    let part: i32 = args[2].parse().expect("Expected the puzzle part");
    let nice_day = format!("{:0>2}", day);
    let file_name = format!("data/day{}.txt", nice_day);

    let contents = fs::read_to_string(&file_name)
        .expect(format!("Something went wrong reading the file: {}", &file_name).as_str());

    let answer = match day {
        1 => days::day01::process(part, contents).to_string(),
        2 => days::day02::process(part, contents).to_string(),
        3 => days::day03::process(part, contents).to_string(),
        4 => days::day04::process(part, contents).to_string(),
        5 => days::day05::process(part, contents),
        6 => days::day06::process(part, contents).to_string(),
        _ => {
            println!("Invalid day: {}", day);
            "ERR".into()
        }
    };

    println!(
        "The answer for day {} part {} is: {}",
        nice_day, part, answer
    );
}
