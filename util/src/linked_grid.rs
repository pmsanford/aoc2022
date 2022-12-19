use std::ops::Index;

use anyhow::{bail, Result};
use petgraph::{stable_graph::NodeIndex, Graph};

#[derive(Debug, Clone)]
pub struct Point<T: core::fmt::Debug + Clone> {
    pub x: usize,
    pub y: usize,
    pub data: T,
}

#[derive(Debug, Clone)]
pub struct LinkedGrid<T: core::fmt::Debug + Clone> {
    graph: Graph<Point<T>, ()>,
    indicies: Vec<Vec<NodeIndex>>,
}

impl<T: core::fmt::Debug + Clone> LinkedGrid<T> {
    pub fn new(width: usize, height: usize, fdata: impl Fn(usize, usize) -> T) -> Self {
        let mut graph = Graph::<Point<T>, ()>::new();
        let mut indicies = vec![vec![NodeIndex::default(); width]; height];
        for x in 0..width {
            #[allow(clippy::needless_range_loop)]
            for y in 0..height {
                indicies[y][x] = graph.add_node(Point {
                    x,
                    y,
                    data: fdata(x, y),
                });
            }
        }

        Self { graph, indicies }
    }

    pub fn try_link(&mut self, from: (isize, isize), to: (isize, isize)) -> Result<()> {
        if !self.in_bounds(from) || !self.in_bounds(to) {
            bail!("Point out of bounds");
        }

        self.graph.add_edge(
            self.indicies[from.1 as usize][from.0 as usize],
            self.indicies[to.1 as usize][to.0 as usize],
            (),
        );

        Ok(())
    }

    pub fn visit_grid(&self, flink: impl Fn(usize, usize)) {
        for y in 0..self.indicies.len() {
            for x in 0..self.indicies[0].len() {
                flink(x, y);
            }
        }
    }

    fn in_bounds(&self, location: (isize, isize)) -> bool {
        location.0 >= 0
            && location.1 >= 0
            && (location.0 as usize) < self.indicies[0].len()
            && (location.1 as usize) < self.indicies.len()
    }

    pub fn neighbors(&self, location: (usize, usize)) -> Vec<&Point<T>> {
        self.graph
            .neighbors(self.indicies[location.1][location.0])
            .map(|n| &self.graph[n])
            .collect()
    }

    pub fn draw_range(
        &self,
        corner: (usize, usize),
        width: usize,
        height: usize,
        render: impl Fn(&T) -> char,
    ) {
        for y in corner.1..(corner.1 + height) {
            for x in corner.0..(corner.0 + width) {
                print!("{}", render(&self.graph[self.indicies[y][x]].data));
            }
            println!();
        }
    }

    pub fn get_data(&self, x: usize, y: usize) -> Result<&T> {
        Ok(&self.graph[self.indicies[y][x]].data)
    }

    pub fn set_data(&mut self, x: usize, y: usize, new_data: T) -> Result<()> {
        if !self.in_bounds((x as isize, y as isize)) {
            bail!("Out of bounds");
        }

        self.graph[self.indicies[y][x]].data = new_data;

        Ok(())
    }
}

impl<T: core::fmt::Debug + Clone> Index<(usize, usize)> for LinkedGrid<T> {
    type Output = Point<T>;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.graph[self.indicies[index.1][index.0]]
    }
}
