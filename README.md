# Advent of Code 2015 - Modular Architecture

## Overview

This project uses a clean, modular architecture where each day is self-contained in its own module. The `app.rs` file stays minimal by delegating to each day's `solve()` function.

## Structure

### Config (`config.rs`)
- Holds day, part, and input path
- `Config::new(day, part)` - Creates config with default path pattern (`resources/dayX.txt`)
- `Config::input_path_str()` - Returns the input file path as a string

### App (`app.rs`)
- Minimal dispatcher (~20 lines total)
- Routes to the appropriate day module's `solve()` function
- Stays clean even with 25 days implemented

### Day Modules (`day1.rs`, `day2.rs`, etc.)
- Each day has a `pub fn solve(config: &Config) -> io::Result<()>` function
- All day-specific logic lives in the day module (reading, parsing, solving)
- Part 1 and Part 2 logic handled within each day's `solve()` function

## Usage

### Running Solutions

```bash
# Run Day 1 Part 1
cargo run -- 1 1

# Run Day 2 Part 2
cargo run -- 2 2

# Run Day 3 Part 1
cargo run -- 3 1
```

### Main Entry Point (`main.rs`)

```rust
use avdent_2015::{app::App, config::Config};
use std::env;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    
    let (day, part) = if args.len() >= 3 {
        let day = args[1].parse::<u8>().unwrap_or(1);
        let part = args[2].parse::<u8>().unwrap_or(1);
        (day, part)
    } else {
        println!("Usage: {} <day> <part>", args[0]);
        println!("Defaulting to Day 1 Part 1");
        (1, 1)
    };

    let config = Config::new(day, part);
    let app = App::new(config);
    app.run()?;
    
    Ok(())
}
```

### App Dispatcher (`app.rs`)

The app file remains minimal - just routing to day modules:

```rust
use crate::config::Config;
use crate::{day1, day2, day3};
use std::io;

pub struct App {
    config: Config,
}

impl App {
    pub fn new(config: Config) -> Self {
        App { config }
    }

    pub fn run(&self) -> Result<(), io::Error> {
        println!("Running Day {} Part {}", self.config.day, self.config.part);

        match self.config.day {
            1 => day1::solve(&self.config),
            2 => day2::solve(&self.config),
            3 => day3::solve(&self.config),
            _ => {
                eprintln!("Day {} not yet implemented.", self.config.day);
                Ok(())
            }
        }
    }
}
```

## Adding a New Day

### Step 1: Create `dayX.rs`

Each day module follows this template:

```rust
use crate::config::Config;
use std::io;

// Day X - [Problem description]

pub fn solve(config: &Config) -> io::Result<()> {
    let path = config.input_path_str();

    match config.part {
        1 => {
            let result = solve_part1(path)?;
            println!("Part 1 result: {}", result);
        }
        2 => {
            let result = solve_part2(path)?;
            println!("Part 2 result: {}", result);
        }
        _ => eprintln!("Part {} not implemented for Day X", config.part),
    }

    Ok(())
}

fn solve_part1(path: &str) -> io::Result<i32> {
    // TODO: Implement part 1
    todo!()
}

fn solve_part2(path: &str) -> io::Result<i32> {
    // TODO: Implement part 2
    todo!()
}
```

### Step 2: Add to `lib.rs`

```rust
pub mod app;
pub mod config;
pub mod day1;
pub mod day2;
pub mod dayX;  // Add this line
```

### Step 3: Add to `app.rs` match statement

```rust
match self.config.day {
    1 => day1::solve(&self.config),
    2 => day2::solve(&self.config),
    3 => day3::solve(&self.config),
    X => dayX::solve(&self.config),  // Add this line
    _ => { ... }
}
```

That's it! Each new day requires:
- One new file (`dayX.rs`)
- One line in `lib.rs`
- One line in `app.rs`

## File Reading Patterns

### For character-by-character input (Day 1, Day 3)

```rust
use std::fs;

fn solve_part1(path: &str) -> io::Result<i32> {
    let contents = fs::read_to_string(path)?;
    
    for ch in contents.chars() {
        match ch {
            '(' => { /* ... */ }
            ')' => { /* ... */ }
            _ => {}
        }
    }
    
    Ok(result)
}
```

### For line-by-line input (Day 2)

```rust
use std::fs::File;
use std::io::{BufRead, BufReader};

fn solve_part1(path: &str) -> io::Result<i32> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut total = 0;
    
    for line in reader.lines().map_while(Result::ok) {
        // Process each line
        let nums: Vec<i32> = line.split('x')
            .filter_map(|s| s.parse().ok())
            .collect();
        
        if let [a, b, c] = nums[..] {
            total += a * b * c;
        }
    }
    
    Ok(total)
}
```

## Project Structure

```
advent-2015/
├── Cargo.toml
├── src/
│   ├── main.rs          (CLI entry point)
│   ├── lib.rs           (Module declarations)
│   ├── app.rs           (Dispatcher - stays ~20 lines)
│   ├── config.rs        (Config struct)
│   ├── day1.rs          (Day 1 complete solution)
│   ├── day2.rs          (Day 2 complete solution)
│   ├── day3.rs          (Day 3 complete solution)
│   └── ...
└── resources/
    ├── day1.txt         (Same file for both parts)
    ├── day2.txt
    ├── day3.txt
    └── ...
```

## Key Principles

1. **Each day is self-contained** - All logic for Day X lives in `dayX.rs`
2. **Same input for both parts** - In AoC, part 1 and part 2 use the same input file
3. **Minimal app.rs** - Just dispatches to the appropriate day module
4. **Consistent pattern** - Every day follows the same `solve(config)` structure
5. **Easy to extend** - Adding a new day is just 3 lines of boilerplate

## Benefits

- **Scalability**: `app.rs` stays ~20 lines even with 25 days
- **Organization**: Day 15's code is in `day15.rs`, not buried in `app.rs`
- **Maintainability**: Each day can be understood independently
- **Simplicity**: Clear, consistent pattern for every day
- **No duplication**: File reading logic is per-day, not centralized (more flexible)

## Testing

Each day module can include its own tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        // Test with example from problem description
        assert_eq!(solve_example(), expected_result);
    }
}
```