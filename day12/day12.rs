use image::ImageBuffer;
use std::collections::HashMap;
use std::env;

type Coordinate = (i32, i32);
type HeightMap = HashMap<Coordinate, usize>;

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

fn draw_map(map: &HeightMap, path: &Vec<Coordinate>, frame: u32) {
    let path_x_min = path.iter().map(|pos| pos.0).min().unwrap();
    let path_x_max = path.iter().map(|pos| pos.0).max().unwrap();
    let path_y_min = path.iter().map(|pos| pos.1).min().unwrap();
    let path_y_max = path.iter().map(|pos| pos.1).max().unwrap();
    let x_min = path_x_min.min(map.iter().map(|(pos, _)| pos.0).min().unwrap_or(0));
    let x_max = path_x_max.max(map.iter().map(|(pos, _)| pos.0).max().unwrap_or(0));
    let y_min = path_y_min.min(map.iter().map(|(pos, _)| pos.1).min().unwrap_or(0));
    let y_max = path_y_max.max(map.iter().map(|(pos, _)| pos.1).max().unwrap_or(0));
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

    for (pos, height) in map.iter() {
        draw_pixel(&mut pixels, *pos, *height as i32);
    }
    for pos in path.iter() {
        draw_pixel(&mut pixels, *pos, 100);
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

    img.save(format!("frames/day12.frame{:05}.png", frame));
}

fn get_shortest_path(map: &HeightMap, start: Coordinate, end: Coordinate) -> Option<usize> {
    let mut queue: Vec<((Coordinate, usize), Vec<Coordinate>)> =
        Vec::from([((start, 0), vec![start])]);

    let mut visited: Vec<Coordinate> = Vec::new();

    let mut frame = 0;
    while !queue.is_empty() {
        // sort by length
        queue.sort_by_key(|((_, height), path)| (26 - (*height as usize)) * path.len());
        queue.reverse();

        match queue.pop() {
            Some(((position, elevation), came_from)) => {
                if visited.contains(&position) {
                    continue;
                } else {
                    visited.push(position.clone());
                }

                draw_map(map, &came_from, frame);
                frame += 1;

                if position == end {
                    draw_map(map, &came_from, frame);
                    return Some(came_from.len() - 1);
                } else {
                    [(0, -1), (0, 1), (-1, 0), (1, 0)].iter().for_each(|step| {
                        let neighbour = (position.0 + step.0, position.1 + step.1);
                        match map.get(&neighbour) {
                            Some(height) => {
                                if (elevation >= *height) || (elevation + 1 == *height) {
                                    if !came_from.contains(&neighbour) {
                                        let mut new_came_from = came_from.clone();
                                        new_came_from.push(neighbour.clone());
                                        queue.push(((neighbour, *height), new_came_from));
                                    }
                                }
                            }
                            None => {}
                        }
                    });
                }
            }
            None => (),
        }
    }

    None
}

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut height_map: HeightMap = HashMap::new();

    let mut start_pos: Coordinate = (0, 0);
    let mut end_pos: Coordinate = (0, 0);

    contents.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, height)| {
            let pos: Coordinate = (x as i32, y as i32);
            let height_value = match height {
                'S' => {
                    start_pos = pos;
                    ('a' as usize) - ('a' as usize)
                }
                'E' => {
                    end_pos = pos;
                    ('z' as usize) - ('a' as usize)
                }
                _ => (height as usize) - ('a' as usize),
            };

            height_map.insert(pos, height_value);
        })
    });

    get_shortest_path(&height_map, start_pos, end_pos).unwrap()
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut height_map: HeightMap = HashMap::new();

    let mut start_positions: Vec<Coordinate> = Vec::new();
    let mut end_pos: Coordinate = (0, 0);

    contents.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, height)| {
            let pos: Coordinate = (x as i32, y as i32);
            let height_value = match height {
                'S' => {
                    start_positions.push(pos);
                    ('a' as usize) - ('a' as usize)
                }
                'E' => {
                    end_pos = pos;
                    ('z' as usize) - ('a' as usize)
                }
                'a' => {
                    start_positions.push(pos);
                    0
                }
                _ => (height as usize) - ('a' as usize),
            };

            height_map.insert(pos, height_value);
        })
    });

    start_positions.iter().fold(0, |acc, start_pos| {
        match get_shortest_path(&height_map, *start_pos, end_pos) {
            Some(shortest_path) => {
                if acc == 0 || shortest_path < acc {
                    println!("path from {:?} is {} steps", start_pos, shortest_path);
                    shortest_path
                } else {
                    acc
                }
            }
            None => acc,
        }
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
