use std::collections::HashSet;

use anyhow::Result;
use util::Input;

fn main() -> Result<()> {
    let input = Input::new()?.into_string()?.chars().collect::<Vec<_>>();

    for idx in 4..input.len() {
        if HashSet::<&char>::from_iter(input[idx - 4..idx].iter()).len() == 4 {
            println!(r"Idx: {idx}");
            break;
        }
    }

    Ok(())
}
