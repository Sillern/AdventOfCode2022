use std::collections::HashMap;
use std::env;

type Coordinate = (i32, i32);
type TreeHeights = HashMap<Coordinate, usize>;

fn is_visible(tree_heights: &TreeHeights, position: Coordinate) -> bool {
    let max_width = tree_heights.iter().map(|((x, _), _)| *x).max().unwrap();
    let max_height = tree_heights.iter().map(|((_, y), _)| *y).max().unwrap();

    let current_height = tree_heights.get(&position).unwrap();

    let check_visibility = |height_opt: Option<&usize>| -> bool {
        match height_opt {
            Some(height) => height < current_height,
            None => true,
        }
    };

    let left = (-1..position.0).all(|x| check_visibility(tree_heights.get(&(x, position.1))));
    let top = (-1..position.1).all(|y| check_visibility(tree_heights.get(&(position.0, y))));
    let right = (position.0 + 1..max_width + 1)
        .all(|x| check_visibility(tree_heights.get(&(x, position.1))));
    let bottom = (position.1 + 1..max_height + 1)
        .all(|y| check_visibility(tree_heights.get(&(position.0, y))));

    left || right || top || bottom
}

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut tree_heights: TreeHeights = HashMap::new();
    contents.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, height)| {
            tree_heights.insert((x as i32, y as i32), height.to_digit(10).unwrap() as usize);
        })
    });

    tree_heights
        .iter()
        .filter(|(position, _)| is_visible(&tree_heights, **position))
        .count()
}

fn scenic_score(tree_heights: &TreeHeights, position: Coordinate) -> usize {
    let max_width = tree_heights.iter().map(|((x, _), _)| *x).max().unwrap();
    let max_height = tree_heights.iter().map(|((_, y), _)| *y).max().unwrap();

    let current_height = tree_heights.get(&position).unwrap();

    let count_visible_trees = |height_opt: Option<&usize>,
                               current_height: usize,
                               trees: usize,
                               blocked_view: bool|
     -> (usize, bool) {
        match height_opt {
            Some(height) => {
                if blocked_view {
                    (trees, blocked_view)
                } else if *height >= current_height {
                    (trees + 1, true)
                } else {
                    (trees + 1, blocked_view)
                }
            }
            None => (trees, blocked_view),
        }
    };

    let left = (-1..position.0)
        .rev()
        .fold((0, false), |(trees, blocked_view), x| {
            count_visible_trees(
                tree_heights.get(&(x, position.1)),
                *current_height,
                trees,
                blocked_view,
            )
        });

    let up = (-1..position.1)
        .rev()
        .fold((0, false), |(trees, blocked_view), y| {
            count_visible_trees(
                tree_heights.get(&(position.0, y)),
                *current_height,
                trees,
                blocked_view,
            )
        });

    let right = (position.0 + 1..max_width + 1).fold((0, false), |(trees, blocked_view), x| {
        count_visible_trees(
            tree_heights.get(&(x, position.1)),
            *current_height,
            trees,
            blocked_view,
        )
    });

    let down = (position.1 + 1..max_height + 1).fold((0, false), |(trees, blocked_view), y| {
        count_visible_trees(
            tree_heights.get(&(position.0, y)),
            *current_height,
            trees,
            blocked_view,
        )
    });

    up.0 * left.0 * right.0 * down.0
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut tree_heights: TreeHeights = HashMap::new();
    contents.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, height)| {
            tree_heights.insert((x as i32, y as i32), height.to_digit(10).unwrap() as usize);
        })
    });

    tree_heights
        .iter()
        .map(|(position, _)| scenic_score(&tree_heights, *position))
        .max()
        .unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
