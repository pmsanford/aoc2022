use anyhow::Result;
use util::{
    grid::{Grid, SparseGrid},
    Input,
};

mod parser {
    use std::num::ParseIntError;

    use anyhow::{bail, Result};
    use nom::{
        bytes::complete::{tag, take_while1},
        character::is_digit,
        combinator::map_res,
        IResult,
    };

    fn from_dec(i: &str) -> Result<isize, ParseIntError> {
        i.parse()
    }

    fn number(i: &str) -> IResult<&str, isize> {
        map_res(
            take_while1(|c: char| is_digit(c as u8) || c == '-'),
            from_dec,
        )(i)
    }

    fn xy(i: &str) -> IResult<&str, (isize, isize)> {
        let (i, _) = tag("x=")(i)?;
        let (i, x) = number(i)?;
        let (i, _) = tag(", y=")(i)?;
        let (i, y) = number(i)?;

        Ok((i, (x, y)))
    }

    #[allow(clippy::type_complexity)]
    fn reading(i: &str) -> IResult<&str, ((isize, isize), (isize, isize))> {
        let (i, _) = tag("Sensor at ")(i)?;
        let (i, sensor) = xy(i)?;
        let (i, _) = tag(": closest beacon is at ")(i)?;
        let (i, beacon) = xy(i)?;

        Ok((i, (sensor, beacon)))
    }

    pub fn parse_reading(i: &str) -> Result<((isize, isize), (isize, isize))> {
        match reading(i) {
            Ok((rest, reading)) => {
                if !rest.is_empty() {
                    bail!("Didn't consume all input. Remaining: {}", rest);
                }
                Ok(reading)
            }
            Err(e) => {
                bail!("Parse error: {}", e);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum Entity {
    #[default]
    Empty,
    Sensor,
    Beacon,
    Covered,
}

fn manhattan(from: (isize, isize), to: (isize, isize)) -> isize {
    (from.0 - to.0).abs() + (from.1 - to.1).abs()
}

fn main() -> Result<()> {
    let input = Input::new()
        .into_lines()?
        .into_iter()
        .map(|s| parser::parse_reading(&s))
        .collect::<Result<Vec<_>>>()?;

    let mut grid = SparseGrid::<Entity>::new(Box::new(|e| match e {
        Entity::Empty => '.',
        Entity::Sensor => 'S',
        Entity::Beacon => 'B',
        Entity::Covered => '#',
    }));

    let line_to_count = 2000000;

    for (sensor, beacon) in input {
        let dist = manhattan(sensor, beacon);
        let y_range = (sensor.1 - dist)..=(sensor.1 + dist);
        if y_range.contains(&line_to_count) {
            for x in (sensor.0 - dist)..=(sensor.0 + dist) {
                if manhattan(sensor, (x, line_to_count)) <= dist
                    && grid.get((x, line_to_count)) != Some(&Entity::Beacon)
                {
                    grid.set((x, line_to_count), Entity::Covered);
                }
            }
        }
        grid.set(sensor, Entity::Sensor);
        grid.set(beacon, Entity::Beacon);
    }

    println!("Bounds: {:?}", grid.get_bounds());

    let ((min_x, _), (max_x, _)) = grid.get_bounds();

    let covered = (min_x..=max_x)
        .into_iter()
        .filter(|x| grid.get((*x, line_to_count)) == Some(&Entity::Covered))
        .count();

    println!("Covered: {covered}");

    Ok(())
}
