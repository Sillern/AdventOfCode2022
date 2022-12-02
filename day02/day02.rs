use itertools::Itertools;
use std::env;

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    contents
        .lines()
        .map(|line| {
            let (a, b) = line.split(" ").next_tuple().unwrap();
            match a {
                "A" => match b {
                    "X" => 1 + 3,
                    "Y" => 2 + 6,
                    "Z" => 3 + 0,
                    _ => panic!(),
                },
                "B" => match b {
                    "X" => 1 + 0,
                    "Y" => 2 + 3,
                    "Z" => 3 + 6,
                    _ => panic!(),
                },
                "C" => match b {
                    "X" => 1 + 6,
                    "Y" => 2 + 0,
                    "Z" => 3 + 3,
                    _ => panic!(),
                },
                _ => panic!(),
            }
        })
        .sum()
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    contents
        .lines()
        .map(|line| {
            let (a, b) = line.split(" ").next_tuple().unwrap();
            match a {
                "A" => match b {
                    "X" => 3 + 0,
                    "Y" => 1 + 3,
                    "Z" => 2 + 6,
                    _ => panic!(),
                },
                "B" => match b {
                    "X" => 1 + 0,
                    "Y" => 2 + 3,
                    "Z" => 3 + 6,
                    _ => panic!(),
                },
                "C" => match b {
                    "X" => 2 + 0,
                    "Y" => 3 + 3,
                    "Z" => 1 + 6,
                    _ => panic!(),
                },
                _ => panic!(),
            }
        })
        .sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
