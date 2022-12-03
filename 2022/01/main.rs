use std::fs;
use std::collections::BTreeMap;

// https://adventofcode.com/2022/day/1

// For first challenge set this to 1
// For second challenge set to 2
const TOP: i32 = 3;

fn main() {
    let contents = fs::read_to_string("input.txt")
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
    for _ in 0..TOP {
        let item = iterable.next().unwrap();
        total_calories += item.0;
    }

    println!("Total calories for top {} elves: {}", TOP, total_calories);
}
