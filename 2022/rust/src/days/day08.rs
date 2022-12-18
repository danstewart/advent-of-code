use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    vec,
};

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

/// Parse the input string and return a list of list of trees
fn _build_tree_map(input: String) -> Vec<Vec<i32>> {
    let mut tree_map: Vec<Vec<i32>> = vec![];

    for line in input.lines() {
        let mut row = vec![];
        for tree in line.split("") {
            if let Ok(tree_height) = tree.parse() {
                row.push(tree_height);
            }
        }
        tree_map.push(row);
    }

    tree_map
}

/// Return a set of the visible trees in the format Pos(X, Y)
/// eg. [ Pos(0, 0), Pos(1, 5), Pos(2, 3) ]
/// tree_map: The list of trees
fn _find_visible_trees(tree_map: &Vec<Vec<i32>>) -> HashSet<Pos> {
    fn _is_visible(tree: i32, pos: i32, tallest: &mut HashMap<i32, i32>) -> bool {
        if tree > *tallest.get(&pos).unwrap_or(&-1) {
            tallest.insert(pos, tree);
            return true;
        }

        false
    }

    let mut visible_trees: HashSet<Pos> = HashSet::new();

    let mut tallest_in_x: HashMap<i32, i32> = HashMap::new();
    let mut tallest_in_y: HashMap<i32, i32> = HashMap::new();

    // First go left to right and top to bottom
    for y_pos in 0..tree_map.len() {
        for x_pos in 0..tree_map[y_pos].len() {
            let tree = tree_map[y_pos][x_pos];
            let visible_y = _is_visible(tree, y_pos as i32, &mut tallest_in_x);
            let visible_x = _is_visible(tree, x_pos as i32, &mut tallest_in_y);

            if visible_y || visible_x {
                visible_trees.insert(Pos {
                    x: x_pos as i32,
                    y: y_pos as i32,
                });
            }
        }
    }

    // Reset
    tallest_in_y.clear();
    tallest_in_x.clear();

    // And again from right to left and bottom to top
    for y_pos in (0..tree_map.len()).rev() {
        for x_pos in (0..tree_map[y_pos].len()).rev() {
            let tree = tree_map[y_pos][x_pos];

            let visible_y = _is_visible(tree, y_pos as i32, &mut tallest_in_x);
            let visible_x = _is_visible(tree, x_pos as i32, &mut tallest_in_y);

            if visible_y || visible_x {
                visible_trees.insert(Pos {
                    x: x_pos as i32,
                    y: y_pos as i32,
                });
            }
        }
    }

    visible_trees
}

fn part1(input: String) -> i32 {
    let tree_map = _build_tree_map(input);
    let visible_trees = _find_visible_trees(&tree_map);
    visible_trees.len() as i32
}

fn part2(input: String) -> i32 {
    let tree_map = _build_tree_map(input);

    let mut highest_score = 0;

    let total_y = tree_map.len();
    for y_pos in 0..total_y {
        let total_x = tree_map[y_pos].len();

        // Just skip trees on the edge
        if y_pos == 0 || y_pos == total_y {
            continue;
        }

        for x_pos in 0..total_x {
            // Just skip trees on the edge
            if x_pos == 0 || x_pos == total_x {
                continue;
            }

            let tree = tree_map[y_pos][x_pos];

            let mut visible_up = 0;
            let mut visible_down = 0;
            let mut visible_left = 0;
            let mut visible_right = 0;

            // Search right
            for x in x_pos + 1..total_x {
                visible_right += 1;

                if tree_map[y_pos][x] >= tree {
                    break;
                }
            }

            // Search left
            for x in (0..x_pos).rev() {
                visible_left += 1;

                if tree_map[y_pos][x] >= tree {
                    break;
                }
            }

            // Search down
            for y in y_pos + 1..total_y {
                visible_down += 1;

                if tree_map[y][x_pos] >= tree {
                    break;
                }
            }

            // Search up
            for y in (0..y_pos).rev() {
                visible_up += 1;

                if tree_map[y][x_pos] >= tree {
                    break;
                }
            }

            let score = visible_up * visible_down * visible_right * visible_left;
            if score > highest_score {
                highest_score = score;
            }
        }
    }

    highest_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = concat!("30373\n", "25512\n", "65332\n", "33549\n", "35390",);
        let result = part1(input.to_string());
        assert_eq!(result, 21);
    }

    #[test]
    fn test_part2() {
        let input = concat!("30373\n", "25512\n", "65332\n", "33549\n", "35390",);
        let result = part2(input.to_string());
        assert_eq!(result, 8);
    }
}
