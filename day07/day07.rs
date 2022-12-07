use itertools::Itertools;
use std::env;

fn get_directory_size(filesystem_flat: &Vec<(String, usize)>, start_path: &str) -> usize {
    filesystem_flat
        .iter()
        .filter(|(path, _)| {
            if path != start_path {
                let path_tokens = path.split("/").collect::<Vec<&str>>();
                let path_tokens_length = path_tokens.len();
                let path_part = path_tokens[..path_tokens_length - 1].join("/");

                path_part == start_path
            } else {
                false
            }
        })
        .map(|(path, size)| {
            if *size == 0 {
                get_directory_size(filesystem_flat, &path)
            } else {
                *size
            }
        })
        .sum()
}

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut filesystem_flat: Vec<(String, usize)> = Vec::new();
    let mut current_path: Vec<&str> = Vec::new();

    contents.lines().for_each(|line| {
        if line.starts_with("$") {
            if line.starts_with("$ cd") {
                let directory = line.get("$ cd ".len()..).unwrap();
                if directory == ".." {
                    current_path.pop();
                } else {
                    current_path.push(directory);
                }
            }
        } else {
            if line.starts_with("dir ") {
                let directory = line.get("dir ".len()..).unwrap();
                current_path.push(directory);
                filesystem_flat.push((current_path.join("/"), 0));
                current_path.pop();
            } else {
                let (filesize_str, filename) = line.split_once(" ").unwrap();
                let filesize = filesize_str.parse::<usize>().unwrap();
                current_path.push(filename);
                filesystem_flat.push((current_path.join("/"), filesize));
                current_path.pop();
            }
        }
    });

    filesystem_flat
        .iter()
        .filter_map(|(path, size)| {
            if *size == 0 {
                let folder_size = get_directory_size(&filesystem_flat, &path);
                if folder_size < 100000 {
                    return Some(folder_size);
                }
            }
            return None;
        })
        .sum()
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut filesystem_flat: Vec<(String, usize)> = Vec::new();
    let mut current_path: Vec<&str> = Vec::new();

    let available_space = 70000000;
    let needed_space = 30000000;
    let root = "/";
    filesystem_flat.push((String::from(root), 0));

    contents.lines().for_each(|line| {
        if line.starts_with("$") {
            if line.starts_with("$ cd") {
                let directory = line.get("$ cd ".len()..).unwrap();
                if directory == ".." {
                    current_path.pop();
                } else {
                    current_path.push(directory);
                }
            }
        } else {
            if line.starts_with("dir ") {
                let directory = line.get("dir ".len()..).unwrap();
                current_path.push(directory);
                filesystem_flat.push((current_path.join("/"), 0));
                current_path.pop();
            } else {
                let (filesize_str, filename) = line.split_once(" ").unwrap();
                let filesize = filesize_str.parse::<usize>().unwrap();
                current_path.push(filename);
                filesystem_flat.push((current_path.join("/"), filesize));
                current_path.pop();
            }
        }
    });

    let used_space = filesystem_flat
        .iter()
        .find_map(|(path, _)| {
            if path == root {
                Some(get_directory_size(&filesystem_flat, path))
            } else {
                None
            }
        })
        .unwrap();

    let current_space = available_space - used_space;

    filesystem_flat
        .iter()
        .filter_map(|(path, size)| {
            if *size == 0 {
                let folder_size = get_directory_size(&filesystem_flat, &path);
                if (current_space + folder_size) > needed_space {
                    return Some(folder_size);
                }
            }
            return None;
        })
        .sorted()
        .next()
        .unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
