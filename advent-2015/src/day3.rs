use std::{fs::File, io::BufReader};

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

    todo!()
}

fn solve_part1(path: &str) -> std::io::Result<i32> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    
    todo!()
}

fn solve_part2(path: &str) -> std::io::Result<i32> {
    todo!()
}
