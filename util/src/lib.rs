use anyhow::Result;
use std::{env, fs};

pub mod linked_grid;

pub struct Input {
    filename: String,
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

impl Input {
    pub fn new() -> Self {
        if env::args().any(|arg| arg == "--test") {
            Input::from_file("test.txt")
        } else {
            Input::from_file("input.txt")
        }
    }

    pub fn from_file(filename: &str) -> Self {
        Self {
            filename: filename.to_owned(),
        }
    }

    pub fn into_lines(self) -> Result<Vec<String>> {
        Ok(fs::read_to_string(self.filename)?
            .lines()
            .map(str::to_owned)
            .collect())
    }

    pub fn into_string(self) -> Result<String> {
        Ok(fs::read_to_string(self.filename)?)
    }
}
