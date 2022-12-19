use anyhow::Result;
use util::Input;

fn parse_pair(pair: &str) -> (u32, u32) {
    let (fst, snd) = pair.split_once('-').unwrap();

    (fst.parse().unwrap(), snd.parse().unwrap())
}

fn parse_pairs(line: &str) -> ((u32, u32), (u32, u32)) {
    let (first, second) = line.split_once(',').unwrap();

    (parse_pair(first), parse_pair(second))
}

fn main() -> Result<()> {
    let input = Input::new().into_lines()?;

    let mut count = 0u32;

    for line in input {
        if !line.is_empty() {
            let ((a, b), (x, y)) = parse_pairs(&line);

            if (a >= x && a <= y) || (b >= x && b <= y) || (x >= a && x <= b) || (y >= a && y <= b)
            {
                count += 1;
            }
        }
    }

    println!("Container count: {count}");

    Ok(())
}
