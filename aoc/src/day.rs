use std::fmt::Display;

use anyhow::{bail, Context};

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum Part {
    One,
    Two,
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Part::One => "1",
                Part::Two => "2",
            }
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Day {
    pub day: usize,
    pub part: Part,
}

impl Day {
    pub fn next_day(&self) -> Day {
        match self.part {
            Part::One => Day {
                day: self.day,
                part: Part::Two,
            },
            Part::Two => Day {
                day: self.day + 1,
                part: Part::One,
            },
        }
    }
}

impl Default for Day {
    fn default() -> Self {
        Self {
            day: 1,
            part: Part::One,
        }
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r"day{:0>2}-{}", self.day, self.part)
    }
}

impl TryFrom<String> for Day {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !value.starts_with("day") {
            bail!("Doesn't start with day");
        }
        if value.len() != 7 {
            bail!("Format doesn't match");
        }
        let day = value
            .chars()
            .skip(3)
            .take(2)
            .collect::<String>()
            .parse::<usize>()
            .context("couldn't parse day number")?;
        let part = value
            .chars()
            .skip(6)
            .take(1)
            .collect::<String>()
            .parse::<usize>()
            .context("couldn't parse part number")?;

        if part != 1 && part != 2 {
            bail!("Part should be 1 or 2, not {}", part);
        }

        let part = if part == 1 { Part::One } else { Part::Two };

        if !(1..=25).contains(&day) {
            bail!("Day should be in [1, 25], not {}", day);
        }

        Ok(Day { day, part })
    }
}

impl PartialOrd for Day {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.day.partial_cmp(&other.day) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.part.partial_cmp(&other.part)
    }
}

impl Ord for Day {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
