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
    rope: Vec<(isize, isize)>,
    visited: HashSet<(isize, isize)>,
}

impl State {
    fn new(len: usize) -> Self {
        Self {
            rope: vec![(0, 0); len],
            visited: HashSet::from([(0, 0)]),
        }
    }

    fn do_move(&mut self, dir: Direction) {
        let head = &mut self.rope[0];
        match dir {
            Direction::Right => head.0 += 1,
            Direction::Left => head.0 -= 1,
            Direction::Up => head.1 -= 1,
            Direction::Down => head.1 += 1,
        }
        for i in 1..self.rope.len() {
            let head = self.rope[i - 1];
            let tail = &mut self.rope[i];
            let delta = (head.0 - tail.0, head.1 - tail.1);
            // Touching
            if delta.0.abs() <= 1 && delta.1.abs() <= 1 {
                return;
            }
            // Same column, vertical move necessary
            if delta.0.abs() == 2 && delta.1 == 0 {
                tail.0 += delta.0 / delta.0.abs();
            }
            // Same row, horizontal move necessary
            else if delta.1.abs() == 2 && delta.0 == 0 {
                tail.1 += delta.1 / delta.1.abs();
            }
            // Diagonal move necessary
            else {
                tail.0 += delta.0 / delta.0.abs();
                tail.1 += delta.1 / delta.1.abs();
            }
        }
        self.visited.insert(*self.rope.last().unwrap());
    }
}

fn main() -> Result<()> {
    let input = Input::new()?
        .into_lines()?
        .into_iter()
        .map(parse_move)
        .collect::<Result<Vec<_>>>()?;
    let mut state = State::new(2);
    for (dir, count) in input {
        for _ in 0..count {
            state.do_move(dir);
        }
    }

    println!("Tail visisted {} locations.", state.visited.len());

    Ok(())
}
