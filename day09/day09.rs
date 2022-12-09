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

struct CoordinateRange {
    head: Coordinate,
    tail: Coordinate,
    current: Option<(Coordinate, Coordinate)>,
    direction: Coordinate,
    amount: i32,
}

impl CoordinateRange {
    fn new(
        head: Coordinate,
        tail: Coordinate,
        direction: Coordinate,
        amount: i32,
    ) -> CoordinateRange {
        CoordinateRange {
            head,
            tail,
            current: None,
            direction,
            amount,
        }
    }
}

impl Iterator for CoordinateRange {
    type Item = (Coordinate, Coordinate);
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.current {
            None => {
                self.current = Some((self.head, self.tail));
                self.current
            }
            Some(current) => {
                let head = current.0;
                let mut tail = current.1;
                if self.amount == 0 && is_touching(head, tail) {
                    return None;
                }

                let next_head = (head.0 + self.direction.0, head.1 + self.direction.1);
                self.amount -= 1;

                if is_touching(next_head, tail) {
                    self.current = Some((next_head, tail));
                }
                // same column
                else if next_head.0 == tail.0 {
                    self.current = Some((next_head, (tail.0, tail.1 + self.direction.1)));
                }
                // same row
                else if next_head.1 == tail.1 {
                    self.current = Some((next_head, (tail.0 + self.direction.0, tail.1)));
                } else {
                    // move diagonally
                    if next_head.0 > tail.0 {
                        tail.0 += 1;
                    } else {
                        tail.0 -= 1;
                    }

                    if next_head.1 > tail.1 {
                        tail.1 += 1;
                    } else {
                        tail.1 -= 1;
                    }

                    self.current = Some((next_head, tail));
                }

                self.current
            }
        }
    }
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

fn draw_map(map: &Space, head: Coordinate, frame: u32) {
    let x_min = map.iter().map(|(pos, _)| pos.0).min().unwrap();
    let x_max = map.iter().map(|(pos, _)| pos.0).max().unwrap();
    let y_min = map.iter().map(|(pos, _)| pos.1).min().unwrap();
    let y_max = map.iter().map(|(pos, _)| pos.1).max().unwrap();
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
    draw_pixel(&mut pixels, head, 0);

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

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut space: Space = Space::new();
    let mut head = (0, 0);
    let mut tail = head.clone();
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

        for (new_head, new_tail) in
            CoordinateRange::new(head.clone(), tail.clone(), direction, amount)
        {
            space.entry(new_tail).and_modify(|e| *e += 1).or_insert(1);
            // draw_map(&space, new_head, frame);
            frame += 1;
            head = new_head;
            tail = new_tail;
        }
    });

    // draw_map(&space, head, frame);

    space.iter().count()
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
