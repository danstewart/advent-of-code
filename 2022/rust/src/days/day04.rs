use std::ops::RangeInclusive;

pub fn process(part: i32, contents: String) -> i32 {
    if part == 1 {
        return part1(contents);
    } else if part == 2 {
        return part2(contents);
    }

    0
}

/// Parse a line of input and return the range of areas covered by each elf
fn parse_line(line: String) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let mut parts = line.split(",");
    let first = parts.next().unwrap();
    let second = parts.next().unwrap();

    let mut first_parts = first.split("-");
    let first_start: i32 = first_parts.next().unwrap().parse().unwrap();
    let first_end: i32 = first_parts.next().unwrap().parse().unwrap();

    let mut second_parts = second.split("-");
    let second_start: i32 = second_parts.next().unwrap().parse().unwrap();
    let second_end: i32 = second_parts.next().unwrap().parse().unwrap();

    let first_range = first_start..=first_end;
    let second_range = second_start..=second_end;

    (first_range, second_range)
}

fn part1(input: String) -> i32 {
    let mut lines: Vec<&str> = input.split("\n").collect();
    if lines[lines.len() - 1] == "" {
        lines.pop();
    }

    let mut duplicates = 0;

    for line in lines {
        let (first_range, second_range) = parse_line(line.into());

        let mut contained = true;
        for i in first_range.clone() {
            if !second_range.contains(&i) {
                contained = false;
                break;
            }
        }

        if contained {
            duplicates += 1;
            continue;
        }

        contained = true;
        for i in second_range {
            if !first_range.contains(&i) {
                contained = false;
                break;
            }
        }

        if contained {
            duplicates += 1;
            continue;
        }
    }

    duplicates
}

fn part2(input: String) -> i32 {
    let mut lines: Vec<&str> = input.split("\n").collect();
    if lines[lines.len() - 1] == "" {
        lines.pop();
    }

    let mut duplicates = 0;

    for line in lines {
        let (first_range, second_range) = parse_line(line.into());

        let mut contained = false;
        for i in first_range.clone() {
            if second_range.contains(&i) {
                contained = true;
                break;
            }
        }

        if contained {
            duplicates += 1;
            continue;
        }

        for i in second_range {
            if first_range.contains(&i) {
                contained = true;
                break;
            }
        }

        if contained {
            duplicates += 1;
            continue;
        }
    }

    duplicates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = concat!(
            "2-4,6-8\n",
            "2-3,4-5\n",
            "5-7,7-9\n",
            "2-8,3-7\n",
            "6-6,4-7\n",
            "2-6,4-8\n"
        );

        let result = part1(input.into());
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part2() {
        let input = concat!(
            "2-4,6-8\n",
            "2-3,4-5\n",
            "5-7,7-9\n",
            "2-8,3-7\n",
            "6-6,4-7\n",
            "2-6,4-8\n"
        );

        let result = part2(input.into());
        assert_eq!(result, 4);
    }
}
