pub fn process(part: i32, contents: String) -> i32 {
    if part == 1 {
        return part1(contents);
    } else if part == 2 {
        println!("{}", part2(contents));
    }

    0
}

fn part1(input: String) -> i32 {
    let mut register: i32 = 1;
    let mut cycle: i32 = 0;
    let mut sum_signal: i32 = 0;

    // Incrememts the cycle number and every 40 cycles (from cycle 20) adds to sum_signal
    fn incr_cycle(cycle: &mut i32, sum_signal: &mut i32, register: i32) {
        *cycle += 1;

        if (*cycle - 20) % 40 == 0 {
            *sum_signal += register * *cycle;
        }
    }

    for line in input.lines() {
        if line == "noop" {
            incr_cycle(&mut cycle, &mut sum_signal, register);
        } else if line.starts_with("addx") {
            let value: i32 = line.split(" ").last().unwrap().parse().unwrap();

            incr_cycle(&mut cycle, &mut sum_signal, register);
            incr_cycle(&mut cycle, &mut sum_signal, register);

            register += value;
        }
    }

    sum_signal
}

fn part2(input: String) -> String {
    const WIDTH: usize = 40;
    const HEIGHT: usize = 6;

    let mut register: i32 = 1;
    let mut cycle: i32 = 0;
    let mut output: Vec<String> = Vec::with_capacity(HEIGHT);

    fn incr_cycle(cycle: &mut i32, register: i32, row: &mut Vec<String>, output: &mut Vec<String>) {
        let mut pixel_on: bool = true;

        if register < *cycle - 1 || register > *cycle + 1 {
            pixel_on = false;
        }

        if pixel_on {
            row.push("#".to_string());
        } else {
            row.push(".".to_string());
        }

        *cycle += 1;

        if *cycle >= 40 {
            output.push(row.join(""));
            row.clear();
            *cycle -= 40;
        }
    }

    let mut row: Vec<String> = Vec::with_capacity(WIDTH);
    for line in input.lines() {
        if line == "noop" {
            incr_cycle(&mut cycle, register, &mut row, &mut output);
        } else if line.starts_with("addx") {
            let value: i32 = line.split(" ").last().unwrap().parse().unwrap();

            incr_cycle(&mut cycle, register, &mut row, &mut output);
            incr_cycle(&mut cycle, register, &mut row, &mut output);

            register += value;
        }
    }

    output.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = concat!(
        "addx 15\n",
        "addx -11\n",
        "addx 6\n",
        "addx -3\n",
        "addx 5\n",
        "addx -1\n",
        "addx -8\n",
        "addx 13\n",
        "addx 4\n",
        "noop\n",
        "addx -1\n",
        "addx 5\n",
        "addx -1\n",
        "addx 5\n",
        "addx -1\n",
        "addx 5\n",
        "addx -1\n",
        "addx 5\n",
        "addx -1\n",
        "addx -35\n",
        "addx 1\n",
        "addx 24\n",
        "addx -19\n",
        "addx 1\n",
        "addx 16\n",
        "addx -11\n",
        "noop\n",
        "noop\n",
        "addx 21\n",
        "addx -15\n",
        "noop\n",
        "noop\n",
        "addx -3\n",
        "addx 9\n",
        "addx 1\n",
        "addx -3\n",
        "addx 8\n",
        "addx 1\n",
        "addx 5\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx -36\n",
        "noop\n",
        "addx 1\n",
        "addx 7\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx 2\n",
        "addx 6\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx 1\n",
        "noop\n",
        "noop\n",
        "addx 7\n",
        "addx 1\n",
        "noop\n",
        "addx -13\n",
        "addx 13\n",
        "addx 7\n",
        "noop\n",
        "addx 1\n",
        "addx -33\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx 2\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx 8\n",
        "noop\n",
        "addx -1\n",
        "addx 2\n",
        "addx 1\n",
        "noop\n",
        "addx 17\n",
        "addx -9\n",
        "addx 1\n",
        "addx 1\n",
        "addx -3\n",
        "addx 11\n",
        "noop\n",
        "noop\n",
        "addx 1\n",
        "noop\n",
        "addx 1\n",
        "noop\n",
        "noop\n",
        "addx -13\n",
        "addx -19\n",
        "addx 1\n",
        "addx 3\n",
        "addx 26\n",
        "addx -30\n",
        "addx 12\n",
        "addx -1\n",
        "addx 3\n",
        "addx 1\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx -9\n",
        "addx 18\n",
        "addx 1\n",
        "addx 2\n",
        "noop\n",
        "noop\n",
        "addx 9\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx -1\n",
        "addx 2\n",
        "addx -37\n",
        "addx 1\n",
        "addx 3\n",
        "noop\n",
        "addx 15\n",
        "addx -21\n",
        "addx 22\n",
        "addx -6\n",
        "addx 1\n",
        "noop\n",
        "addx 2\n",
        "addx 1\n",
        "noop\n",
        "addx -10\n",
        "noop\n",
        "noop\n",
        "addx 20\n",
        "addx 1\n",
        "addx 2\n",
        "addx 2\n",
        "addx -6\n",
        "addx -11\n",
        "noop\n",
        "noop\n",
        "noop",
    );

    #[test]
    fn test_part1() {
        let result = part1(INPUT.to_string());
        assert_eq!(result, 13140);
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT.to_string());

        let expected = concat!(
            "##..##..##..##..##..##..##..##..##..##..\n",
            "###...###...###...###...###...###...###.\n",
            "####....####....####....####....####....\n",
            "#####.....#####.....#####.....#####.....\n",
            "######......######......######......####\n",
            "#######.......#######.......#######.....",
        );

        assert_eq!(result, expected.to_string());
    }
}
