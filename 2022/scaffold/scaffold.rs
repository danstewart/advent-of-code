use std::{env, fs};

// https://adventofcode.com/2022/day/<DAY>

// For first challenge: cargo run -- 1
// For second challenge: cargo run -- 2

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run -- <1 or 2>");
        return;
    }

    let part: i32 = args[1].parse().expect("Expected 1 or 2 as first argument");

    let contents =
        fs::read_to_string("data/input.txt").expect("Something went wrong reading the file");

    if part == 1 {
        // Do part 1
    } else if part == 2 {
        // Do part 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        assert_eq!(2 + 2, 4)
    }
}
