use anyhow::Result;
use util::Input;

fn main() -> Result<()> {
    let mut elves = vec![];
    let mut current = 0u32;

    for line in Input::new().into_lines()? {
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

    println!("Top three elves have {cals} calories");

    Ok(())
}
