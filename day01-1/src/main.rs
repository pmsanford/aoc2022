use std::{fs::File, io::{BufReader, BufRead}};
use anyhow::Result;

fn main() -> Result<()> {
    let file = File::open("input.txt")?;
    let bufread = BufReader::new(file);

    let mut current = 0u32;
    let mut max = 0u32;

    for line in bufread.lines() {
        let line = line?;

        if line.is_empty() {
            if current > max {
                max = current;
            }
            current = 0;
        } else {
            let cals: u32 = line.parse()?;
            current += cals;
        }
    }

    if current > max {
        max = current;
    }

    println!("Elf with the most calories has {}", max);

    Ok(())
}
