use std::{fs::File, io::{BufReader, BufRead}};
use anyhow::Result;

fn main() -> Result<()> {
    let file = File::open("input.txt")?;
    let bufread = BufReader::new(file);

    let mut elves = vec![];
    let mut current = 0u32;

    for line in bufread.lines() {
        let line = line?;

        if line.is_empty() {
            if current > 0 {
                elves.push(current);
            }
            current = 0;
        } else {
            let cals: u32 = line.parse()?;
            current += cals;
        }
    }

    elves.sort();
    elves.reverse();

    let cals: u32 = elves.into_iter().take(3).sum();

    println!("Top three elves have {} calories", cals);

    Ok(())
}
