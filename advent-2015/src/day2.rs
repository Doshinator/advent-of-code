use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Result};
use std::path::Path;
// Day 2
// 2x3x4 -> (axb + axc + bxc) * 2 + min(axb, axc, bxc)

pub fn file_path_lines(path: &str) {
    if let Ok(lines) = read_lines(path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            println!("{}", line);
        }
    }
}

pub fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
