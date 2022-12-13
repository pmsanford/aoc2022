use std::{fs::File, io::{BufReader, BufRead}, env};
use anyhow::Result;

pub struct Input {
    file: File
}

impl Input {
    pub fn new() -> Result<Self> {
        if env::args().any(|arg| arg == "--test") {
            Input::from_file("test.txt")
        } else {
            Input::from_file("input.txt")
        }
    }

    pub fn from_file(filename: &str) -> Result<Self> {
        let file = File::open(filename)?;

        Ok(Self {
            file
        })
    }

    pub fn into_lines(self) -> Result<Vec<String>> {
        let reader = BufReader::new(self.file);

        let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

        Ok(lines)
    }
}
