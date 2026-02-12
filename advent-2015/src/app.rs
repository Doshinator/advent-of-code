use crate::config::Config;
use crate::day1;

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
            1 => self.run_day1(),
            2 => self.run_day2(),
            _ => {
                eprintln!("Day {} not yet implemented.", self.config.day);
                Ok(())
            }
        }
    }

    fn run_day1(&self) -> std::io::Result<()> {
        let data = day1::read_input(self.config.input_path_str())?;

        match self.config.part {
            1 => {
                let floor = day1::parse_parenthese(&data);
                println!("{} floor", floor);
            },
            2 => {
                if let Some(index) = day1::first_position_less_than_zero(&data) {
                    println!("first position {}", index + 1);
                }
            },
            _ => eprintln!("Part {} not implemented for Day 1", self.config.part),
        }

        Ok(())
    }

    fn run_day2(&self) -> std::io::Result<()> {
        Ok(())
    }
}