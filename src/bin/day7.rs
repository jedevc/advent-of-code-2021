use advent_of_code_2021::solver::{solve_file, Solver};

struct Day7Solver {
    positions: Vec<u64>,
    min_position: u64,
    max_position: u64,
}

impl Solver<u64> for Day7Solver {
    fn new(problem: &str) -> Self {
        let positions: Vec<u64> = problem
            .split(",")
            .map(|position| position.parse().unwrap())
            .collect();
        let min_position: u64 = *positions.iter().min().unwrap();
        let max_position: u64 = *positions.iter().max().unwrap();
        Self {
            positions,
            min_position,
            max_position,
        }
    }

    fn solve1(&self) -> Option<u64> {
        let result = self.solve(|cost| cost);
        Some(result)
    }

    fn solve2(&self) -> Option<u64> {
        let result = self.solve(|cost| cost * (cost + 1) / 2);
        Some(result)
    }
}

impl Day7Solver {
    fn solve<F>(&self, cost: F) -> u64
    where
        F: Fn(u64) -> u64,
    {
        (self.min_position..self.max_position)
            .map(|i| {
                self.positions
                    .iter()
                    .map(|j| cost(i64::abs(i as i64 - *j as i64) as u64))
                    .sum()
            })
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod day7tests {
    use super::*;
    static EXAMPLE_DATA: &'static str = "16,1,2,0,4,2,7,1,2,14";
    static EXAMPLE_POSITIONS: [u64; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn test_parse() {
        let solver = Day7Solver::new(EXAMPLE_DATA);
        assert_eq!(solver.positions, EXAMPLE_POSITIONS.to_vec());
        assert_eq!(solver.max_position, 16);
        assert_eq!(solver.min_position, 0);
    }

    #[test]
    fn test_example_part1() {
        let solver = Day7Solver {
            positions: EXAMPLE_POSITIONS.to_vec(),
            min_position: 0,
            max_position: 16,
        };
        assert_eq!(solver.solve1(), Some(37));
    }

    #[test]
    fn test_example_part2() {
        let solver = Day7Solver {
            positions: EXAMPLE_POSITIONS.to_vec(),
            min_position: 0,
            max_position: 16,
        };
        assert_eq!(solver.solve2(), Some(168));
    }
}

fn main() {
    solve_file::<Day7Solver, u64>("day7.txt");
}
