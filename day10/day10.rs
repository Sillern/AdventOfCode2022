use std::env;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    AddX,
    Unknown,
}

type DecodedInstruction = ((Instruction, usize), Option<i64>);
struct CPU {
    instructions: Vec<DecodedInstruction>,
    x: i64,
    pc: usize,
    tick: usize,
    delay: usize,
}

impl CPU {
    pub fn new(program: String, start_value: i64) -> Self {
        let instructions = program
            .lines()
            .map(|line| {
                let mut tokens = line.split(" ");
                let instruction_opt = tokens.next().unwrap();
                let value_opt = match tokens.next() {
                    Some(value) => Some(value.parse::<i64>().unwrap()),
                    None => None,
                };

                match instruction_opt {
                    "noop" => ((Instruction::Noop, 1), None),
                    "addx" => ((Instruction::AddX, 2), value_opt),
                    _ => ((Instruction::Unknown, 0), None),
                }
            })
            .collect::<Vec<DecodedInstruction>>();

        Self {
            instructions,
            x: start_value,
            pc: 0,
            tick: 0,
            delay: 0,
        }
    }

    pub fn value(&self) -> i64 {
        self.x
    }

    pub fn tick(&mut self) {
        let ((instruction, cycle_count), value_opt) = self.instructions[self.pc];
        if self.delay == cycle_count {
            match instruction {
                Instruction::Noop => {}
                Instruction::AddX => self.x += value_opt.unwrap(),
                Instruction::Unknown => panic!(),
            }
            self.delay = 1;
            self.pc += 1;
        } else {
            self.delay += 1;
        }
    }
}

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut cpu = CPU::new(contents, 1);

    let mut result = 0;
    let thresholds = vec![20, 60, 100, 140, 180, 220];
    for tick in 0..221 {
        if thresholds.contains(&tick) {
            result += tick * cpu.value();
        }

        cpu.tick();
    }
    result as usize
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut cpu = CPU::new(contents, 1);
    let mut pixels: Vec<char> = Vec::new();
    let width = 40;
    let height = 6;
    for tick in 0..(width * height) {
        cpu.tick();
        let xpos = cpu.value();
        let pixel_pos = tick % width;
        let sprite = [xpos - 1, xpos, xpos + 1];

        pixels.push(if sprite.contains(&pixel_pos) {
            '#'
        } else {
            '.'
        });
    }

    pixels.iter().enumerate().for_each(|(index, value)| {
        print!("{}", value);
        if (index + 1) % width as usize == 0 {
            println!();
        }
    });
    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
