use std::{fs::File, io::{BufReader, BufRead}};
use anyhow::Result;

pub struct Input {
    file: File
}

impl Input {
    pub fn new() -> Result<Self> {
        Input::from_file("input.txt")
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
