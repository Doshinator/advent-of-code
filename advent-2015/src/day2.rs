use std::fs::File;
use std::io::{self, BufRead, BufReader};

use crate::config::Config;
// Day 2
// 2x3x4 -> (axb + axc + bxc) * 2 + min(axb, axc, bxc)

pub fn solve_day2_part1(path: &str) -> io::Result<i32> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut total_paper: i32 = 0;

    for lines in reader.lines().map_while(Result::ok) {
        let nums: Vec<i32> = lines
            .split('x')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        if nums.len() == 3 {
            let (a, b, c) = (nums[0], nums[1], nums[2]);
            let side1 = a * b;
            let side2 = a * c;
            let side3 = b * c;

            let surface_area = 2 * (side1 + side2 + side3);
            let slack = side1.min(side2).min(side3);

            total_paper += surface_area + slack;
        }
    }

    Ok(total_paper)
}

pub fn solve_day2_part2(path: &str) -> io::Result<i32> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut total_ribbion = 0;

    for lines in reader.lines().map_while(Result::ok) {
        let parts: Vec<i32> = lines
            .split('x')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        if parts.len() == 3 {
            let (a, b, c) = (parts[0], parts[1], parts[2]);
            let mut sides = [a, b, c];
            sides.sort();
            
            let wrap = 2 * (sides[0] + sides[1]);
            let bow = a * b * c;

            total_ribbion += wrap + bow;
        }
    }

    Ok(total_ribbion)
}


pub fn solve(config: &Config) -> std::io::Result<()> {
    let path = config.input_path_str();

    match config.part {
        1 => {
            let total_paper = solve_day2_part1(path)?;
            println!("Total paper: {}", total_paper);
        },
        2 => {
            let total_ribbon = solve_day2_part2(path)?;
            println!("Total ribbon: {}", total_ribbon);
        },
        _ => panic!("Upsupport part {}", config.part),
    }
    Ok(())
}