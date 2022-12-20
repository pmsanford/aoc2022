use std::collections::HashMap;

pub trait Grid<T>
where
    T: core::fmt::Debug,
{
    fn get(&self, location: (isize, isize)) -> Option<&T>;

    fn set(&mut self, location: (isize, isize), value: T);

    fn draw(&self, window: ((isize, isize), (isize, isize))) -> Vec<String>;
}

pub struct SparseGrid<T>
where
    T: core::fmt::Debug + Default,
{
    bounds: ((isize, isize), (isize, isize)),
    data: HashMap<(isize, isize), T>,
    renderer: Box<dyn Fn(&T) -> char>,
}

impl<T> SparseGrid<T>
where
    T: core::fmt::Debug + Default,
{
    pub fn new(renderer: Box<dyn Fn(&T) -> char>) -> Self {
        Self {
            bounds: ((0, 0), (0, 0)),
            data: HashMap::new(),
            renderer,
        }
    }

    pub fn get_bounds(&self) -> ((isize, isize), (isize, isize)) {
        self.bounds
    }
}

impl<T> Grid<T> for SparseGrid<T>
where
    T: core::fmt::Debug + Default,
{
    fn get(&self, location: (isize, isize)) -> Option<&T> {
        self.data.get(&location)
    }

    fn set(&mut self, location: (isize, isize), value: T) {
        if self.data.is_empty() {
            self.bounds = (location, location);
        } else {
            self.bounds.0 .0 = self.bounds.0 .0.min(location.0);
            self.bounds.0 .1 = self.bounds.0 .1.min(location.1);
            self.bounds.1 .0 = self.bounds.1 .0.max(location.0);
            self.bounds.1 .1 = self.bounds.1 .1.max(location.1);
        }
        self.data.insert(location, value);
    }

    fn draw(&self, window: ((isize, isize), (isize, isize))) -> Vec<String> {
        let lower_bound = window.0;
        let upper_bound = window.1;
        let default = T::default();
        let mut result = vec![];

        let (pad, lower_x, upper_x) = clamp_x_axis(window);
        let x_num_len = lower_x.to_string().len().max(upper_x.to_string().len());
        let y_num_len = lower_bound
            .0
            .to_string()
            .len()
            .max(upper_bound.0.to_string().len());
        let xl = (lower_x..=upper_x)
            .step_by(5)
            .map(|l| format!("{l: >x_num_len$}").chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let pad_str = " ".repeat(pad + y_num_len + 1);
        for i in 0..x_num_len {
            result.push(format!(
                "{}{}",
                pad_str,
                xl.iter()
                    .map(|n| n[i].to_string())
                    .collect::<Vec<_>>()
                    .join("    ")
            ));
        }
        result.push("".to_owned());
        for y in lower_bound.1..=upper_bound.1 {
            let mut line = vec![];
            for x in lower_bound.0..=upper_bound.0 {
                let v = self.data.get(&(x, y)).unwrap_or(&default);
                let c = (self.renderer)(v);
                line.push(c);
            }
            result.push(format!(
                "{: >y_num_len$} {}",
                y,
                line.into_iter().collect::<String>()
            ));
        }

        result
    }
}

fn clamp_x_axis(window: ((isize, isize), (isize, isize))) -> (usize, isize, isize) {
    let mut lb = window.0 .0;
    let mut pad = 0;

    while lb % 5 != 0 {
        lb += 1;
        pad += 1;
    }

    (pad as usize, lb, window.1 .0)
}
