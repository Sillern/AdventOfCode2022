use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;
use std::cmp::Ordering;
use std::env;

#[derive(Debug)]
pub struct ParsedToken {
    pub value: Option<usize>,
    pub list: Vec<ParsedToken>,
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

fn compare(lhs: &ParsedToken, rhs: &ParsedToken) -> Ordering {
    if lhs.value.is_none() && rhs.value.is_none() {
        // both are list
        lhs.list
            .iter()
            .zip_longest(rhs.list.iter())
            .fold(Ordering::Equal, |acc, items| {
                if acc != Ordering::Equal {
                    return acc;
                }
                match items {
                    Both(lhs_item, rhs_item) => compare(lhs_item, rhs_item),
                    Right(_) => Ordering::Less,
                    Left(_) => Ordering::Greater,
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
            Ordering::Equal
        } else if lhs.value.unwrap() < rhs.value.unwrap() {
            Ordering::Less
        } else {
            Ordering::Greater
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
                Ordering::Less => Some(pair_index + 1),
                Ordering::Greater => None,
                Ordering::Equal => panic!(),
            }
        })
        .sum()
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut divider_packets: Vec<ParsedToken> = Vec::new();
    for packet in ["[[2]]", "[[6]]"] {
        if packet != "\n" {
            parse(&mut divider_packets, packet);
        }
    }

    let mut packets: Vec<ParsedToken> = Vec::new();
    contents.lines().for_each(|packet| {
        if packet != "\n" {
            parse(&mut packets, packet);
        }
    });

    packets.sort_by(|a, b| compare(a, b));
    packets
        .iter()
        .enumerate()
        .filter_map(|(index, packet)| {
            divider_packets.iter().find_map(|divider_packet| {
                match compare(&packet, &divider_packet) {
                    Ordering::Less => None,
                    Ordering::Greater => None,
                    Ordering::Equal => Some(index + 1),
                }
            })
        })
        .product()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
