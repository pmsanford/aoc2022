use std::collections::HashSet;

use anyhow::Result;
use util::Input;

fn main() -> Result<()> {
    let input = Input::new()?.into_lines()?;

    let mut total = 0u32;

    for knapsack in input {
        let (c1, c2) = knapsack.split_at(knapsack.len() / 2);
        let c1: HashSet<char> = HashSet::from_iter(c1.chars());
        let c2: HashSet<char> = HashSet::from_iter(c2.chars());

        let item = c1.intersection(&c2).into_iter().next().unwrap();
        let mut priority = *item as u8;

        if item.is_ascii_uppercase() {
            priority -= 38;
        } else {
            priority -= 96;
        }

        total += priority as u32;
    }

    println!("Priority total is {total}");

    Ok(())
}
