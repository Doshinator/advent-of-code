use crate::config::Config;

pub fn solve(config: &Config) -> std::io::Result<()> {
    match config.part {
        1 => {
            let result = solve_part_1();
            println!("Day {} Part {} result: {}", config.day, config.part, result);
        },
        2 => {
            let result = solve_part_2 ();
            println!("Day {} Part {} result: {}", config.day, config.part, result);
        },
        _ => panic!("Day {} Part {} not implemented", config.day, config.part),
    }
    Ok(())
}

fn solve_part_1() -> i32 {
    todo!()
}

fn solve_part_2() -> i32 {
    todo!()
}