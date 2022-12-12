use std::collections::HashMap;
use std::env;

type Coordinate = (i32, i32);
type HeightMap = HashMap<Coordinate, usize>;

fn get_shortest_path(map: &HeightMap, start: Coordinate, end: Coordinate) -> Option<usize> {
    let mut queue: Vec<((Coordinate, usize), Vec<Coordinate>)> =
        Vec::from([((start, 0), vec![start])]);

    let mut visited: Vec<Coordinate> = Vec::new();

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

                if position == end {
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
