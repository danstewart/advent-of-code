use std::collections::BTreeMap;

pub fn process(part: i32, contents: String) -> i32 {
    if part == 1 {
        return count_calories(contents, 1);
    } else if part == 2 {
        return count_calories(contents, 3);
    }

    0
}

fn count_calories(contents: String, count_needed: i32) -> i32 {
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

        calories_to_elve.insert(total, idx);

        idx += 1;
    }

    // Grab the top $TOP and total the calories
    let mut total_calories = 0;
    let mut iterable = calories_to_elve.iter().rev();
    for _ in 0..count_needed {
        let item = iterable.next().unwrap();
        total_calories += item.0;
    }

    println!(
        "Total calories for top {} elves: {}",
        count_needed, total_calories
    );
    total_calories
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = concat!(
            "1000\n2000\n3000\n",
            "\n",
            "4000\n",
            "\n",
            "5000\n6000\n",
            "\n",
            "7000\n8000\n9000\n",
            "\n",
            "10000\n"
        );

        let result = count_calories(input.into(), 1);
        assert_eq!(result, 24000);
    }

    #[test]
    fn test_part2() {
        let input = concat!(
            "1000\n2000\n3000\n",
            "\n",
            "4000\n",
            "\n",
            "5000\n6000\n",
            "\n",
            "7000\n8000\n9000\n",
            "\n",
            "10000\n"
        );

        let result = count_calories(input.into(), 3);
        assert_eq!(result, 45000);
    }
}
