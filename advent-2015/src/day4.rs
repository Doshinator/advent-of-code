use crate::config::Config;

pub fn solve(config: &Config) -> std::io::Result<()> {
    let path = config.input_path_str();

    match config.day {
        1 => {
            let result = solve_part1(path);
            println!("Part 1 result: {}", result);
        },
        2 => {
            let result = solve_part2(path);
            println!("Part 2 result: {}", result);
        },
        _ => eprintln!("Day {} Part {} not implemented", config.day, config.part),
    }

    Ok(())
}

fn solve_part1(path: &str) -> i32 {
    todo!()
}

fn solve_part2(path: &str) -> i32 {
    todo!()
}