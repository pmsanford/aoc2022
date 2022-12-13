use anyhow::{anyhow, Result};
use util::Input;

enum Throw {
    Rock,
    Paper,
    Scissors,
}

enum RoundResult {
    Win,
    Draw,
    Lose,
}

impl Throw {
    fn throw_score(&self) -> usize {
        match self {
            Throw::Rock => 1,
            Throw::Paper => 2,
            Throw::Scissors => 3,
        }
    }

    fn against(&self, other: &Self) -> RoundResult {
        match (self, other) {
            (Throw::Rock, Throw::Rock) => RoundResult::Draw,
            (Throw::Rock, Throw::Paper) => RoundResult::Lose,
            (Throw::Rock, Throw::Scissors) => RoundResult::Win,
            (Throw::Paper, Throw::Rock) => RoundResult::Win,
            (Throw::Paper, Throw::Paper) => RoundResult::Draw,
            (Throw::Paper, Throw::Scissors) => RoundResult::Lose,
            (Throw::Scissors, Throw::Rock) => RoundResult::Lose,
            (Throw::Scissors, Throw::Paper) => RoundResult::Win,
            (Throw::Scissors, Throw::Scissors) => RoundResult::Draw,
        }
    }
}

impl TryFrom<&str> for Throw {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() > 1 {
            return Err(anyhow!("Unknown throw"));
        }

        match value.chars().next().unwrap() {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scissors),
            _ => Err(anyhow!("Unknown throw")),
        }
    }
}

fn main() -> Result<()> {
    let input = Input::new()?.into_lines()?;

    let mut total = 0usize;

    for round in input {
        let (opp, me) = round.split_once(' ').unwrap();
        let opp = Throw::try_from(opp)?;
        let me = Throw::try_from(me)?;

        let round_score = match me.against(&opp) {
            RoundResult::Win => 6,
            RoundResult::Draw => 3,
            RoundResult::Lose => 0,
        } + me.throw_score();

        total += round_score;
    }

    println!("Total score: {total}");

    Ok(())
}
