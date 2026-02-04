use avdent_2015::day1::{first_position_less_than_zero, parse_parenthese, read_input};

fn main() -> Result<(), std::io::Error> {
    let path = format!(
        "{}/resources/day1.txt",
        env!("CARGO_MANIFEST_DIR")
    );
    let data = read_input(&path)?;
    
    // --- Part 1
    let floor = parse_parenthese(&data);
    println!("Floor:{}", floor);

    // --- Part 2
    if let Some(index) = first_position_less_than_zero(&data) {
        println!("First position that's in the basement: {}", index + 1);
    }

    Ok(())
}
