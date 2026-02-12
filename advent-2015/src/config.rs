use std::path::PathBuf;

pub struct Config {
    pub day: u8,
    pub part: u8,
    pub input_path: PathBuf,
}

impl Config {
    pub fn new(day: u8, part: u8) -> Self {
        let input_path = PathBuf::from(format!(
            "{}/resources/day{}.txt",
            env!("CARGO_MANIFEST_DIR"),
            day,
        ));

        Config { 
            day, 
            part, 
            input_path 
        }
    }

    pub fn input_path_str(&self) -> &str {
        self.input_path.to_str().unwrap_or("")
    }
}