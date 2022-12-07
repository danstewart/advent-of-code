use std::{collections::HashMap, env, fs};

// https://adventofcode.com/2022/day/2

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
        println!("{}", parse_guide_part1(contents));
    } else if part == 2 {
        println!("{}", parse_guide_part2(contents));
    }
}

fn parse_guide_part1(input: String) -> i32 {
    let lines = input.split("\n");

    // A = Rock, B = Paper, C = Scissors
    // X = Rock, Y = Paper, Z = Scissors
    // Rock = 1pts, Paper = 2pts, Scissors = 3pts
    let mut score = 0;

    let mut map: HashMap<&str, String> = HashMap::new();
    map.insert("A", "Rock".into());
    map.insert("B", "Paper".into());
    map.insert("C", "Scissors".into());
    map.insert("X", "Rock".into());
    map.insert("Y", "Paper".into());
    map.insert("Z", "Scissors".into());

    for line in lines {
        if line == "" {
            continue;
        }
        let parts: Vec<&str> = line.split(" ").collect();
        let opponent = map.get(parts[0]).unwrap();
        let player = map.get(parts[1]).unwrap();

        if player == "Rock" {
            score += 1;
        }
        if player == "Paper" {
            score += 2;
        }
        if player == "Scissors" {
            score += 3;
        }

        if player == "Rock" && opponent == "Scissors" {
            score += 6;
        }
        if player == "Paper" && opponent == "Rock" {
            score += 6;
        }
        if player == "Scissors" && opponent == "Paper" {
            score += 6;
        }
        if player == opponent {
            score += 3;
        }
    }

    score
}

fn parse_guide_part2(input: String) -> i32 {
    let lines = input.split("\n");

    // A = Rock, B = Paper, C = Scissors
    // X = Rock, Y = Paper, Z = Scissors
    // Rock = 1pts, Paper = 2pts, Scissors = 3pts
    let mut score = 0;

    let mut opponent_action: HashMap<&str, String> = HashMap::new();
    opponent_action.insert("A", "Rock".into());
    opponent_action.insert("B", "Paper".into());
    opponent_action.insert("C", "Scissors".into());

    let mut wanted_outcome: HashMap<&str, String> = HashMap::new();
    wanted_outcome.insert("X", "Lose".into());
    wanted_outcome.insert("Y", "Draw".into());
    wanted_outcome.insert("Z", "Win".into());

    for line in lines {
        if line == "" {
            continue;
        }

        let parts: Vec<&str> = line.split(" ").collect();
        let opponent = opponent_action.get(parts[0]).unwrap();
        let outcome = wanted_outcome.get(parts[1]).unwrap();

        let player: &str = match outcome.as_str() {
            "Lose" if opponent == "Paper" => "Rock",
            "Lose" if opponent == "Scissors" => "Paper",
            "Lose" if opponent == "Rock" => "Scissors",
            "Win" if opponent == "Scissors" => "Rock",
            "Win" if opponent == "Rock" => "Paper",
            "Win" if opponent == "Paper" => "Scissors",
            _ => opponent,
        };

        if player == "Rock" {
            score += 1;
        }
        if player == "Paper" {
            score += 2;
        }
        if player == "Scissors" {
            score += 3;
        }

        if player == "Rock" && opponent == "Scissors" {
            score += 6;
        }
        if player == "Paper" && opponent == "Rock" {
            score += 6;
        }
        if player == "Scissors" && opponent == "Paper" {
            score += 6;
        }
        if player == opponent {
            score += 3;
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "A Y\nB X\nC Z";
        let score = parse_guide_part1(input.into());
        assert_eq!(score, 15)
    }

    #[test]
    fn part2() {
        let input = "A Y\nB X\nC Z";
        let score = parse_guide_part2(input.into());
        assert_eq!(score, 12)
    }
}
