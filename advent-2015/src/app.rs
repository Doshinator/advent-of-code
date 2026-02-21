use crate::config::Config;
use crate::day1;
use crate::day2;
use crate::day3;
use crate::day4;
use crate::day5;

pub struct App {
    config: Config,
}

impl App {
    pub fn new(config: Config) -> Self {
        App {
            config
        }
    }

    pub fn run(&self) -> Result<(), std::io::Error> {
        println!(
            "Running Day {} Part {}", 
            self.config.day,
            self.config.part,
        );

        match self.config.day {
            1 => day1::solve(&self.config),
            2 => day2::solve(&self.config),
            3 => day3::solve(&self.config),
            4 => day4::solve(&self.config),
            5 => day5::solve(&self.config),
            _ => {
                eprintln!("Day {} Part {} not yet implemented.", self.config.day, self.config.part);
                Ok(())
            }
        }
    }
}