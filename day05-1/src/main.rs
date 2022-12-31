use anyhow::Result;
use util::Input;

use crate::parser::parse_move;

mod parser {
    use anyhow::{anyhow, Result};
    use std::num::ParseIntError;

    #[derive(Debug, Clone, Copy)]
    pub struct Move {
        pub count: usize,
        pub from: usize,
        pub to: usize,
    }

    use nom::{
        bytes::complete::{tag, take_while1},
        character::is_digit,
        combinator::{all_consuming, map_res},
        IResult,
    };

    fn from_dec(i: &str) -> Result<usize, ParseIntError> {
        i.parse()
    }

    fn number(i: &str) -> IResult<&str, usize> {
        map_res(
            take_while1(|c: char| is_digit(c as u8) || c == '-'),
            from_dec,
        )(i)
    }

    fn mv(i: &str) -> IResult<&str, Move> {
        let (i, _) = tag("move ")(i)?;
        let (i, count) = number(i)?;
        let (i, _) = tag(" from ")(i)?;
        let (i, from) = number(i)?;
        let (i, _) = tag(" to ")(i)?;
        let (i, to) = number(i)?;

        Ok((i, Move { count, from, to }))
    }

    pub fn parse_move(i: &str) -> Result<Move> {
        let (_, mv) = all_consuming(mv)(i).map_err(|_| anyhow!("Couldn't parse move"))?;

        Ok(mv)
    }
}

fn print_stacks(stacks: &Vec<Vec<char>>) {
    let height = stacks.iter().map(|s| s.len()).max().unwrap();
    let count = stacks.len();

    for i in (0..height).rev() {
        for stack in stacks.iter().take(count) {
            if let Some(v) = stack.get(i) {
                print!(r"[{v}] ");
            } else {
                print!("    ");
            }
        }
        println!();
    }
    for i in 1..=count {
        print!(r" {i}  ");
    }
    println!();
}

fn main() -> Result<()> {
    let lines = Input::new().into_lines()?;

    let stack_count = (lines[0].len() + 1) / 4;
    let mut stacks = vec![vec![]; stack_count];

    let mut instructions_start = 0;

    for (no, line) in lines.iter().enumerate() {
        let chars = line.chars().collect::<Vec<char>>();
        if chars[1].is_ascii_digit() {
            instructions_start = no + 2;
            break;
        }
        for (stack_num, stack) in stacks.iter_mut().enumerate().take(stack_count) {
            let idx = 1 + stack_num * 4;
            let c = chars[idx];
            if c != ' ' {
                stack.push(c);
            }
        }
    }

    for ref mut stack in &mut stacks {
        stack.reverse();
    }

    print_stacks(&stacks);

    for inst in lines.iter().skip(instructions_start) {
        println!("{inst}");
        let mv = parse_move(inst)?;

        for _ in 0..mv.count {
            let popped = stacks[mv.from - 1].pop().unwrap();
            stacks[mv.to - 1].push(popped);
        }

        print_stacks(&stacks);
    }

    for stack in stacks {
        print!("{}", stack.iter().last().unwrap());
    }
    println!();

    Ok(())
}
