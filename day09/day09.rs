use image::ImageBuffer;
use std::collections::HashMap;
use std::env;

type Coordinate = (i32, i32);

fn is_touching(head: Coordinate, tail: Coordinate) -> bool {
    return [
        (0, 0),
        (0, 1),
        (0, -1),
        (1, 0),
        (1, 1),
        (1, -1),
        (-1, 0),
        (-1, 1),
        (-1, -1),
    ]
    .iter()
    .any(|&(x, y)| (tail.0 + x, tail.1 + y) == head);
}

type Space = HashMap<Coordinate, i32>;

type Color = (u8, u8, u8);

fn draw_pixel(pixels: &mut Vec<(Coordinate, Color)>, position: Coordinate, color_index: i32) {
    let color = match color_index {
        0 => (219, 109, 80),
        1 => (166, 145, 80),
        2 => (177, 157, 94),
        3 => (186, 168, 111),
        4 => (194, 178, 128),
        5 => (202, 188, 145),
        6 => (211, 199, 162),
        7 => (219, 209, 180),
        _ => (219, 209, 180),
    };

    pixels.push((position, color));
}

fn draw_map(map: &Space, rope: &Vec<Coordinate>, frame: u32) {
    let rope_x_min = rope.iter().map(|pos| pos.0).min().unwrap();
    let rope_x_max = rope.iter().map(|pos| pos.0).max().unwrap();
    let rope_y_min = rope.iter().map(|pos| pos.1).min().unwrap();
    let rope_y_max = rope.iter().map(|pos| pos.1).max().unwrap();
    let x_min = rope_x_min.min(map.iter().map(|(pos, _)| pos.0).min().unwrap_or(0));
    let x_max = rope_x_max.max(map.iter().map(|(pos, _)| pos.0).max().unwrap_or(0));
    let y_min = rope_y_min.min(map.iter().map(|(pos, _)| pos.1).min().unwrap_or(0));
    let y_max = rope_y_max.max(map.iter().map(|(pos, _)| pos.1).max().unwrap_or(0));
    let x_range = (x_max - x_min) as u32;
    let y_range = (y_max - y_min) as u32;
    let dimensions: Coordinate = (32.max(1 + x_range as i32), 32.max(1 + y_range as i32));

    let scale = 8;
    let border = 2;
    let real_size = (
        scale * (border * 2 + dimensions.0 as u32),
        scale * (border * 2 + dimensions.1 as u32),
    );

    let mut pixels = Vec::<(Coordinate, Color)>::new();

    for (pos, visited_count) in map.iter() {
        draw_pixel(&mut pixels, *pos, *visited_count);
    }
    for pos in rope.iter() {
        draw_pixel(&mut pixels, *pos, 0);
    }

    let mut img = ImageBuffer::from_fn(real_size.0, real_size.1, |_x, _y| {
        image::Rgb([255, 255, 255])
    });

    for ((x, y), color) in pixels {
        let pixel = image::Rgb([color.0, color.1, color.2]);
        let draw_pos = (
            scale as i32 * (border as i32 + (x - x_min)),
            scale as i32 * (border as i32 + (y - y_min)),
        );

        if draw_pos.0 >= 0
            && draw_pos.1 >= 0
            && draw_pos.0 < real_size.0 as i32
            && draw_pos.1 < real_size.1 as i32
        {
            for offset_y in 0..scale {
                for offset_x in 0..scale {
                    img.put_pixel(
                        draw_pos.0 as u32 + offset_x,
                        draw_pos.1 as u32 + offset_y,
                        pixel,
                    );
                }
            }
        }
    }

    img.save(format!("frames/day09.frame{:05}.png", frame));
}

fn pull_rope(
    space: &mut Space,
    rope: &mut Vec<Coordinate>,
    direction: Coordinate,
    amount: i32,
    frame: &mut u32,
) {
    for _ in 0..amount {
        let mut previous = *rope.first().unwrap();
        for (index, section) in rope.iter_mut().enumerate() {
            if index == 0 {
                // head
                *section = (section.0 + direction.0, section.1 + direction.1);
            } else {
                // tail-sections
                if is_touching(previous, *section) {
                }
                // same column
                else if previous.0 == section.0 {
                    *section = (
                        section.0,
                        if previous.1 > section.1 {
                            section.1 + 1
                        } else {
                            section.1 - 1
                        },
                    );
                }
                // same row
                else if previous.1 == section.1 {
                    *section = (
                        if previous.0 > section.0 {
                            section.0 + 1
                        } else {
                            section.0 - 1
                        },
                        section.1,
                    );
                } else {
                    // move diagonally
                    *section = (
                        if previous.0 > section.0 {
                            section.0 + 1
                        } else {
                            section.0 - 1
                        },
                        if previous.1 > section.1 {
                            section.1 + 1
                        } else {
                            section.1 - 1
                        },
                    );
                }
            }
            previous = *section;
        }

        space
            .entry(*rope.last().unwrap())
            .and_modify(|e| *e += 1)
            .or_insert(1);

        if *frame > 0 {
            draw_map(&space, &rope, *frame);
            *frame += 1;
        }
    }
}

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut space: Space = Space::new();
    let mut rope = vec![(0, 0), (0, 0)];
    let mut frame = 0;

    contents.lines().for_each(|line| {
        let (direction_str, amount_str) = line.split_once(" ").unwrap();
        let amount = amount_str.parse::<i32>().unwrap();

        let direction: (i32, i32) = match direction_str {
            "L" => (-1, 0),
            "R" => (1, 0),
            "U" => (0, -1),
            "D" => (0, 1),
            _ => panic!(),
        };

        pull_rope(&mut space, &mut rope, direction, amount, &mut frame);
    });

    // draw_map(&space, head, frame);

    space.iter().count()
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut space: Space = Space::new();
    let mut rope = vec![
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ];
    let mut frame = 0;

    draw_map(&space, &rope, 0);

    contents.lines().for_each(|line| {
        let (direction_str, amount_str) = line.split_once(" ").unwrap();
        let amount = amount_str.parse::<i32>().unwrap();

        let direction: (i32, i32) = match direction_str {
            "L" => (-1, 0),
            "R" => (1, 0),
            "U" => (0, -1),
            "D" => (0, 1),
            _ => panic!(),
        };

        pull_rope(&mut space, &mut rope, direction, amount, &mut frame);
    });

    draw_map(&space, &rope, frame);

    space.iter().count()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
