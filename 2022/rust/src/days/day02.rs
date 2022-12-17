use std::collections::HashMap;

pub fn process(part: i32, contents: String) -> i32 {
    if part == 1 {
        return part1(contents);
    } else if part == 2 {
        return part2(contents);
    }

    0
}

fn part1(input: String) -> i32 {
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

fn part2(input: String) -> i32 {
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
    fn test_part1() {
        let input = "A Y\nB X\nC Z";
        let score = part1(input.into());
        assert_eq!(score, 15)
    }

    #[test]
    fn test_part2() {
        let input = "A Y\nB X\nC Z";
        let score = part2(input.into());
        assert_eq!(score, 12)
    }
}
