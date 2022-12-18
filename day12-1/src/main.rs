use anyhow::{bail, Result};
use petgraph::{algo::dijkstra, stable_graph::NodeIndex, Graph};
use util::Input;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn left(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn right(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }
    fn up(&self) -> Point {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn down(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }
}

fn cell_height(grid: &[Vec<char>], point: &Point) -> Result<u8> {
    Ok(match grid[point.y as usize][point.x as usize] {
        'S' => 0,
        'E' => 25,
        c if c as u8 >= 97 && c as u8 <= 122 => c as u8 - 97,
        _ => bail!("Not a height"),
    })
}

fn in_bounds(grid: &Vec<Vec<char>>, point: &Point) -> bool {
    !(point.x as usize >= grid[0].len() || point.y as usize >= grid.len())
}

fn get_index(indicies: &[Vec<NodeIndex>], point: &Point) -> NodeIndex {
    indicies[point.y as usize][point.x as usize]
}

fn try_link(
    graph: &mut Graph<Point, ()>,
    grid: &Vec<Vec<char>>,
    indicies: &[Vec<NodeIndex>],
    from_point: &Point,
    to_point: &Point,
) -> Result<()> {
    let height = cell_height(grid, from_point)?;

    if in_bounds(grid, to_point) && cell_height(grid, to_point)? <= height + 1 {
        let from_index = get_index(indicies, from_point);
        let to_index = get_index(indicies, to_point);
        graph.add_edge(from_index, to_index, ());
    }

    Ok(())
}

fn main() -> Result<()> {
    let input = Input::new()?.into_lines()?;
    let grid = input
        .into_iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut g = Graph::<Point, ()>::new();

    let mut indicies = vec![vec![]; grid.len()];

    let mut start = None;
    let mut end = None;

    for x in 0..grid[0].len() {
        for y in 0..grid.len() {
            let pt = Point {
                x: x as isize,
                y: y as isize,
            };
            if grid[y][x] == 'S' {
                start = Some(pt);
            }
            if grid[y][x] == 'E' {
                end = Some(pt);
            }
            let idx = g.add_node(pt);
            indicies[y].push(idx);
        }
    }

    for x in 0..indicies[0].len() {
        for y in 0..indicies.len() {
            let point = g[indicies[y][x]];
            try_link(&mut g, &grid, &indicies, &point, &point.left())?;
            try_link(&mut g, &grid, &indicies, &point, &point.right())?;
            try_link(&mut g, &grid, &indicies, &point, &point.up())?;
            try_link(&mut g, &grid, &indicies, &point, &point.down())?;
        }
    }

    let start = start.unwrap();
    let end = end.unwrap();

    let path = dijkstra(
        &g,
        get_index(&indicies, &start),
        Some(get_index(&indicies, &end)),
        |_| 1,
    );

    println!("Path: {}", path[&get_index(&indicies, &end)]);

    Ok(())
}
