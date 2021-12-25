use advent_of_code_2021::solver::{solve_file, Solver};

#[derive(PartialEq, Eq, Debug)]
struct Day25Solver {
    grid: Vec<Vec<Square>>,
    width: usize,
    height: usize,
}

impl Solver<u64> for Day25Solver {
    fn new(problem: &str) -> Self {
        let grid: Vec<Vec<_>> = problem
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| match ch {
                        '>' => Square::East,
                        'v' => Square::South,
                        '.' => Square::Empty,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();
        let height = grid.len();
        let width = grid[0].len();
        Self {
            grid,
            width,
            height,
        }
    }

    fn solve1(&self) -> Option<u64> {
        let mut grid = self.grid.clone();
        let mut changed = true;
        let mut count = 0;

        while changed {
            let (new_grid, new_changed) = step(grid, self.width, self.height);
            grid = new_grid;
            changed = new_changed;

            count += 1;

            if count != 1 {
                continue;
            }
        }

        Some(count)
    }

    fn solve2(&self) -> Option<u64> {
        None
    }
}

fn step(mut grid: Vec<Vec<Square>>, width: usize, height: usize) -> (Vec<Vec<Square>>, bool) {
    let mut changed = false;

    let old = grid.clone();
    for row in 0..height {
        let mut skip = false;
        for col in 0..width {
            if skip {
                skip = false;
                continue;
            }

            if grid[row][col] != Square::East {
                continue;
            }

            if old[row][(col + 1) % width] == Square::Empty {
                grid[row][col] = Square::Empty;
                grid[row][(col + 1) % width] = Square::East;
                changed = true;
                skip = true;
            }
        }
    }

    let old = grid.clone();
    for col in 0..width {
        let mut skip = false;
        for row in 0..height {
            if skip {
                skip = false;
                continue;
            }

            if grid[row][col] != Square::South {
                continue;
            }

            if old[(row + 1) % height][col] == Square::Empty {
                grid[row][col] = Square::Empty;
                grid[(row + 1) % height][col] = Square::South;
                changed = true;
                skip = true;
            }
        }
    }

    (grid, changed)
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum Square {
    Empty,
    East,
    South,
}

#[cfg(test)]
mod day25tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_DATA: &'static str = indoc!(
        "
        v...>>.vv>
        .vv>>.vv..
        >>.>v>...v
        >>v>>.>.v.
        v>v.vv.v..
        >.>>..v...
        .vv..>.>v.
        v.v..>>v.v
        ....v..v.>
    "
    );

    #[test]
    fn test_steps() {
        let mut solver = Day25Solver::new(EXAMPLE_DATA.trim());

        let (grid, changed) = step(solver.grid, solver.width, solver.height);
        solver.grid = grid;
        assert!(changed);
        assert_eq!(
            solver,
            Day25Solver::new(
                indoc!(
                    "
                    ....>.>v.>
                    v.v>.>v.v.
                    >v>>..>v..
                    >>v>v>.>.v
                    .>v.v...v.
                    v>>.>vvv..
                    ..v...>>..
                    vv...>>vv.
                    >.v.v..v.v
                    "
                )
                .trim()
            )
        );

        let (grid, changed) = step(solver.grid, solver.width, solver.height);
        solver.grid = grid;
        assert!(changed);
        assert_eq!(
            solver,
            Day25Solver::new(
                indoc!(
                    "
                    >.v.v>>..v
                    v.v.>>vv..
                    >v>.>.>.v.
                    >>v>v.>v>.
                    .>..v....v
                    .>v>>.v.v.
                    v....v>v>.
                    .vv..>>v..
                    v>.....vv.
                    "
                )
                .trim()
            )
        );
    }

    #[test]
    fn test_example_part1() {
        let solver = Day25Solver::new(EXAMPLE_DATA.trim());
        assert_eq!(solver.solve1(), Some(58));
    }

    #[test]
    fn test_example_part2() {
        let solver = Day25Solver::new(EXAMPLE_DATA.trim());
        assert_eq!(solver.solve2(), None);
    }
}

fn main() {
    solve_file::<Day25Solver, u64>("day25.txt");
}
