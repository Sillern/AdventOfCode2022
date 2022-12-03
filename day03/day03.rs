use itertools::Itertools;
use std::env;

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let to_priority = |item: char| -> u32 {
        if item.is_lowercase() {
            1 + (item as u32) - ('a' as u32)
        } else {
            27 + (item as u32) - ('A' as u32)
        }
    };

    contents
        .lines()
        .filter_map(|line| {
            let line_size = line.len();
            let (a, b) = line.split_at(line_size / 2);

            for a_ in a.chars() {
                if b.contains(a_) {
                    return Some(a_);
                }
            }
            None
        })
        .fold(0, |acc, item| acc + to_priority(item) as usize)
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let to_priority = |item: char| -> u32 {
        if item.is_lowercase() {
            1 + (item as u32) - ('a' as u32)
        } else {
            27 + (item as u32) - ('A' as u32)
        }
    };

    contents
        .lines()
        .map(|line| line.chars().sorted().unique().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
        .chunks(3)
        .map(|group| {
            group
                .concat()
                .into_iter()
                .sorted()
                .dedup_with_count()
                .find_map(|(count, badge)| {
                    if count == 3 {
                        Some(to_priority(badge))
                    } else {
                        None
                    }
                })
                .unwrap() as usize
        })
        .sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
