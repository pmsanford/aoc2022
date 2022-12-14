use anyhow::Result;
use regex::Regex;
use util::Input;

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
    let lines = Input::new()?.into_lines()?;

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
        let re = Regex::new("move ([0-9]+) from ([0-9]+) to ([0-9]+)")?;
        let c = re.captures(inst).unwrap();
        let count = c.get(1).unwrap().as_str().parse::<usize>()?;
        let from = c.get(2).unwrap().as_str().parse::<usize>()? - 1;
        let to = c.get(3).unwrap().as_str().parse::<usize>()? - 1;

        let mut in_motion = vec![];

        for _ in 0..count {
            in_motion.push(stacks[from].pop().unwrap());
        }
        in_motion.reverse();
        for popped in in_motion.drain(..) {
            stacks[to].push(popped);
        }
        print_stacks(&stacks);
    }

    for stack in stacks {
        print!("{}", stack.iter().last().unwrap());
    }
    println!();

    Ok(())
}
