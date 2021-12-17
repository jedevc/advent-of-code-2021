use advent_of_code_2021::solver::{solve_file, Solver};

struct Day17Solver {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

impl Solver<i64> for Day17Solver {
    fn new(problem: &str) -> Self {
        let problem = problem.trim_start_matches("target area: ");

        let (xs, ys) = problem.split_once(", ").unwrap();
        let xs = xs.trim_start_matches("x=");
        let ys = ys.trim_start_matches("y=");

        let (min_x, max_x) = xs.split_once("..").unwrap();
        let (min_y, max_y) = ys.split_once("..").unwrap();

        let (min_x, max_x) = (min_x.parse().unwrap(), max_x.parse().unwrap());
        let (min_y, max_y) = (min_y.parse().unwrap(), max_y.parse().unwrap());

        Self {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    fn solve1(&self) -> Option<i64> {
        let paths = self.find_paths();
        let result = paths.iter().map(|solve| solve.highest).max();
        result
    }

    fn solve2(&self) -> Option<i64> {
        let paths = self.find_paths();
        Some(paths.len() as i64)
    }
}

impl Day17Solver {
    fn find_paths(&self) -> Vec<Solve> {
        let min_scan_x = -i64::max(i64::abs(self.min_x), i64::abs(self.max_x));
        let max_scan_x = i64::max(i64::abs(self.min_x), i64::abs(self.max_x));
        let min_scan_y = -i64::max(i64::abs(self.min_y), i64::abs(self.max_y));
        let max_scan_y = i64::max(i64::abs(self.min_y), i64::abs(self.max_y));

        let max_iters = 2 * i64::max(
            i64::max(i64::abs(self.min_x), i64::abs(self.max_x)),
            i64::max(i64::abs(self.min_y), i64::abs(self.max_y)),
        );

        let mut paths = vec![];
        for i in min_scan_x..max_scan_x + 1 {
            for j in min_scan_y..max_scan_y + 1 {
                let diff = if i == 0 { 0 } else { i64::abs(i) / i };

                let (mut x, mut y) = (0, 0);
                let (mut dx, mut dy) = (i, j);
                let mut highest = y;
                for _ in 0..max_iters {
                    x += dx;
                    y += dy;
                    if dx != 0 {
                        dx -= diff;
                    }
                    dy -= 1;
                    highest = i64::max(highest, y);
                    if x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y {
                        if !(x + dx >= self.min_x
                            && x + dx <= self.max_x
                            && y + dy >= self.min_y
                            && y + dy <= self.max_y)
                        {
                            paths.push(Solve {
                                dx: i,
                                dy: j,
                                highest,
                            });
                            break;
                        }
                    }
                }
            }
        }
        paths
    }
}

#[derive(Debug, Clone)]
struct Solve {
    dx: i64,
    dy: i64,
    highest: i64,
}

#[cfg(test)]
mod day17tests {
    use super::*;
    static EXAMPLE_DATA: &'static str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_parse() {
        let solver = Day17Solver::new(EXAMPLE_DATA);
        assert_eq!(solver.min_x, 20);
        assert_eq!(solver.max_x, 30);
        assert_eq!(solver.min_y, -10);
        assert_eq!(solver.max_y, -5);
    }

    #[test]
    fn test_example_part1() {
        let solver = Day17Solver::new(EXAMPLE_DATA);
        assert_eq!(solver.solve1(), Some(45));
    }

    #[test]
    fn test_example_part2() {
        let solver = Day17Solver::new(EXAMPLE_DATA);
        assert_eq!(solver.solve2(), Some(112));
    }
}

fn main() {
    solve_file::<Day17Solver, i64>("day17.txt");
}
