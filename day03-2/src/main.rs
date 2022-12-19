use std::collections::HashSet;

use anyhow::Result;
use util::Input;

fn priority(item: &char) -> u8 {
    let mut priority = *item as u8;

    if item.is_ascii_uppercase() {
        priority -= 38;
    } else {
        priority -= 96;
    }

    priority
}

fn main() -> Result<()> {
    let input = Input::new().into_lines()?;

    let mut total = 0u32;

    for knapsacks in input.chunks(3) {
        let e1: HashSet<char> = HashSet::from_iter(knapsacks[0].chars());
        let e2: HashSet<char> = HashSet::from_iter(knapsacks[1].chars());
        let e3: HashSet<char> = HashSet::from_iter(knapsacks[2].chars());

        let badge: HashSet<char> = e1.intersection(&e2).into_iter().copied().collect();
        let badge = badge.intersection(&e3);

        let priority = priority(badge.into_iter().next().unwrap());

        total += priority as u32;
    }

    println!("Priority total is {total}");

    Ok(())
}
