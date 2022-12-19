// Template for new puzzles
// Remember
// - Update mod.rs to export the new day
// - Update the `match` statement in `main.rs`
// - Add the input file to `data/`

use std::collections::HashSet;

pub fn process(part: i32, contents: String) -> i32 {
    if part == 1 {
        return part1(contents);
    } else if part == 2 {
        return part2(contents);
    }

    0
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    /// Move towards another position
    /// If the target position is "touching" our position do nothing
    fn move_towards(&mut self, other: &Pos) {
        let diff_x = other.x - self.x;
        let diff_y = other.y - self.y;

        // Points are "touching", do nothing
        if diff_x.abs() <= 1 && diff_y.abs() <= 1 {
            return;
        }

        if diff_x > 0 {
            self.x += 1;
        } else if diff_x < 0 {
            self.x -= 1;
        }

        if diff_y > 0 {
            self.y += 1;
        } else if diff_y < 0 {
            self.y -= 1;
        }
    }
}

fn part1(input: String) -> i32 {
    let mut positions: HashSet<Pos> = HashSet::new();

    let mut head = Pos { x: 0, y: 0 };
    let mut tail = Pos { x: 0, y: 0 };

    for line in input.lines() {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let direction: &str = parts[0];
        let spaces: i32 = parts[1].parse().unwrap();

        for _ in 0..spaces {
            match direction {
                "R" => head.x += 1,
                "L" => head.x -= 1,
                "U" => head.y += 1,
                "D" => head.y -= 1,
                _ => continue,
            }

            tail.move_towards(&head);
            positions.insert(Pos {
                x: tail.x,
                y: tail.y,
            });
        }
    }

    positions.len() as i32
}

fn part2(input: String) -> i32 {
    let mut positions: HashSet<Pos> = HashSet::new();

    let mut knots: Vec<Pos> = vec![];
    for _ in 0..=9 {
        let knot = Pos { x: 0, y: 0 };
        knots.push(knot);
    }

    for line in input.lines() {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let direction: &str = parts[0];
        let spaces: i32 = parts[1].parse().unwrap();

        for _ in 0..spaces {
            match direction {
                "R" => knots[0].x += 1,
                "L" => knots[0].x -= 1,
                "U" => knots[0].y += 1,
                "D" => knots[0].y -= 1,
                _ => continue,
            }

            for knot in 1..=9 {
                let head = Pos {
                    x: knots[knot - 1].x,
                    y: knots[knot - 1].y,
                };
                let tail = &mut knots[knot];
                tail.move_towards(&head);

                if knot == 9 {
                    positions.insert(Pos {
                        x: tail.x,
                        y: tail.y,
                    });
                }
            }
        }
    }

    positions.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = concat!("R 4\n", "U 4\n", "L 3\n", "D 1\n", "R 4\n", "D 1\n", "L 5\n", "R 2",);
        let result = part1(input.to_string());
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part2() {
        let input = concat!("R 4\n", "U 4\n", "L 3\n", "D 1\n", "R 4\n", "D 1\n", "L 5\n", "R 2",);
        let result = part2(input.to_string());
        assert_eq!(result, 1);

        let input =
            concat!("R 5\n", "U 8\n", "L 8\n", "D 3\n", "R 17\n", "D 10\n", "L 25\n", "U 20",);
        let result = part2(input.to_string());
        assert_eq!(result, 36);
    }
}
