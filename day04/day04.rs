use itertools::Itertools;
use std::env;

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    contents
        .lines()
        .filter_map(|line| {
            let mut groups = line.split(",").map(|group| {
                group
                    .split("-")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            });
            let a = groups.next().unwrap();
            let b = groups.next().unwrap();

            if (a[0] >= b[0] && a[1] <= b[1]) || (b[0] >= a[0] && b[1] <= a[1]) {
                return Some(line);
            }
            None
        })
        .count()
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    contents
        .lines()
        .filter_map(|line| {
            let mut groups = line.split(",").map(|group| {
                group
                    .split("-")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            });
            let a = groups.next().unwrap();
            let b = groups.next().unwrap();

            if (a[0] >= b[0] && a[0] <= b[1])
                || (a[1] <= b[1] && a[1] >= b[0])
                || (b[0] >= a[0] && b[0] <= a[1])
                || (b[0] <= a[1] && b[1] >= a[0])
            {
                return Some(line);
            }
            None
        })
        .count()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
