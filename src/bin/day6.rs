use advent_of_code_2021::solver::{solve_file, Solver};

struct Day6Solver {
    bins: [u64; 9],
}

impl Solver<u64> for Day6Solver {
    fn new(problem: &str) -> Self {
        let counts: Vec<usize> = problem
            .split(",")
            .map(|count| count.parse().unwrap())
            .collect();

        let mut bins = [0; 9];
        for count in counts {
            bins[count] += 1;
        }
        Self { bins }
    }

    fn solve1(&self) -> Option<u64> {
        Some(self.solve(80))
    }

    fn solve2(&self) -> Option<u64> {
        Some(self.solve(256))
    }
}

impl Day6Solver {
    fn solve(&self, generations: u64) -> u64 {
        let mut bins = self.bins;
        for _ in 0..generations {
            let tmp = bins[0];
            for i in 0..8 {
                bins[i] = bins[i + 1];
            }
            bins[6] += tmp;
            bins[8] = tmp;
        }

        bins.iter().sum()
    }
}

#[cfg(test)]
mod day6tests {
    use super::*;
    static EXAMPLE_INPUT: &'static str = "3,4,3,1,2";
    static EXAMPLE_BINS: [u64; 9] = [0, 1, 1, 2, 1, 0, 0, 0, 0];

    #[test]
    fn test_parse() {
        let solver = Day6Solver::new(EXAMPLE_INPUT);
        assert_eq!(solver.bins, EXAMPLE_BINS);
    }

    #[test]
    fn test_example_part1() {
        let solver = Day6Solver { bins: EXAMPLE_BINS };
        assert_eq!(solver.solve1(), Some(5934));
    }

    #[test]
    fn test_example_part2() {
        let solver = Day6Solver { bins: EXAMPLE_BINS };
        assert_eq!(solver.solve2(), Some(26984457539));
    }
}

fn main() {
    solve_file::<Day6Solver, u64>("day6.txt");
}
