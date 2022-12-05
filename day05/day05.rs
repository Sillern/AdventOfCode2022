use regex::Regex;
use std::collections::VecDeque;
use std::env;

type Stacks = Vec<VecDeque<char>>;

use image::ImageBuffer;
type Coordinate = (i32, i32);
type Color = (u8, u8, u8);

fn draw_pixel(pixels: &mut Vec<(Coordinate, Color)>, position: Coordinate, color_index: usize) {
    let palette = [(255, 255, 255), (197, 203, 23), (210, 220, 12)];

    let default_color = palette[0];

    let color = match palette.get(color_index) {
        Some(valid_color) => *valid_color,
        None => default_color,
    };

    pixels.push((position, color));
}

fn draw_crate(pixels: &mut Vec<(Coordinate, Color)>, position: Coordinate, margin: u32) {
    let crate_pattern = [
        [1, 1, 1, 1, 1, 1, 1, 1],
        [1, 2, 2, 2, 2, 2, 2, 1],
        [1, 2, 2, 2, 2, 2, 2, 1],
        [1, 2, 2, 2, 2, 2, 2, 1],
        [1, 2, 2, 2, 2, 2, 2, 1],
        [1, 2, 2, 2, 2, 2, 2, 1],
        [1, 2, 2, 2, 2, 2, 2, 1],
        [1, 1, 1, 1, 1, 1, 1, 1],
    ];
    let crate_size = 8;

    for y in margin..(crate_size + 1) {
        for x in margin..(crate_size + 1) {
            let tile_pos = (
                (position.0 as u32 + x) as i32,
                (position.1 as u32 + y) as i32,
            );

            let x_pos = (x - margin) as usize;
            let y_pos = (y - margin) as usize;
            draw_pixel(pixels, tile_pos, crate_pattern[y_pos][x_pos]);
        }
    }
}

fn draw_stacks(stacks: &Stacks, gantry: u32, frame: u32) {
    let max_height = stacks.iter().map(|stack| stack.len()).max().unwrap();
    let max_length = stacks.len();

    let dimensions: Coordinate = (max_length as i32, max_height as i32);

    let border: u32 = 2;
    let margin: u32 = 1;
    let block_size: u32 = 8;
    let scale: u32 = 4;
    let real_size = (
        scale * ((block_size + margin) * dimensions.0 as u32 + margin + border * 2),
        scale * ((block_size + margin) * dimensions.1 as u32 + margin + border * 2),
    );

    // Translate value to a color from a palette
    let mut pixels = Vec::<(Coordinate, Color)>::new();

    for depth in 0..max_height {
        for (stack_index, stack) in stacks.iter().enumerate() {
            let index = max_height - depth;
            let crate_pos = (
                border + ((block_size + margin) * stack_index as u32),
                border + ((block_size + margin) * depth as u32),
            );
            if index <= stack.len() {
                draw_crate(
                    &mut pixels,
                    (crate_pos.0 as i32, crate_pos.1 as i32),
                    margin,
                );
            }
        }
    }

    let mut img = ImageBuffer::from_fn(real_size.0, real_size.1, |_x, _y| {
        image::Rgb([255, 255, 255])
    });

    for ((x, y), color) in pixels {
        let pixel = image::Rgb([color.0, color.1, color.2]);
        if x >= 0 && y >= 0 && x < real_size.0 as i32 && y < real_size.1 as i32 {
            for offset_y in 0..scale {
                for offset_x in 0..scale {
                    img.put_pixel(
                        scale * x as u32 + offset_x,
                        scale * y as u32 + offset_y,
                        pixel,
                    );
                }
            }
        }
    }

    img.save(format!("frames/day05.frame{:05}.png", frame));
}

fn print_stacks(stacks: &Stacks) {
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

    let mut stacks: Stacks = Vec::new();
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

    let mut frame = 0;
    draw_stacks(&stacks, 0, frame);
    frame += 1;

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
            draw_stacks(&stacks, 0, frame);
            frame += 1;
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
