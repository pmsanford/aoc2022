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
            print!("{c}");
        }
        println!();
    }
}

fn main() -> Result<()> {
    let input = Input::new().into_lines()?;
    let mut rows = input
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

    for y in rows.iter_mut() {
        y[0].visible = true;
        let mut peak = y[0].height;
        for tree in y.iter_mut().skip(1) {
            if tree.height > peak {
                tree.visible = true;
                peak = tree.height;
            }
        }
        let last = y.len() - 1;
        y[last].visible = true;
        let mut peak = y[last].height;
        for i in (0..y.len() - 1).rev() {
            if y[i].height > peak {
                y[i].visible = true;
                peak = y[i].height;
            }
        }
    }

    for x in 0..rows[0].len() {
        rows[0][x].visible = true;
        let mut peak = rows[0][x].height;
        for row in rows.iter_mut().skip(1) {
            if row[x].height > peak {
                row[x].visible = true;
                peak = row[x].height;
            }
        }
        let last = rows.len() - 1;
        rows[last][x].visible = true;
        let mut peak = rows[last][x].height;
        for y in (0..rows.len() - 1).rev() {
            if rows[y][x].height > peak {
                rows[y][x].visible = true;
                peak = rows[y][x].height;
            }
        }
    }

    println!("{rows:#?}");

    let visible = rows.iter().flatten().filter(|tree| tree.visible).count();

    print_vismap(&rows);

    println!("Visible trees: {visible}");

    Ok(())
}
