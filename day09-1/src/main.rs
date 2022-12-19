use std::collections::HashSet;

use anyhow::{anyhow, bail, Result};
use util::Input;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

fn parse_move(line: String) -> Result<(Direction, usize)> {
    let (dir, num) = line
        .split_once(' ')
        .ok_or_else(|| anyhow!("couldn't parse direction"))?;

    let dir = match dir {
        "R" => Direction::Right,
        "L" => Direction::Left,
        "U" => Direction::Up,
        "D" => Direction::Down,
        _ => bail!("unknown direction"),
    };

    Ok((dir, num.parse()?))
}

struct State {
    head: (isize, isize),
    tail: (isize, isize),
    visited: HashSet<(isize, isize)>,
}

impl State {
    fn new() -> Self {
        Self {
            head: (0, 0),
            tail: (0, 0),
            visited: HashSet::from([(0, 0)]),
        }
    }

    fn do_move(&mut self, dir: Direction) {
        match dir {
            Direction::Right => self.head.0 += 1,
            Direction::Left => self.head.0 -= 1,
            Direction::Up => self.head.1 -= 1,
            Direction::Down => self.head.1 += 1,
        }
        let delta = (self.head.0 - self.tail.0, self.head.1 - self.tail.1);
        // Touching
        if delta.0.abs() <= 1 && delta.1.abs() <= 1 {
            return;
        }
        // Same column, vertical move necessary
        if delta.0.abs() == 2 && delta.1 == 0 {
            self.tail.0 += delta.0 / delta.0.abs();
        }
        // Same row, horizontal move necessary
        else if delta.1.abs() == 2 && delta.0 == 0 {
            self.tail.1 += delta.1 / delta.1.abs();
        }
        // Diagonal move necessary
        else {
            self.tail.0 += delta.0 / delta.0.abs();
            self.tail.1 += delta.1 / delta.1.abs();
        }
        self.visited.insert(self.tail);
    }
}

fn main() -> Result<()> {
    let input = Input::new()
        .into_lines()?
        .into_iter()
        .map(parse_move)
        .collect::<Result<Vec<_>>>()?;
    let mut state = State::new();
    for (dir, count) in input {
        for _ in 0..count {
            state.do_move(dir);
        }
    }

    println!("Tail visisted {} locations.", state.visited.len());

    Ok(())
}
