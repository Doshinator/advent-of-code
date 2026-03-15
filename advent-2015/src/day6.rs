use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::config::Config;

pub fn solve(config: &Config) -> std::io::Result<()> {
    let path = config.input_path_str();

    match config.part {
        1 => {
            let result = solve_part_1(&path)?;
            println!("Lights on: {}", result);
        },
        2 => {
            let result = solve_part_2(&path)?;
            println!("Total brightness: {}", result);
        },
        _ => panic!("Invalid part {}", config.part),
    }

    Ok(())
}

fn solve_part_1(path: &str) -> std::io::Result<usize> {
    let lines = read_lines(path)?;
    let mut grid = vec![vec![false; 1000]; 1000];
    for line in lines {
        let line = line?;
        let instruction = parse_lines(&line);
        apply_instruction(instruction, &mut grid);
    }

    Ok(count_lights(&grid))
}

fn solve_part_2(path: &str) -> std::io::Result<u32> {
    let lines = read_lines(path)?;
    let mut grid = vec![vec![0u32; 1000]; 1000];

    for line in lines {
        let line = line?;
        let instruction = parse_lines(&line);
        apply_instruction_part2(instruction, &mut grid);
    }

    Ok(count_brightness(&grid))
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

enum Instruction {
    Toggle ((usize, usize), (usize, usize)),
    On ((usize, usize), (usize, usize)),
    Off ((usize, usize), (usize, usize)),
}

fn parse_lines(line: &str) -> Instruction {
    let parts: Vec<&str> = line.split_whitespace().collect();

    match parts.as_slice() {
        ["toggle", start, "through", end] => {
            Instruction::Toggle(parse_coord(start), parse_coord(end))
        },
        ["turn", "on", start, "through", end] => {
            Instruction::On(parse_coord(start), parse_coord(end))
        }

        ["turn", "off", start, "through", end] => {
            Instruction::Off(parse_coord(start), parse_coord(end))
        }
        _ => panic!("Invalid instruction"),
    }
}

fn parse_coord(s: &str) -> (usize, usize) {
    let (x, y) = s.split_once(',').unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

fn apply_instruction(instruction: Instruction, grid: &mut Vec<Vec<bool>>) {
    match instruction {
        Instruction::Toggle(start, end) => {
            apply_region(grid, start, end, |light| *light = !*light);
        },
        Instruction::On(start, end) => {
            apply_region(grid, start, end, |light| *light = true);
        },
        Instruction::Off(start, end) => {
            apply_region(grid, start, end, |light| *light = false);
        },
    }
}

fn apply_instruction_part2(instruction: Instruction, grid: &mut Vec<Vec<u32>>) {
    match instruction {
        Instruction::Toggle(start, end) => {
            apply_region(grid, start, end, |light| *light += 2);
        },
        Instruction::On(start, end) => {
            apply_region(grid, start, end, |light| *light += 1);
        },
        Instruction::Off(start, end) => {
            apply_region(grid, start, end, |light| {
                if *light > 0 {
                    *light -= 1;
                }
            });
        },
    }
}

fn apply_region<T, F>(
    grid: &mut Vec<Vec<T>>,
    start: (usize, usize),
    end: (usize, usize),
    mut op: F,
)
where
    F: FnMut(&mut T),
{
    let (x1, y1) = start;
    let (x2, y2) = end;

    for x in x1..=x2 {
        for y in y1..=y2 {
            op(&mut grid[x][y]);
        }
    }
}

fn count_lights(grid: &Vec<Vec<bool>>) -> usize {
    grid
        .iter()
        .flatten()
        .filter(|&&light| light == true)
        .count()
}

fn count_brightness(grid: &Vec<Vec<u32>>) -> u32 {
    grid
        .iter()
        .flatten()
        .sum()
}