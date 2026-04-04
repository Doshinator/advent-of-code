use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use rayon::string;

use crate::config::Config;

pub fn solve(config: &Config) -> std::io::Result<()> {
    let part = config.part;
    let path = config.input_path_str();
    match part {
        1 => {
            let result = solve_day7_part1(path)?;
            println!("Result: {}", result);
        },
        2 => {
            println!("Result:");
            let result = solve_day7_part2();
        },
        _ => panic!("Invalid part: {}", config.part),
    }

    Ok(())
}

fn solve_day7_part1(path: &str) -> std::io::Result<u16> {
    let lines = read_lines(path)?;
    let mut wires: HashMap<String, Expr> = HashMap::new();
    for line in lines {
        let line = line?;
        let instr = parse_line(&line);
        wires.insert(instr.target, instr.expr);
    }

    Ok(0101)
}

fn solve_day7_part2() {
    todo!()
}


// wire -> gate, wire, or specific value

// 1 source can send signal to -> wire (1 to 1)
// wire send signal to -> multiple sources
// gate -> destination - only when it has input from all source

// ex. x AND y -> z = connect wires x and y and an AND gate, then connects its output to z
// <expr> -> <wire>

struct Instruction {
    expr: Expr,
    target: String,
}

enum Expr {
    Value(u16),
    Wire(String),

    And(String, String),
    Or(String, String),

    Not(String),

    LShift(String, u16),
    RShift(String, u16),
}

fn parse_line(line: &str) -> Instruction {
    let parts: Vec<&str> = line.split(" -> ").collect();
    let expr_part = parts[0];
    let target = parts[1].to_string();
    
    let tokens: Vec<&str> = expr_part.split_whitespace().collect();

    let expr = match tokens.as_slice() {
        [a] => {
            if let Ok(num) = a.parse::<u16>() {
                Expr::Value(num)
            }
            else {
                Expr::Wire(a.to_string())
            }
        },
        ["NOT", a] => Expr::Not(a.to_string()),
        [a, "AND", b] => Expr::And(a.to_string(), b.to_string()),
        [a, "OR", b] => Expr::Or(a.to_string(), b.to_string()),
        [a, "LSHIFT", b] => Expr::LShift(a.to_string(), b.parse().unwrap()),
        [a, "RSHIFT", b] => Expr::RShift(a.to_string(), b.parse().unwrap()),
        _ => panic!("Uknown pattern"),
    };

    Instruction {
        expr,
        target, 
    }
}

fn eval() {
    todo!()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}