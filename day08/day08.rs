use std::collections::HashMap;
use std::env;

type Coordinate = (i32, i32);
type TreeHeights = HashMap<Coordinate, usize>;

fn is_visible(tree_heights: &TreeHeights, position: Coordinate) -> bool {
    let max_width = tree_heights.iter().map(|((x, _), _)| *x).max().unwrap();
    let max_height = tree_heights.iter().map(|((_, y), _)| *y).max().unwrap();

    let current_height = tree_heights.get(&position).unwrap();

    let left = (-1..position.0).all(|x| match tree_heights.get(&(x, position.1)) {
        Some(height) => height < current_height,
        None => true,
    });

    let top = (-1..position.1).all(|y| match tree_heights.get(&(position.0, y)) {
        Some(height) => height < current_height,
        None => true,
    });

    let right = (position.0 + 1..max_width + 1).all(|x| match tree_heights.get(&(x, position.1)) {
        Some(height) => height < current_height,
        None => true,
    });

    let bottom =
        (position.1 + 1..max_height + 1).all(|y| match tree_heights.get(&(position.0, y)) {
            Some(height) => height < current_height,
            None => true,
        });

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

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
