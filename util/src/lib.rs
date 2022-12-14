use anyhow::Result;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Read},
};

pub struct Input {
    file: File,
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

        Ok(Self { file })
    }

    pub fn into_lines(self) -> Result<Vec<String>> {
        let reader = BufReader::new(self.file);

        let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

        Ok(lines)
    }

    pub fn into_string(mut self) -> Result<String> {
        let mut s = String::new();
        self.file.read_to_string(&mut s)?;
        let s = s.trim().to_owned();

        Ok(s)
    }
}
