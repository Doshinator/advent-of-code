use md5::{Digest, Md5};

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

        if result[0] == 0 && result[1] == 0 && (result[2] & 0xf0) == 0 {
            return counter;
        }
        counter += 1;
    }
}

fn solve_part2(key: &str) -> i32 {
    todo!()
}