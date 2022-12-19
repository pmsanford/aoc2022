use std::collections::HashSet;

use anyhow::Result;
use util::Input;

fn main() -> Result<()> {
    let input = Input::new().into_string()?.chars().collect::<Vec<_>>();

    let indicator_len = 14;

    for idx in indicator_len..input.len() {
        if HashSet::<&char>::from_iter(input[idx - indicator_len..idx].iter()).len()
            == indicator_len
        {
            println!(r"Idx: {idx}");
            break;
        }
    }

    Ok(())
}
