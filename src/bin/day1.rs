use advent_of_code_2021::solver::{solve_file, Solver};

struct Day1Solver {
    scan: Vec<u64>,
}

impl Solver<u64> for Day1Solver {
    fn new(problem: &str) -> Self {
        Day1Solver {
            scan: problem
                .split('\n')
                .map(|line| line.parse().unwrap())
                .collect(),
        }
    }

    fn solve1(&self) -> Option<u64> {
        let result = Day1Solver::count_increasing(&self.scan);
        Some(result as u64)
    }

    fn solve2(&self) -> Option<u64> {
        let count = 3;
        let windows = (0..self.scan.len() - count + 1).map(|i| &self.scan[i..i + count]);
        let windows_sum: Vec<u64> = windows
            .map(|window| window.iter().copied().reduce(|a, b| a + b).unwrap())
            .collect();
        let result = Day1Solver::count_increasing(&windows_sum);
        Some(result as u64)
    }
}

impl Day1Solver {
    fn count_increasing(items: &[u64]) -> usize {
        items
            .iter()
            .zip(items[1..].iter())
            .filter(|(first, second)| second > first)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: [u64; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn test_example_part1() {
        let solver = Day1Solver {
            scan: EXAMPLE.to_vec(),
        };
        assert_eq!(solver.solve1(), Some(7));
    }

    #[test]
    fn test_example_part2() {
        let solver = Day1Solver {
            scan: EXAMPLE.to_vec(),
        };
        assert_eq!(solver.solve2(), Some(5));
    }
}

fn main() {
    solve_file::<Day1Solver, u64>("day1.txt");
}
