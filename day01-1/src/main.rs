use anyhow::Result;
use util::Input;

fn main() -> Result<()> {
    let mut current = 0u32;
    let mut max = 0u32;

    for line in Input::new().into_lines()? {
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

    println!("Elf with the most calories has {max}");

    Ok(())
}
