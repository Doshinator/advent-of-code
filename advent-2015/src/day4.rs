use md5::{Digest, Md5};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::config::Config;

pub fn solve(config: &Config) -> std::io::Result<()> {
    let input: &str = "iwrupvqb";

    match config.part {
        1 => {
            let result = solve_part1(input);
            println!("Part 1 result: {}", result);
        },
        2 => {
            let result = solve_part2(input);
            println!("Part 2 result: {}", result);
        },
        _ => eprintln!("Day {} Part {} not implemented", config.day, config.part),
    }

    Ok(())
}

fn solve_part1(key: &str) -> i32 {
    let mut counter = 1;
    loop {
        let mut hash = Md5::new();
        hash.update(format!("{}{}", key, counter));
        let result = hash.finalize();
        
        // each byte (u8) represents 2 hex char; we're checking first 2.5 byte -> 5 char
        if result[0] == 0 && result[1] == 0 && (result[2] & 0xf0) == 0 {
            return counter;
        }
        counter += 1;
    }
}

fn solve_part2(key: &str) -> u32 {
    let ans = (1..u32::MAX)
        .into_par_iter()
        .find_first(|&n| {
            let mut hash = Md5::new();
            hash.update(format!("{}{}", key, n));
            let result = hash.finalize();
            result[0] == 0 && result[1] == 0 && result[2] == 0
        });

    ans.unwrap()
}