use std::collections::HashSet;

use anyhow::Result;
use geo::{line_intersection::line_intersection, Coord, Line};
use util::{
    grid::{Grid, SparseGrid},
    Input,
};

mod parser {
    use std::num::ParseFloatError;

    use anyhow::{bail, Result};
    use geo::Coord;
    use nom::{
        bytes::complete::{tag, take_while1},
        character::is_digit,
        combinator::map_res,
        IResult,
    };

    fn from_dec(i: &str) -> Result<f64, ParseFloatError> {
        i.parse()
    }

    fn number(i: &str) -> IResult<&str, f64> {
        map_res(
            take_while1(|c: char| is_digit(c as u8) || c == '-'),
            from_dec,
        )(i)
    }

    fn xy(i: &str) -> IResult<&str, Coord<f64>> {
        let (i, _) = tag("x=")(i)?;
        let (i, x) = number(i)?;
        let (i, _) = tag(", y=")(i)?;
        let (i, y) = number(i)?;

        Ok((i, (x, y).into()))
    }

    #[allow(clippy::type_complexity)]
    fn reading(i: &str) -> IResult<&str, (Coord<f64>, Coord<f64>)> {
        let (i, _) = tag("Sensor at ")(i)?;
        let (i, sensor) = xy(i)?;
        let (i, _) = tag(": closest beacon is at ")(i)?;
        let (i, beacon) = xy(i)?;

        Ok((i, (sensor, beacon)))
    }

    pub fn parse_reading(i: &str) -> Result<(Coord<f64>, Coord<f64>)> {
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
}

fn manhattan(from: Coord<f64>, to: Coord<f64>) -> f64 {
    (from.x - to.x).abs() + (from.y - to.y).abs()
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
    }));

    for (sensor, beacon) in input.iter() {
        grid.set((sensor.x as isize, sensor.y as isize), Entity::Sensor);
        grid.set((beacon.x as isize, beacon.y as isize), Entity::Beacon);
    }

    let lines = input
        .iter()
        .map(|(sensor, beacon)| (sensor, manhattan(*sensor, *beacon)))
        .flat_map(|(sensor, distance)| {
            vec![
                Line::new(
                    (sensor.x + distance, sensor.y),
                    (sensor.x, sensor.y + distance),
                ),
                Line::new(
                    (sensor.x, sensor.y + distance),
                    (sensor.x - distance, sensor.y),
                ),
                Line::new(
                    (sensor.x - distance, sensor.y),
                    (sensor.x, sensor.y - distance),
                ),
                Line::new(
                    (sensor.x, sensor.y - distance),
                    (sensor.x + distance, sensor.y),
                ),
            ]
        })
        .collect::<Vec<_>>();

    let mut intersections = HashSet::<Coord<isize>>::new();

    for line in lines.iter() {
        for other in lines.iter() {
            if line == other {
                continue;
            }
            match line_intersection(*line, *other) {
                Some(geo::LineIntersection::SinglePoint { intersection, .. }) => {
                    intersections.insert((intersection.x as isize, intersection.y as isize).into());
                }
                Some(geo::LineIntersection::Collinear { intersection }) => {
                    let start = intersection.start;
                    let end = intersection.end;
                    intersections.insert((start.x as isize, start.y as isize).into());
                    intersections.insert((end.x as isize, end.y as isize).into());
                }
                None => {}
            };
        }
    }

    let max = 4_000_000;

    let mut intersections = intersections.into_iter().collect::<Vec<_>>();
    intersections.sort_by_key(|i| i.x);

    'i: for intersection in intersections {
        for x in (intersection.x - 1).max(0)..=(intersection.x + 1).min(max) {
            'y: for y in (intersection.y - 1).max(0)..=(intersection.y + 1).min(max) {
                for (sensor, distance) in input
                    .iter()
                    .map(|(sensor, beacon)| (sensor, manhattan(*sensor, *beacon)))
                {
                    let dist_to = manhattan(*sensor, (x as f64, y as f64).into());
                    if dist_to <= distance {
                        continue 'y;
                    }
                }
                println!("Found {x}, {y}");
                println!("Tuning freq: {}", (x * 4_000_000) + y);
                break 'i;
            }
        }
    }

    /*
    for line in grid.draw(grid.get_bounds()) {
        println!("{}", line);
    }
    */

    println!("Bounds: {:?}", grid.get_bounds());

    Ok(())
}
