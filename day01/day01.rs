use itertools::Itertools;
use std::env;

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    contents
        .split("\n\n")
        .map(|lines| {
            lines
                .lines()
                .map(|x| x.parse::<usize>().unwrap())
                .fold(0, |acc, value| acc + value)
        })
        .max()
        .unwrap()
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    contents
        .split("\n\n")
        .map(|lines| {
            lines
                .lines()
                .map(|x| x.parse::<usize>().unwrap())
                .fold(0, |acc, value| acc + value)
        })
        .sorted()
        .rev()
        .take(3)
        .fold(0, |acc, value| acc + value)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
