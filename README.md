# Advent of Code 2015 - Config Architecture

## Overview

This project uses a clean config-based architecture that eliminates the need to reimplement file reading for each day.

## Structure

### Config (`config.rs`)
- Holds day, part, and input path
- `Config::new(day, part)` - Creates config with default path pattern
- `Config::with_custom_path(day, part, path)` - Creates config with custom path

### App (`app.rs`)
- Main application runner
- `App::new(config)` - Creates app with config
- `app.run()` - Dispatches to the appropriate day/part implementation

### Day Modules (`day1.rs`, `day2.rs`, etc.)
- Each day has its own module with solution functions
- Keep the data reading and parsing logic within each day module

## Usage

### CLI Usage

```rust
use avdent_2015::{app::App, config::Config};
use std::env;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    
    let day = args[1].parse::<u8>().unwrap_or(1);
    let part = args[2].parse::<u8>().unwrap_or(1);

    let config = Config::new(day, part);
    let app = App::new(config);
    app.run()?;
    
    Ok(())
}
```

Then run with: `cargo run -- 1 2` for Day 1 Part 2

## Adding a New Day

1. Create `dayX.rs` with your solution functions
2. Add `pub mod dayX;` to `lib.rs`
3. Add a case in `App::run()`:

```rust
match self.config.day {
    1 => self.run_day1(),
    2 => self.run_day2(),
    X => self.run_dayX(),  // Add this
    _ => { ... }
}
```

4. Implement `run_dayX()` method in `app.rs`:

```rust
fn run_dayX(&self) -> Result<(), io::Error> {
    let data = dayX::read_input(self.config.input_path_str())?;
    
    match self.config.part {
        1 => {
            let result = dayX::solve_part1(&data);
            println!("Result: {}", result);
        }
        2 => {
            let result = dayX::solve_part2(&data);
            println!("Result: {}", result);
        }
        _ => eprintln!("Part {} not implemented for Day X", self.config.part),
    }
    
    Ok(())
}
```

## Benefits

- No need to reimplement file reading each day
- Easy to switch between days and parts
- Clean separation of concerns
- Consistent error handling
- Easy to extend with CLI arguments or configuration files
