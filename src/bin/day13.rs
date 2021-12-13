use itertools::Itertools;

use advent_of_code_2021::solver::{solve_file, Solver};

struct Day13Solver {
    holes: Vec<(isize, isize)>,
    folds: Vec<Fold>,
}

#[derive(Debug)]
enum Fold {
    X(isize),
    Y(isize),
}

impl Fold {
    fn execute(&self, holes: &[(isize, isize)]) -> Vec<(isize, isize)> {
        holes
            .iter()
            .map(|(x, y)| match self {
                Fold::X(n) => (if *x <= *n { *x } else { *x - 2 * (*x - *n) }, *y),
                Fold::Y(n) => (*x, if *y <= *n { *y } else { *y - 2 * (*y - *n) }),
            })
            .sorted()
            .dedup()
            .collect()
    }
}

impl Solver<usize> for Day13Solver {
    fn new(problem: &str) -> Self {
        let (holes, folds) = problem.split_once("\n\n").unwrap();
        let holes = holes
            .split("\n")
            .map(|line| {
                let (x, y) = line.split_once(",").unwrap();
                let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
                (x, y)
            })
            .collect();
        let folds = folds
            .split("\n")
            .map(|line| {
                let (axis, n) = line
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .split_once("=")
                    .unwrap();
                let n = n.parse().unwrap();
                match axis {
                    "x" => Fold::X(n),
                    "y" => Fold::Y(n),
                    _ => panic!("bad axis"),
                }
            })
            .collect();
        Self { holes, folds }
    }

    fn solve1(&self) -> Option<usize> {
        let holes = self.folds[0].execute(&self.holes);
        Some(holes.len())
    }

    fn solve2(&self) -> Option<usize> {
        let result = self
            .folds
            .iter()
            .fold(self.holes.clone(), |acc, fold| fold.execute(&acc));
        let width = result.iter().map(|(x, _)| x).max().unwrap() + 1;
        let height = result.iter().map(|(_, y)| y).max().unwrap() + 1;
        for y in 0..height {
            for x in 0..width {
                if result.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        None
    }
}

#[cfg(test)]
mod day13tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_DATA: &'static str = indoc!(
        "
        6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0
        
        fold along y=7
        fold along x=5
    "
    );

    #[test]
    fn test_example_part1() {
        let solver = Day13Solver::new(EXAMPLE_DATA.trim());
        assert_eq!(solver.solve1(), Some(17));
    }
}

fn main() {
    solve_file::<Day13Solver, usize>("day13.txt");
}
