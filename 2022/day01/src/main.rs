use std::env;
use std::fs;
use std::collections::BTreeMap;

// https://adventofcode.com/2022/day/1

// For first challenge: cargo run -- 1
// For second challenge: cargo run -- 3

fn main() {
    let args: Vec<String> = env::args().collect();
    let count: i32 = args[1].parse().expect("Invalid integer");

    let contents = fs::read_to_string("data/input.txt")
        .expect("Something went wrong reading the file");

    // Keep track of which elve has the highest number of calories
    let mut idx = 0;
    let mut calories_to_elve = BTreeMap::new();

    // Read our file and build our BTreeMap
    let elves = contents.split("\n\n");
    for elve in elves {
        let mut total = 0;
        let foods = elve.split("\n");

        for food in foods {
            if food == "" {
                continue;
            }

            let calories: i32 = food.parse().unwrap();
            total += calories;
        }

        calories_to_elve.insert(
            total,
            idx,
        );

        idx += 1;
    }

    // Grab the top $TOP and total the calories
    let mut total_calories = 0;
    let mut iterable = calories_to_elve.iter().rev();
    for _ in 0..count {
        let item = iterable.next().unwrap();
        total_calories += item.0;
    }

    println!("Total calories for top {} elves: {}", count, total_calories);
}