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
