use image::ImageBuffer;
use itertools::Itertools;
use std::collections::HashMap;
use std::env;

type Coordinate = (i32, i32);
type BoulderSequence = Vec<Coordinate>;
type CavityMap = HashMap<Coordinate, usize>;

type Color = (u8, u8, u8);
fn draw_pixel(pixels: &mut Vec<(Coordinate, Color)>, position: Coordinate, color_index: i32) {
    let color = match color_index {
        100 => (219, 109, 80),
        _ => (
            (120 + color_index * 5) as u8,
            (100 + color_index * 5) as u8,
            (50 + color_index * 5) as u8,
        ),
    };

    pixels.push((position, color));
}

fn draw_map(map: &CavityMap, sand_drop: Coordinate, frame: u32) {
    let x_min = sand_drop
        .0
        .min(map.iter().map(|(pos, _)| pos.0).min().unwrap_or(0));
    let x_max = sand_drop
        .0
        .max(map.iter().map(|(pos, _)| pos.0).max().unwrap_or(0));
    let y_min = 0.min(map.iter().map(|(pos, _)| pos.1).min().unwrap_or(0));
    let y_max = sand_drop
        .1
        .max(map.iter().map(|(pos, _)| pos.1).max().unwrap_or(0));
    let x_range = (x_max - x_min) as u32;
    let y_range = (y_max - y_min) as u32;
    let start_width = 16;
    let start_height = 16;
    let dimensions: Coordinate = (
        start_width.max(1 + x_range as i32),
        start_height.max(1 + y_range as i32),
    );

    let scale = 4;
    let border = 2;
    let real_size = (
        scale * (border * 2 + dimensions.0 as u32),
        scale * (border * 2 + dimensions.1 as u32),
    );

    let mut pixels = Vec::<(Coordinate, Color)>::new();

    for (pos, id) in map.iter() {
        draw_pixel(&mut pixels, *pos, *id as i32);
    }
    draw_pixel(&mut pixels, sand_drop, 15);

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

    img.save(format!("frames/day14.frame{:05}.png", frame));
}

fn parse(contents: &String) -> CavityMap {
    let boulder_sequences = contents
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|coordinate_str| {
                    let (x, y) = coordinate_str.split_once(",").unwrap();
                    (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
                })
                .collect::<BoulderSequence>()
        })
        .collect::<Vec<BoulderSequence>>();

    let mut cavity_map: CavityMap = HashMap::new();

    boulder_sequences.iter().for_each(|boulder_sequence| {
        boulder_sequence
            .iter()
            .tuple_windows()
            .for_each(|(start, stop)| {
                let x_offset = start.0.min(stop.0);
                let y_offset = start.1.min(stop.1);
                let x_length = (start.0 - stop.0).abs() + 1;
                let y_length = (start.1 - stop.1).abs() + 1;
                for y in 0..y_length {
                    for x in 0..x_length {
                        cavity_map.insert((x_offset + x, y_offset + y), 1);
                    }
                }
            })
    });
    cavity_map
}

fn drop_sand(map: &CavityMap, start_drop: Coordinate, y_max: i32, frame: &mut u32) -> Coordinate {
    let mut sand = start_drop;
    while sand.1 < y_max {
        let below = (sand.0, sand.1 + 1);
        let diagonal_left = (sand.0 - 1, sand.1 + 1);
        let diagonal_right = (sand.0 + 1, sand.1 + 1);

        if let Some(below_id) = map.get(&below) {
            // blocked
            if let Some(left_id) = map.get(&diagonal_left) {
                // blocked
                if let Some(right_id) = map.get(&diagonal_right) {
                    // blocked
                    return sand;
                } else {
                    sand = diagonal_right;
                }
            } else {
                sand = diagonal_left;
            }
        } else {
            // not blocked
            sand = below;
        }

        if *frame > 0 {
            if *frame % 500 == 0 {
                println!("frame: {}", *frame);
            }
            draw_map(&map, sand, *frame);
            *frame += 1;
        }
    }
    return sand;
}

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut cavity_map = parse(&contents);
    let start_drop: Coordinate = (500, 0);

    // enable drawing by setting frame to 1
    let mut frame = 0;

    let y_max = cavity_map.iter().map(|(pos, _)| pos.1).max().unwrap_or(0);

    loop {
        let sand = drop_sand(&mut cavity_map, start_drop, y_max, &mut frame);
        if sand.1 < y_max {
            cavity_map.insert(sand, 10);
        } else {
            break;
        }
    }

    cavity_map.values().filter(|&id| *id == 10).count()
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut cavity_map = parse(&contents);
    let start_drop: Coordinate = (500, 0);
    // enable drawing by setting frame to 1
    let mut frame = 0;

    let y_max = 1 + cavity_map.iter().map(|(pos, _)| pos.1).max().unwrap_or(0);
    while !cavity_map.contains_key(&start_drop) {
        let sand = drop_sand(&mut cavity_map, start_drop, y_max, &mut frame);

        if sand.1 <= y_max {
            // came to rest on boulders/sand
            cavity_map.insert(sand, 10);
        } else {
            // On the floor
            cavity_map.insert(sand, 10);
        }
    }

    cavity_map.values().filter(|&id| *id == 10).count()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
