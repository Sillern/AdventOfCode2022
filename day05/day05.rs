use regex::Regex;
use std::collections::VecDeque;
use std::env;

fn print_stacks(stacks: &Vec<VecDeque<char>>) {
    let max_height = stacks.iter().map(|stack| stack.len()).max().unwrap();
    for depth in 0..max_height {
        for stack in stacks {
            let index = max_height - depth;
            if index > stack.len() {
                print!("    ");
            } else {
                print!("[{}] ", stack[index - 1]);
            }
        }
        println!();
    }
}

fn solve_part1(inputfile: String) -> String {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let re = Regex::new(r"(\[|\s)(?P<crate>([A-Z]|\s))(\]|\s)\s?").unwrap();
    let mut contents_tokens = contents.split("\n\n");

    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    contents_tokens.next().unwrap().lines().for_each(|line| {
        re.captures_iter(line)
            .enumerate()
            .for_each(|(index, cargo)| {
                if stacks.len() < (index + 1) {
                    stacks.push(VecDeque::new());
                }
                let crate_id = cargo["crate"].chars().next().unwrap();
                if crate_id != ' ' {
                    stacks[index].push_front(crate_id);
                }
            });
    });

    let instruction_pattern =
        Regex::new(r"move\s(?P<amount>\d+)\sfrom\s(?P<source>\d+)\sto\s(?P<destination>\d+)")
            .unwrap();
    contents_tokens.next().unwrap().lines().for_each(|line| {
        let parsed = instruction_pattern.captures(line).unwrap();

        let amount = parsed["amount"].parse::<usize>().unwrap();
        let source = parsed["source"].parse::<usize>().unwrap() - 1;
        let destination = parsed["destination"].parse::<usize>().unwrap() - 1;
        for _ in 0..amount {
            let cargo = stacks[source].pop_back().unwrap();
            stacks[destination].push_back(cargo);
        }
    });

    stacks
        .iter()
        .map(|stack| stack[stack.len() - 1])
        .collect::<String>()
}

fn solve_part2(inputfile: String) -> String {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let re = Regex::new(r"(\[|\s)(?P<crate>([A-Z]|\s))(\]|\s)\s?").unwrap();
    let mut contents_tokens = contents.split("\n\n");

    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    contents_tokens.next().unwrap().lines().for_each(|line| {
        re.captures_iter(line)
            .enumerate()
            .for_each(|(index, cargo)| {
                if stacks.len() < (index + 1) {
                    stacks.push(VecDeque::new());
                }
                let crate_id = cargo["crate"].chars().next().unwrap();
                if crate_id != ' ' {
                    stacks[index].push_front(crate_id);
                }
            });
    });

    let instruction_pattern =
        Regex::new(r"move\s(?P<amount>\d+)\sfrom\s(?P<source>\d+)\sto\s(?P<destination>\d+)")
            .unwrap();
    contents_tokens.next().unwrap().lines().for_each(|line| {
        let parsed = instruction_pattern.captures(line).unwrap();

        let amount = parsed["amount"].parse::<usize>().unwrap();
        let source = parsed["source"].parse::<usize>().unwrap() - 1;
        let destination = parsed["destination"].parse::<usize>().unwrap() - 1;

        let stack_size = stacks[source].len();
        let cargo = stacks[source]
            .drain(stack_size - amount..)
            .collect::<VecDeque<char>>();
        for x in cargo {
            stacks[destination].push_back(x);
        }
    });

    stacks
        .iter()
        .map(|stack| stack[stack.len() - 1])
        .collect::<String>()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
