use anyhow::Result;
use util::Input;

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Tree {
    x: usize,
    y: usize,
    height: usize,
    visible: bool,
}

fn print_vismap(rows: &Vec<Vec<Tree>>) {
    for row in rows {
        for tree in row {
            let c = if tree.visible { "V" } else { "H" };
            print!("{}", c);
        }
        println!();
    }
}

fn in_bounds(rows: &Vec<Vec<Tree>>, cur: (isize, isize)) -> bool {
    cur.0 >= 0 && cur.1 >= 0 && cur.1 < rows[0].len() as isize && cur.0 < rows.len() as isize
}

fn count_visible(rows: &Vec<Vec<Tree>>, start: (usize, usize)) -> [usize; 4] {
    [
        count_visible_to(rows, start, (1, 0)),
        count_visible_to(rows, start, (0, 1)),
        count_visible_to(rows, start, (-1, 0)),
        count_visible_to(rows, start, (0, -1)),
    ]
}

fn count_visible_to(rows: &Vec<Vec<Tree>>, start: (usize, usize), step: (isize, isize)) -> usize {
    let mut count = 0;
    let start_height = rows[start.1][start.0].height;

    let mut cur = (start.0 as isize + step.0, start.1 as isize + step.1);
    while in_bounds(rows, cur) && rows[cur.1 as usize][cur.0 as usize].height < start_height {
        count += 1;
        cur = (cur.0 + step.0, cur.1 + step.1);
    }

    if in_bounds(rows, cur) {
        count += 1;
    }

    count
}

fn main() -> Result<()> {
    let input = Input::new()?.into_lines()?;
    let rows = input
        .into_iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| Tree {
                    x,
                    y,
                    height: c as usize - 48,
                    visible: false,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut max_score = 0;

    for x in 0..rows[0].len() {
        for y in 0..rows.len() {
            max_score = count_visible(&rows, (x, y))
                .into_iter()
                .fold(1, |acc, x| acc * x)
                .max(max_score);
        }
    }

    println!("Max visibility score: {}", max_score);

    Ok(())
}
