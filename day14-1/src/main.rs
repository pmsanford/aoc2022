use anyhow::Result;
use util::{linked_grid::LinkedGrid, Input};

mod parser {
    use anyhow::{bail, Result};
    use nom::{
        bytes::complete::{tag, take_while1},
        character::is_digit,
        multi::separated_list1,
        sequence::separated_pair,
        IResult,
    };

    fn number(i: &str) -> IResult<&str, usize> {
        let (rest, number) = take_while1(|c: char| is_digit(c as u8))(i)?;

        Ok((rest, number.parse().unwrap()))
    }

    fn pair(i: &str) -> IResult<&str, (usize, usize)> {
        separated_pair(number, tag(","), number)(i)
    }

    pub fn parse_line(i: &str) -> Result<Vec<(usize, usize)>> {
        match separated_list1(tag(" -> "), pair)(i) {
            Ok(("", result)) => Ok(result),
            Ok((_, _)) => bail!("Didn't parse all input"),
            Err(e) => bail!("Failed to parse input: {e}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Contents {
    Empty,
    Rock,
    Sand,
}

fn main() -> Result<()> {
    let input = Input::new().into_lines()?;

    let pairs = input
        .iter()
        .map(|p| parser::parse_line(p))
        .collect::<Result<Vec<_>>>()?;

    println!("Pairs: \n{pairs:#?}");

    let max_x = *pairs.iter().flatten().map(|(x, _)| x).max().unwrap() + 1;
    let max_y = *pairs.iter().flatten().map(|(_, y)| y).max().unwrap() + 5;

    let mut lg = LinkedGrid::new(max_x, max_y, |x, y| {
        for rocks in &pairs {
            for line in rocks.windows(2) {
                let lx = line[0].0.min(line[1].0);
                let ly = line[0].1.min(line[1].1);
                let mx = line[0].0.max(line[1].0);
                let my = line[0].1.max(line[1].1);
                if x >= lx && x <= mx && y >= ly && y <= my {
                    return Contents::Rock;
                }
            }
        }
        Contents::Empty
    });

    lg.draw_range((493, 0), 11, 13, |c| match c {
        Contents::Empty => '.',
        Contents::Rock => '#',
        Contents::Sand => 'o',
    });

    for x in 0isize..max_x as isize {
        for y in 0isize..max_y as isize {
            let _ = lg.try_link((x, y), (x - 1, y + 1));
            let _ = lg.try_link((x, y), (x, y + 1));
            let _ = lg.try_link((x, y), (x + 1, y + 1));
        }
    }

    let mut sand = (500, 0);

    let mut resting = 0;

    loop {
        println!("sand: {sand:?}");
        if sand.1 == max_y - 1 {
            println!("Off the bottom!");
            break;
        }
        let mut neighbors = lg.neighbors(sand);

        neighbors.sort_by_key(|p| p.x);

        if neighbors[1].data == Contents::Empty {
            sand = (neighbors[1].x, neighbors[1].y);
            continue;
        }
        if neighbors[0].data == Contents::Empty {
            sand = (neighbors[0].x, neighbors[0].y);
            continue;
        }
        if neighbors[2].data == Contents::Empty {
            sand = (neighbors[2].x, neighbors[2].y);
            continue;
        }
        lg.set_data(sand.0, sand.1, Contents::Sand)?;
        resting += 1;
        sand = (500, 0);
    }

    lg.draw_range((493, 0), 11, 13, |c| match c {
        Contents::Empty => '.',
        Contents::Rock => '#',
        Contents::Sand => 'o',
    });

    println!("Resting: {resting}");

    Ok(())
}
