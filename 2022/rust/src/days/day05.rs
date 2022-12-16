use std::collections::VecDeque;

pub fn process(part: i32, contents: String) -> String {
    if part == 1 {
        return part1(contents);
    } else if part == 2 {
        return part2(contents);
    }

    "".into()
}

// TODO: Split out the parsing and moving into functions
fn part1(input: String) -> String {
    // First build our current stacks
    // Parse the crate columns until we reach the bottom
    let mut stacks: Vec<VecDeque<String>> = vec![];
    let mut end_of_stacks = false;

    for line in input.lines() {
        let col_width = 3;
        let mut col_idx = 0;
        let mut pos = 0;

        if !end_of_stacks {
            loop {
                if pos + col_width > line.len() {
                    break;
                }

                let mut col = line[pos..pos + col_width].to_string();
                col = col.replace("[", "").replace("]", "").trim().to_string();

                // When we hit a number we're at the bottom of the stack
                if col == "1".to_string() {
                    end_of_stacks = true;
                    break;
                }

                if col_idx >= stacks.len() {
                    let mut stack = VecDeque::new();
                    if col != "".to_string() {
                        stack.push_back(col);
                    }
                    stacks.push(stack);
                } else {
                    if col != "".to_string() {
                        stacks[col_idx].push_back(col);
                    }
                }

                col_idx += 1;
                pos += 1 + col_width;
            }
        } else {
            // Now start parsing instructions
            if line.starts_with("move") {
                let parts: Vec<&str> = line.split(" ").collect();
                let quantity: usize = parts[1].parse().unwrap();
                let from_col: usize = parts[3].parse().unwrap();
                let to_col: usize = parts[5].parse().unwrap();

                for _ in 0..quantity {
                    let item = stacks[from_col - 1].pop_front().unwrap();
                    stacks[to_col - 1].push_front(item);
                }
            }
        }
    }

    let mut result = "".to_string();
    for stack in stacks {
        if stack.len() > 0 {
            result.push_str(&stack[0]);
        }
    }

    result
}

fn part2(input: String) -> String {
    // First build our current stacks
    // Parse the crate columns until we reach the bottom
    let mut stacks: Vec<VecDeque<String>> = vec![];
    let mut end_of_stacks = false;

    for line in input.lines() {
        let col_width = 3;
        let mut col_idx = 0;
        let mut pos = 0;

        if !end_of_stacks {
            loop {
                if pos + col_width > line.len() {
                    break;
                }

                let mut col = line[pos..pos + col_width].to_string();
                col = col.replace("[", "").replace("]", "").trim().to_string();

                // When we hit a number we're at the bottom of the stack
                if col == "1".to_string() {
                    end_of_stacks = true;
                    break;
                }

                if col_idx >= stacks.len() {
                    let mut stack = VecDeque::new();
                    if col != "".to_string() {
                        stack.push_back(col);
                    }
                    stacks.push(stack);
                } else {
                    if col != "".to_string() {
                        stacks[col_idx].push_back(col);
                    }
                }

                col_idx += 1;
                pos += 1 + col_width;
            }
        } else {
            // Now start parsing instructions
            if line.starts_with("move") {
                let parts: Vec<&str> = line.split(" ").collect();
                let quantity: usize = parts[1].parse().unwrap();
                let from_col: usize = parts[3].parse().unwrap();
                let to_col: usize = parts[5].parse().unwrap();

                let mut items_to_move: Vec<String> = vec![];

                for _ in 0..quantity {
                    items_to_move.insert(0, stacks[from_col - 1].pop_front().unwrap());
                }

                for item in items_to_move {
                    stacks[to_col - 1].push_front(item);
                }
            }
        }
    }

    let mut result = "".to_string();
    for stack in stacks {
        if stack.len() > 0 {
            result.push_str(&stack[0]);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = concat!(
            "    [D]    \n",
            "[N] [C]    \n",
            "[Z] [M] [P]\n",
            " 1   2   3 \n",
            "\n",
            "move 1 from 2 to 1\n",
            "move 3 from 1 to 3\n",
            "move 2 from 2 to 1\n",
            "move 1 from 1 to 2"
        )
        .into();

        let result = part1(input);
        assert_eq!(result, "CMZ".to_string());
    }

    #[test]
    fn test_part2() {
        let input = concat!(
            "    [D]    \n",
            "[N] [C]    \n",
            "[Z] [M] [P]\n",
            " 1   2   3 \n",
            "\n",
            "move 1 from 2 to 1\n",
            "move 3 from 1 to 3\n",
            "move 2 from 2 to 1\n",
            "move 1 from 1 to 2"
        )
        .into();

        let result = part2(input);
        assert_eq!(result, "MCD".to_string());
    }
}
