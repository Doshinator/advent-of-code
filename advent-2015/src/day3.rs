use std::fs;
use std::{collections::HashSet, io::BufReader};
use std::io::{BufRead};

use crate::config::Config;

pub fn solve(config: &Config) -> std::io::Result<()> {
    let path = config.input_path_str();

    match config.part {
        1 => {
            let result = solve_part1(path)?;
            println!("Part 1 result: {}", result);
        },
        2 => {
            let result = solve_part2(path)?;
            println!("Part 2 result: {}", result);
        },
        _ => eprintln!("Day {} Part {} not implemented", config.day, config.part),
    }

    Ok(())
}

fn solve_part1(path: &str) -> std::io::Result<i32> {
    let file = fs::read_to_string(path)?;

    let mut house = (0, 0);
    let mut houses: HashSet<(i32, i32)> = HashSet::new();
    houses.insert(house);
    
    for ch in file.chars() {
        match ch {
            '^' => house.1 += 1,
            'v' => house.1 -= 1,
            '>' => house.0 += 1,
            '<' => house.0 -= 1,
            _ => {},
        }
        houses.insert(house);
    }

    Ok(houses.len() as i32)
}

fn solve_part2(path: &str) -> std::io::Result<i32> {
    todo!()
}
