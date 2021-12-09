use itertools::Itertools;
use std::collections::HashSet;

use advent_of_code_2021::solver::{solve_file, Solver};

struct Day9Solver {
    width: usize,
    height: usize,
    grid: Vec<Vec<u8>>,
}

impl Solver<u64> for Day9Solver {
    fn new(problem: &str) -> Self {
        let grid: Vec<Vec<u8>> = problem
            .split("\n")
            .map(|line| {
                line.chars()
                    .map(|ch| ch.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();
        let width = grid.len();
        let height = grid[0].len();
        Self {
            grid,
            width,
            height,
        }
    }

    fn solve1(&self) -> Option<u64> {
        let result = self
            .low_points()
            .into_iter()
            .map(|(x, y)| self.grid[x][y] as u64 + 1)
            .sum();
        Some(result)
    }

    fn solve2(&self) -> Option<u64> {
        let result = self
            .basins()
            .into_iter()
            .map(|basin| basin.len() as u64)
            .sorted()
            .rev()
            .take(3)
            .product();
        Some(result)
    }
}

impl Day9Solver {
    fn low_points(&self) -> Vec<(usize, usize)> {
        let mut low_points: Vec<(usize, usize)> = vec![];
        for x in 0..self.width {
            for y in 0..self.height {
                let value = self.grid[x][y];
                if x > 0 && self.grid[x - 1][y] <= value {
                    continue;
                }
                if x < self.width - 1 && self.grid[x + 1][y] <= value {
                    continue;
                }
                if y > 0 && self.grid[x][y - 1] <= value {
                    continue;
                }
                if y < self.height - 1 && self.grid[x][y + 1] <= value {
                    continue;
                }
                low_points.push((x, y));
            }
        }
        low_points
    }

    fn basins(&self) -> Vec<HashSet<(usize, usize)>> {
        let mut basins: Vec<HashSet<(usize, usize)>> = self
            .low_points()
            .into_iter()
            .map(|p| {
                let mut h = HashSet::new();
                h.insert(p);
                h
            })
            .collect();

        loop {
            let mut added = false;
            for basin in &mut basins {
                let mut nbasin = vec![];
                for (x, y) in basin.iter() {
                    let value = self.grid[*x][*y];

                    if *x > 0 && self.grid[*x - 1][*y] >= value {
                        nbasin.push((*x - 1, *y));
                    }
                    if *x < self.width - 1 && self.grid[*x + 1][*y] >= value {
                        nbasin.push((*x + 1, *y));
                    }
                    if *y > 0 && self.grid[*x][*y - 1] >= value {
                        nbasin.push((*x, *y - 1));
                    }
                    if *y < self.height - 1 && self.grid[*x][*y + 1] >= value {
                        nbasin.push((*x, *y + 1));
                    }
                }

                for (x, y) in nbasin {
                    if self.grid[x][y] == 9 {
                        continue;
                    }
                    added |= basin.insert((x, y));
                }
            }
            if !added {
                break;
            }
        }

        basins
    }
}

#[cfg(test)]
mod day9tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_DATA: &'static str = indoc!(
        "
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
    "
    );

    #[test]
    fn test_example_part1() {
        let solver = Day9Solver::new(EXAMPLE_DATA.trim());
        assert_eq!(solver.solve1(), Some(15));
    }

    #[test]
    fn test_example_part2() {
        let solver = Day9Solver::new(EXAMPLE_DATA.trim());
        assert_eq!(solver.solve2(), Some(1134));
    }
}

fn main() {
    solve_file::<Day9Solver, u64>("day9.txt");
}
