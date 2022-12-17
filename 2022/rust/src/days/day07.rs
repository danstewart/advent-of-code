use std::collections::HashMap;

pub fn process(part: i32, contents: String) -> i32 {
    if part == 1 {
        return part1(contents);
    } else if part == 2 {
        return part2(contents);
    }

    0
}

fn _get_directory_sizes(input: String) -> HashMap<String, i32> {
    let mut dir_size: HashMap<String, i32> = HashMap::new();
    let mut pos = vec![];
    let mut cmd = "";

    for line in input.lines() {
        // Parse command
        if line.starts_with("$ ") {
            let parts: Vec<&str> = line.split(" ").collect();
            cmd = parts[1];

            if cmd == "cd" {
                let arg = parts[2];

                if arg == ".." {
                    pos.pop();
                } else if arg == "/" {
                    pos = vec!["/"];
                } else {
                    pos.push(arg);
                }
            }
        // Parse output of the last command
        } else {
            if cmd == "ls" {
                let parts: Vec<&str> = line.split(" ").collect();

                if parts[0] == "dir" {
                    continue;
                }

                let size: i32 = parts[0].parse().unwrap();

                let mut full_path = "".to_string();
                for dir in &pos {
                    full_path.push_str(&dir);

                    dir_size
                        .entry(full_path.clone())
                        .and_modify(|s| *s += size)
                        .or_insert(size);
                }
            }
        }
    }

    dir_size
}

fn part1(input: String) -> i32 {
    let dir_size = _get_directory_sizes(input);

    let mut total_size = 0;
    for dir in dir_size.keys() {
        let size = *dir_size.get(dir).unwrap();
        if size <= 100000 {
            total_size += size;
        }
    }

    total_size
}

fn part2(input: String) -> i32 {
    const DISK_SIZE: i32 = 70000000;
    const REQUIRED_SPACE: i32 = 30000000;

    let dir_size = _get_directory_sizes(input);
    let current_usage = *dir_size.get("/").unwrap();
    let available_space = DISK_SIZE - current_usage;
    let to_clear = REQUIRED_SPACE - available_space;

    let mut smallest_size: i32 = 0;
    for dir in dir_size.keys() {
        let size = *dir_size.get(dir).unwrap();

        if size >= to_clear {
            if smallest_size == 0 || size < smallest_size {
                smallest_size = size;
            }
        }
    }

    smallest_size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = concat!(
            "$ cd /\n",
            "$ ls\n",
            "dir a\n",
            "14848514 b.txt\n",
            "8504156 c.dat\n",
            "dir d\n",
            "$ cd a\n",
            "$ ls\n",
            "dir e\n",
            "29116 f\n",
            "2557 g\n",
            "62596 h.lst\n",
            "$ cd e\n",
            "$ ls\n",
            "584 i\n",
            "$ cd ..\n",
            "$ cd ..\n",
            "$ cd d\n",
            "$ ls\n",
            "4060174 j\n",
            "8033020 d.log\n",
            "5626152 d.ext\n",
            "7214296 k\n",
        );

        let result = part1(input.to_string());
        assert_eq!(result, 95437);
    }

    #[test]
    fn test_part2() {
        let input = concat!(
            "$ cd /\n",
            "$ ls\n",
            "dir a\n",
            "14848514 b.txt\n",
            "8504156 c.dat\n",
            "dir d\n",
            "$ cd a\n",
            "$ ls\n",
            "dir e\n",
            "29116 f\n",
            "2557 g\n",
            "62596 h.lst\n",
            "$ cd e\n",
            "$ ls\n",
            "584 i\n",
            "$ cd ..\n",
            "$ cd ..\n",
            "$ cd d\n",
            "$ ls\n",
            "4060174 j\n",
            "8033020 d.log\n",
            "5626152 d.ext\n",
            "7214296 k\n",
        );

        let result = part2(input.to_string());
        assert_eq!(result, 24933642);
    }
}
