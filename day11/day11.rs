use itertools::Itertools;
use std::env;

#[derive(Debug)]
enum Operation {
    Unknown,
    Add,
    Multiply,
}

#[derive(Debug)]
struct Monkey {
    inspect_count: usize,
    items: Vec<i64>,
    operation: Operation,
    second_operand: Option<i64>,
    divisible_by: i64,
    throw_to_if_true: usize,
    throw_to_if_false: usize,
}

fn parse_monkeys(contents: String) -> Vec<Monkey> {
    contents
        .split("\n\n")
        .map(|section| {
            let mut monkey_id: usize = 0;
            let mut items: Vec<i64> = Vec::new();
            let mut operation: Operation = Operation::Unknown;
            let mut second_operand: Option<i64> = None;
            let mut divisible_by: i64 = 0;
            let mut throw_to_if_true: usize = 0;
            let mut throw_to_if_false: usize = 0;

            section.lines().for_each(|line| {
                if line.starts_with("Monkey") {
                    monkey_id = line
                        .split(" ")
                        .last()
                        .unwrap()
                        .split(":")
                        .next()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap_or(0);
                } else {
                    let (key, value) = line.split_once(": ").unwrap();

                    match key {
                        "  Starting items" => {
                            items = value
                                .split(", ")
                                .map(|x| x.parse::<i64>().unwrap())
                                .collect::<Vec<i64>>();
                        }
                        "  Operation" => {
                            let statement = value.split(" = ").last().unwrap();
                            if statement.contains(" + ") {
                                operation = Operation::Add;
                                //
                                let token = statement.split(" + ").last().unwrap();
                                if !token.contains("old") {
                                    second_operand = Some(token.parse::<i64>().unwrap());
                                }
                            } else {
                                operation = Operation::Multiply;
                                let token = statement.split(" * ").last().unwrap();
                                if !token.contains("old") {
                                    second_operand = Some(token.parse::<i64>().unwrap());
                                }
                            };
                        }
                        "  Test" => {
                            divisible_by = value
                                .split("divisible by ")
                                .last()
                                .unwrap()
                                .parse::<i64>()
                                .unwrap()
                        }
                        "    If true" => {
                            throw_to_if_true = value
                                .split("throw to monkey ")
                                .last()
                                .unwrap()
                                .parse::<usize>()
                                .unwrap();
                        }
                        "    If false" => {
                            throw_to_if_false = value
                                .split("throw to monkey ")
                                .last()
                                .unwrap()
                                .parse::<usize>()
                                .unwrap();
                        }
                        _ => panic!(),
                    }
                }
            });

            Monkey {
                inspect_count: 0,
                items,
                operation,
                second_operand,
                divisible_by,
                throw_to_if_true,
                throw_to_if_false,
            }
        })
        .collect::<Vec<Monkey>>()
}

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut monkeys = parse_monkeys(contents);

    for _ in 0..20 {
        for monkey_id in 0..monkeys.len() {
            let monkey = monkeys.get(monkey_id).unwrap();
            let throws = monkey
                .items
                .iter()
                .map(|worry_level| {
                    let new_worry_level = match monkey.operation {
                        Operation::Unknown => panic!(),
                        Operation::Add => {
                            worry_level + monkey.second_operand.unwrap_or(*worry_level)
                        }
                        Operation::Multiply => {
                            worry_level * monkey.second_operand.unwrap_or(*worry_level)
                        }
                    } / 3;

                    if (new_worry_level % monkey.divisible_by) == 0 {
                        (monkey.throw_to_if_true, new_worry_level)
                    } else {
                        (monkey.throw_to_if_false, new_worry_level)
                    }
                })
                .collect::<Vec<(usize, i64)>>();

            for (throw_id, worry_level) in throws {
                monkeys.get_mut(monkey_id).unwrap().inspect_count += 1;
                monkeys.get_mut(throw_id).unwrap().items.push(worry_level);
            }

            monkeys.get_mut(monkey_id).unwrap().items.clear();
        }
    }

    monkeys
        .iter()
        .sorted_by_key(|monkey| monkey.inspect_count)
        .rev()
        .take(2)
        .fold(1, |acc, monkey| acc * monkey.inspect_count)
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut monkeys = parse_monkeys(contents);

    let common_denominator = monkeys
        .iter()
        .fold(1, |acc, monkey| acc * monkey.divisible_by);

    for _ in 0..10000 {
        for monkey_id in 0..monkeys.len() {
            let monkey = monkeys.get(monkey_id).unwrap();
            let throws = monkey
                .items
                .iter()
                .map(|worry_level| {
                    let new_worry_level = match monkey.operation {
                        Operation::Unknown => panic!(),
                        Operation::Add => {
                            worry_level + monkey.second_operand.unwrap_or(*worry_level)
                        }
                        Operation::Multiply => {
                            worry_level * monkey.second_operand.unwrap_or(*worry_level)
                        }
                    } % common_denominator;

                    if (new_worry_level % monkey.divisible_by) == 0 {
                        (monkey.throw_to_if_true, new_worry_level)
                    } else {
                        (monkey.throw_to_if_false, new_worry_level)
                    }
                })
                .collect::<Vec<(usize, i64)>>();

            for (throw_id, worry_level) in throws {
                monkeys.get_mut(monkey_id).unwrap().inspect_count += 1;
                monkeys.get_mut(throw_id).unwrap().items.push(worry_level);
            }

            monkeys.get_mut(monkey_id).unwrap().items.clear();
        }
    }

    monkeys
        .iter()
        .sorted_by_key(|monkey| monkey.inspect_count)
        .rev()
        .take(2)
        .fold(1, |acc, monkey| acc * monkey.inspect_count)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
