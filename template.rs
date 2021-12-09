use advent_of_code_2021::solver::{solve_file, Solver};

struct DayXSolver {
}

impl Solver<u64> for DayXSolver {
    fn new(problem: &str) -> Self {
        Self {
        }
    }

    fn solve1(&self) -> Option<u64> {
        None
    }

    fn solve2(&self) -> Option<u64> {
        None
    }
}

#[cfg(test)]
mod dayXtests {
    use super::*;
    static EXAMPLE_DATA: &'static str = "";

    #[test]
    fn test_parse() {
        let solver = DayXSolver::new(EXAMPLE_DATA);
    }

    #[test]
    fn test_example_part1() {
        let solver = DayXSolver {
        };
        assert_eq!(solver.solve1(), None);
    }

    #[test]
    fn test_example_part2() {
        let solver = DayXSolver {
        };
        assert_eq!(solver.solve2(), None);
    }
}

fn main() {
    solve_file::<DayXSolver, u64>("dayX.txt");
}
