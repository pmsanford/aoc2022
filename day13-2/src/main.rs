use std::fmt::Display;

use anyhow::Result;
use list::{ListParser, Rule};
use pest::{iterators::Pair, Parser};
use util::Input;

mod list {
    use pest_derive::Parser;

    #[derive(Parser)]
    #[grammar = "list.pest"]
    pub struct ListParser;
}

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

fn parse_list(outer: Pair<Rule>) -> Result<Packet> {
    let mut packets = vec![];

    for pair in outer.into_inner() {
        match pair.as_rule() {
            Rule::number => packets.push(Packet::Number(pair.as_str().parse()?)),
            Rule::WHITESPACE => {}
            Rule::digit => unreachable!(),
            Rule::list => packets.push(parse_list(pair)?),
        }
    }

    Ok(Packet::List(packets))
}

fn main() -> Result<()> {
    let input = Input::new()?
        .into_lines()?
        .drain(..)
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    let mut packets = vec![];

    for line in input {
        let parsed = ListParser::parse(Rule::list, &line)?;

        let outer = parsed.into_iter().next().unwrap();

        packets.push(parse_list(outer)?);
    }
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
