use std::fmt::Display;

use anyhow::{bail, Result};
use util::Input;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Number(usize),
    List(Vec<Packet>),
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Number(n) => write!(f, "{n}"),
            Packet::List(l) => {
                write!(f, "[")?;
                for i in 0..l.len() {
                    write!(f, "{}", l[i])?;
                    if i < l.len() - 1 {
                        write!(f, ",")?;
                    }
                }
                write!(f, "]")
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Packet::Number(l), Packet::Number(r)) => l.partial_cmp(r),
            (Packet::Number(l), Packet::List(_)) => {
                let l = Packet::List(vec![Packet::Number(*l)]);
                l.partial_cmp(other)
            }
            (Packet::List(_), Packet::Number(r)) => {
                let r = Packet::List(vec![Packet::Number(*r)]);
                self.partial_cmp(&r)
            }
            (Packet::List(l), Packet::List(r)) => {
                for i in 0..l.len().min(r.len()) {
                    match l[i].cmp(&r[i]) {
                        std::cmp::Ordering::Equal => {}
                        o => return Some(o),
                    }
                }

                Some(l.len().cmp(&r.len()))
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn number(i: &str) -> nom::IResult<&str, Packet> {
    let (rest, number) =
        nom::bytes::complete::take_while1(|c: char| nom::character::is_digit(c as u8))(i)?;

    Ok((rest, Packet::Number(number.parse().unwrap())))
}

fn list_item(i: &str) -> nom::IResult<&str, Packet> {
    nom::branch::alt((number, list))(i)
}

fn inside_list(i: &str) -> nom::IResult<&str, Vec<Packet>> {
    nom::multi::separated_list0(nom::bytes::complete::tag(","), list_item)(i)
}

fn list(i: &str) -> nom::IResult<&str, Packet> {
    let (rest, parsed) = nom::sequence::delimited(
        nom::bytes::complete::tag("["),
        inside_list,
        nom::bytes::complete::tag("]"),
    )(i)?;

    Ok((rest, Packet::List(parsed)))
}

fn parse_line(i: &str) -> Result<Packet> {
    match list(i) {
        Ok((rest, parsed)) => {
            if !rest.is_empty() {
                bail!("Didn't parse whole line");
            }

            Ok(parsed)
        }
        Err(e) => {
            bail!("Failed to parse: {}", e);
        }
    }
}

fn main() -> Result<()> {
    let input = Input::new()?
        .into_lines()?
        .drain(..)
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    let mut packets = input
        .iter()
        .map(|p| parse_line(p))
        .collect::<Result<Vec<_>>>()?;

    let one = Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
    packets.push(one.clone());
    let two = Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);
    packets.push(two.clone());

    packets.sort();

    let mut dividers = vec![];

    for (idx, packet) in packets.into_iter().enumerate() {
        if packet == one || packet == two {
            dividers.push(idx + 1);
        }
        println!("{packet}");
    }

    println!(
        "dividers: {dividers:?} product: {}",
        dividers.iter().product::<usize>()
    );

    Ok(())
}
