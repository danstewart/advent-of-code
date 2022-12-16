use std::collections::HashSet;
use std::collections::VecDeque;

pub fn process(part: i32, contents: String) -> i32 {
    if part == 1 {
        return part1(contents);
    } else if part == 2 {
        return part2(contents);
    }

    0
}

fn _find_index_of_first_n_unique(input: String, n: usize) -> i32 {
    let mut seq: VecDeque<char> = VecDeque::new();

    let mut idx = 0;
    for letter in input.chars() {
        if seq.len() > n {
            seq.pop_front();
        }

        if seq.len() == n {
            let mut set: HashSet<&char> = HashSet::new();

            for item in &seq {
                set.insert(item);
            }

            if set.len() == n {
                return idx;
            }
        }

        seq.push_back(letter);
        idx += 1;
    }

    idx
}

fn part1(input: String) -> i32 {
    return _find_index_of_first_n_unique(input, 4);
}

fn part2(input: String) -> i32 {
    return _find_index_of_first_n_unique(input, 14);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut input = "bvwbjplbgvbhsrlpgdmjqwftvncz".into();
        assert_eq!(part1(input), 5);

        input = "nppdvjthqldpwncqszvftbrmjlhg".into();
        assert_eq!(part1(input), 6);

        input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".into();
        assert_eq!(part1(input), 10);

        input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".into();
        assert_eq!(part1(input), 11);
    }

    #[test]
    fn test_part2() {
        let mut input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".into();
        assert_eq!(part2(input), 19);

        input = "bvwbjplbgvbhsrlpgdmjqwftvncz".into();
        assert_eq!(part2(input), 23);

        input = "nppdvjthqldpwncqszvftbrmjlhg".into();
        assert_eq!(part2(input), 23);

        input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".into();
        assert_eq!(part2(input), 29);

        input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".into();
        assert_eq!(part2(input), 26);
    }
}
