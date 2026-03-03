use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::config::Config;

pub fn solve(config: &Config) -> std::io::Result<()> {
    let path = config.input_path_str();

    match config.part {
        1 => {
            let result = solve_part_1(path)?;
            println!("Day {} Part {}. Total nice strings:{}", config.day, config.part, result);
        },
        2 => {
            let result = solve_part_2 (path)?;
            println!("Day {} Part {} result: {}", config.day, config.part, result);
        },
        _ => panic!("Day {} Part {} not implemented", config.day, config.part),
    }
    Ok(())
}

fn solve_part_1(path: &str) -> std::io::Result<i32> {
    let lines = read_lines(path)?;
    let result = lines
        .map_while(Result::ok)
        .filter(|line| nice_str_part_1(line))
        .count() as i32;
        
    Ok(result)
}

fn solve_part_2(path: &str) -> std::io::Result<i32> {
    let lines = read_lines(path)?;
    let result = lines
        .map_while(Result::ok)
        .filter(|line| nice_str_part_2(line))
        .count() as i32;
    Ok(result)
}

fn nice_str_part_1(s: &str) -> bool {
    if !contains_three_vowels(s) {
        return false;
    }

    let mut has_double = false;
    
    for (a, b) in s.chars().zip(s.chars().skip(1)) {
        // Forbidden pairs
        if matches!((a, b), 
            ('a','b') | ('c','d') | ('p','q') | ('x','y')) {
            return false;
        }
        if a == b {
            has_double = true;
        }
    }

    has_double
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn contains_three_vowels(s: &str) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u',];
    s.chars()
        .filter(|c| vowels.contains(c))
        .count() >= 3
}

fn nice_str_part_2(s: &str) -> bool {
    has_repeated_pair(s) && has_repeat_with_gap(s)
}

fn has_repeat_with_gap(s: &str) -> bool {
    s.as_bytes()
        .windows(3)
        .any(|w| w[0] == w[2])
}

fn has_repeated_pair(s: &str) -> bool {
    let bytes = s.as_bytes();
    let mut seen: HashMap<(u8, u8), usize> = HashMap::new();
    for i in 0..bytes.len().saturating_sub(1) {
        let pair = (bytes[i], bytes[i + 1]);

        if let Some(&prev_index) = seen.get(&pair) {
            if i >= prev_index + 2 {
                return true;
            }
        } else {
            seen.insert(pair, i);
        }
    }

    false
}