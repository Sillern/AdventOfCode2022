use itertools::Itertools;
use std::env;

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    contents
        .lines()
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .tuple_windows()
        .find_map(|((_, a), (_, b), (_, c), (count, d))| {
            if vec![a, b, c, d].iter().all_unique() {
                return Some(count + 1);
            }
            None
        })
        .unwrap()
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let line = contents.lines().next().unwrap().chars();

    let line_length = line.clone().count();
    for start in 0..line_length {
        let end = start + 14;
        if end > line_length {
            return 0;
        }
        let all_unique = line.clone().skip(start).take(14).all_unique();
        if all_unique {
            return start + 14;
        }
    }
    return 0;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
