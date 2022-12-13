use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;
use std::env;

#[derive(Debug)]
struct ParsedToken {
    value: Option<usize>,
    list: Vec<ParsedToken>,
}

fn parse(parsed_tokens: &mut Vec<ParsedToken>, packet: &str) -> usize {
    let mut offset = 0;
    let mut start_new_token_offset = 0;
    while offset < packet.len() {
        match packet.chars().skip(offset).next() {
            Some(token) => {
                if token == '[' {
                    let start_offset = offset + 1;
                    let mut parsed_list: Vec<ParsedToken> = Vec::new();
                    offset += parse(&mut parsed_list, &packet[start_offset..]);

                    parsed_tokens.push(ParsedToken {
                        value: None,
                        list: parsed_list,
                    });
                    start_new_token_offset = offset;
                } else if token == ']' {
                    if start_new_token_offset != offset {
                        parsed_tokens.push(ParsedToken {
                            value: Some(
                                packet[start_new_token_offset..offset]
                                    .parse::<usize>()
                                    .unwrap(),
                            ),
                            list: vec![],
                        });
                    }
                    // list end
                    return offset + 2;
                } else if token == ',' {
                    if start_new_token_offset != offset {
                        parsed_tokens.push(ParsedToken {
                            value: Some(
                                packet[start_new_token_offset..offset]
                                    .parse::<usize>()
                                    .unwrap(),
                            ),
                            list: vec![],
                        });
                    }
                    // separator
                    offset += 1;
                    start_new_token_offset = offset;
                } else {
                    // neither are list
                    offset += 1;
                }
            }
            None => {} //
        }
    }
    offset
}

fn compare(lhs: &ParsedToken, rhs: &ParsedToken) -> i32 {
    if lhs.value.is_none() && rhs.value.is_none() {
        // both are list
        lhs.list
            .iter()
            .zip_longest(rhs.list.iter())
            .fold(0, |acc, items| {
                if acc != 0 {
                    return acc;
                }
                match items {
                    Both(lhs_item, rhs_item) => compare(lhs_item, rhs_item),
                    Right(_) => -1,
                    Left(_) => 1,
                }
            })
    } else if rhs.value.is_none() {
        let promoted = format!("[{}]", lhs.value.unwrap());

        let mut promoted_parsed: Vec<ParsedToken> = Vec::new();
        parse(&mut promoted_parsed, &promoted);

        compare(&promoted_parsed.first().unwrap(), rhs)
    } else if lhs.value.is_none() {
        let promoted = format!("[{}]", rhs.value.unwrap());

        let mut promoted_parsed: Vec<ParsedToken> = Vec::new();
        parse(&mut promoted_parsed, &promoted);

        compare(lhs, &promoted_parsed.first().unwrap())
    } else {
        // both have value

        if lhs.value.unwrap() == rhs.value.unwrap() {
            0
        } else if lhs.value.unwrap() < rhs.value.unwrap() {
            -1
        } else {
            1
        }
    }
}

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    contents
        .split("\n\n")
        .enumerate()
        .filter_map(|(pair_index, pair)| {
            let (lhs, rhs) = pair.split_once("\n").unwrap();

            let mut lhs_parsed: Vec<ParsedToken> = Vec::new();
            parse(&mut lhs_parsed, lhs);

            let mut rhs_parsed: Vec<ParsedToken> = Vec::new();
            parse(&mut rhs_parsed, rhs);

            match compare(&lhs_parsed.first().unwrap(), &rhs_parsed.first().unwrap()) {
                -1 => Some(pair_index + 1),
                1 => None,
                _ => panic!(),
            }
        })
        .sum()
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
