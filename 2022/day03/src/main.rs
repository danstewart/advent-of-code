use std::{collections::HashMap, env, fs};

// https://adventofcode.com/2022/day/3

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
        println!("{}", parse_rucksack_part1(contents));
    } else if part == 2 {
        println!("{}", parse_rucksack_part2(contents));
    }
}

fn build_score_map() -> HashMap<char, i32> {
    let mut score_map: HashMap<char, i32> = HashMap::new();

    let mut points = 1;
    for letter in b'a'..=b'z' {
        score_map.insert(letter as char, points);
        points += 1;
    }

    for letter in b'A'..=b'Z' {
        score_map.insert(letter as char, points);
        points += 1;
    }

    score_map
}

fn parse_rucksack_part1(input: String) -> i32 {
    let mut lines: Vec<&str> = input.split("\n").collect();
    if lines[lines.len()] == "" {
        lines.pop();
    }

    let score_map = build_score_map();

    let mut result: i32 = 0;
    for line in lines {
        let halfway = line.len() / 2;

        let left: String = line.chars().take(halfway).collect();
        let right: String = line.chars().skip(halfway).collect();

        let mut compartment: HashMap<char, bool> = HashMap::new();
        for item in left.chars() {
            compartment.insert(item, true);
        }

        for item in right.chars() {
            if compartment.contains_key(&item) {
                result += score_map.get(&item).unwrap();
                break;
            }
        }
    }

    result
}

fn parse_rucksack_part2(input: String) -> i32 {
    let mut lines: Vec<&str> = input.split("\n").collect();
    if lines[lines.len() - 1] == "" {
        lines.pop();
    }

    let score_map = build_score_map();

    let mut idx = 0;
    let mut result = 0;
    while idx < lines.len() {
        let first: &str = lines[idx];
        let second: &str = lines[idx + 1];
        let third: &str = lines[idx + 2];
        idx += 3;

        let mut first_items: HashMap<char, bool> = HashMap::new();
        for item in first.chars() {
            first_items.insert(item, true);
        }

        let mut second_items: HashMap<char, bool> = HashMap::new();
        for item in second.chars() {
            second_items.insert(item, true);
        }

        for item in third.chars() {
            if first_items.contains_key(&item) && second_items.contains_key(&item) {
                result += score_map.get(&item).unwrap();
                break;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_map() {
        let map = build_score_map();

        println!("{:?}", map);
        assert_eq!(map.get(&'a').unwrap(), &1);
        assert_eq!(map.get(&'z').unwrap(), &26);
        assert_eq!(map.get(&'A').unwrap(), &27);
        assert_eq!(map.get(&'Z').unwrap(), &52);
    }

    #[test]
    fn test_part1() {
        let input = concat!(
            "vJrwpWtwJgWrhcsFMMfFFhFp\n",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n",
            "PmmdzqPrVvPwwTWBwg\n",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n",
            "ttgJtRGJQctTZtZT\n",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        );

        let result = parse_rucksack_part1(input.into());

        assert_eq!(result, 157)
    }

    #[test]
    fn test_part2() {
        let input = concat!(
            "vJrwpWtwJgWrhcsFMMfFFhFp\n",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n",
            "PmmdzqPrVvPwwTWBwg\n",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n",
            "ttgJtRGJQctTZtZT\n",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        );

        let result = parse_rucksack_part2(input.into());

        assert_eq!(result, 70)
    }
}
