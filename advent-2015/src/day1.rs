// Day1 Part1
// ( = +1
// ) = -1
// keep i64 and return final answer

use std::{fs, io};
use crate::config::Config;

#[derive(Debug, Clone, Copy)]
pub enum Parenthesis {
    Open,
    Closed,
}

impl TryFrom<char> for Parenthesis {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(Parenthesis::Open),
            ')' => Ok(Parenthesis::Closed),
            _ => Err(()),
        }
    }
}

pub struct Data {
    pub vector: Vec<Parenthesis>,
}

pub fn read_input(path: &str) -> Result<Data, std::io::Error> {
    let contents = fs::read_to_string(path)
        .map_err(|e| {
            io::Error::new(e.kind(),format!("{}: {}", path, e))
        })?;

    let vector = contents
        .chars()
        .filter_map(|c| Parenthesis::try_from(c).ok())
        .collect();

    Ok(Data { vector })
}

pub fn parse_parenthese(input: &Data) -> i64 {
    let mut floor: i64 = 0;
    for p in input.vector.iter() {
        match p {
            Parenthesis::Open => floor += 1,
            Parenthesis::Closed => floor -= 1,
        }
    }
    floor
}

// part 2
// enumerate to find the first position floor < 0
pub fn first_position_less_than_zero(input: &Data) -> Option<usize> {
    let mut floor: i64 = 0;
    for (i, p) in input.vector.iter().enumerate() {
        match p {
            Parenthesis::Open => floor += 1,
            Parenthesis::Closed => floor -= 1,
        }
        if floor < 0 {
            return Some(i);
        }
    }

    None
}

pub fn solve(config: &Config) -> io::Result<()> {
    let data = read_input(config.input_path_str())?;
        match config.part {
            1 => {
                let floor = parse_parenthese(&data);
                println!("{} floor", floor);
            },
            2 => {
                if let Some(index) = first_position_less_than_zero(&data) {
                    println!("first position {}", index + 1);
                }
            },
            _ => eprintln!("Day {} Part {} not implemented", config.day, config.part),
        }

    Ok(())
}