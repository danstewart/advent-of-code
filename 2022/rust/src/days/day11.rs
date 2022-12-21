pub fn process(part: i32, contents: String) -> i32 {
    if part == 1 {
        return part1(contents);
    } else if part == 2 {
        return part2(contents);
    }

    0
}

enum Operation {
    Add,
    Multiply,
}

struct Monkey {
    items: Vec<i32>,
    division_test: i32,
    operation: Operation,
    operation_amount: i32,
    on_true: i32,
    on_false: i32,
}

impl Monkey {
    fn from_string(input: String) -> Self {
        let lines: Vec<&str> = input.split("\n").collect();
        let starting_items: Vec<i32> = lines[1]
            .split(":")
            .last()
            .unwrap()
            .split(",")
            .map(|item| item.trim().parse().unwrap())
            .collect();
        let operation = lines[2].split(":").last().unwrap();
        let target_when_true: i32 = lines[4].split(" ").last().unwrap().parse().unwrap();
        let target_when_false: i32 = lines[5].split(" ").last().unwrap().parse().unwrap();

        // TODO: Parse operation

        Monkey {
            items: starting_items,
            division_test: 0,
            operation: Operation::Add,
            operation_amount: 0,
            on_true: target_when_true,
            on_false: target_when_false,
        }
    }
}

fn part1(input: String) -> i32 {
    let mut monkeys: Vec<Monkey> = vec![];

    for chunk in input.split("\n\n") {
        monkeys.push(Monkey::from_string(chunk.to_string()));
    }

    0
}

fn part2(_input: String) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = concat!(
        "Monkey 0:\n",
        "    Starting items: 79, 98\n",
        "    Operation: new = old * 19\n",
        "    Test: divisible by 23\n",
        "        If true: throw to monkey 2\n",
        "        If false: throw to monkey 3\n",
        "\n",
        "    Monkey 1:\n",
        "    Starting items: 54, 65, 75, 74\n",
        "    Operation: new = old + 6\n",
        "    Test: divisible by 19\n",
        "        If true: throw to monkey 2\n",
        "        If false: throw to monkey 0\n",
        "\n",
        "    Monkey 2:\n",
        "    Starting items: 79, 60, 97\n",
        "    Operation: new = old * old\n",
        "    Test: divisible by 13\n",
        "        If true: throw to monkey 1\n",
        "        If false: throw to monkey 3\n",
        "\n",
        "    Monkey 3:\n",
        "    Starting items: 74\n",
        "    Operation: new = old + 3\n",
        "    Test: divisible by 17\n",
        "        If true: throw to monkey 0\n",
        "        If false: throw to monkey 1\n",
    );

    #[test]
    fn test_part1() {
        let result = part1(INPUT.to_string());
        assert_eq!(result, 10605);
    }

    #[test]
    fn test_part2() {
        let input = "".into();
        let result = part2(input);
        assert_eq!(result, 0);
    }
}
