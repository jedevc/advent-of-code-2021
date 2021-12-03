use advent_of_code_2021::solver::{solve_file, Solver};

struct Day3Solver {
    size: u8,
    numbers: Vec<u64>,
}

impl Solver<u64> for Day3Solver {
    fn new(problem: &str) -> Self {
        Day3Solver {
            numbers: problem
                .split("\n")
                .map(|line| u64::from_str_radix(line, 2).unwrap())
                .collect(),
            size: problem.find("\n").unwrap() as u8,
        }
    }

    fn solve1(&self) -> Option<u64> {
        let gamma = (0..self.size).fold(0, |acc, i| {
            if common_numbers(&self.numbers, i, true) {
                acc | (1 << i)
            } else {
                acc
            }
        });
        let epsilon = !gamma & ((1 << self.size) - 1);
        Some(gamma * epsilon)
    }

    fn solve2(&self) -> Option<u64> {
        let o2 = self.filter_bit_criteria(|numbers, i| common_numbers(&numbers, i, true));
        assert_eq!(o2.len(), 1);
        let co2 = self.filter_bit_criteria(|numbers, i| !common_numbers(&numbers, i, true));
        assert_eq!(co2.len(), 1);
        Some(o2[0] * co2[0])
    }
}

impl Day3Solver {
    fn filter_bit_criteria<F>(&self, criteria: F) -> Vec<u64>
    where
        F: Fn(&[u64], u8) -> bool,
    {
        (0..self.size)
            .rev()
            .fold(self.numbers.clone(), |numbers, i| {
                if numbers.len() <= 1 {
                    return numbers;
                }
                let mask = 1 << i;
                if criteria(&numbers, i) {
                    // keep matching 1s
                    numbers.into_iter().filter(|n| *n & mask != 0).collect()
                } else {
                    // keep matching 0s
                    numbers.into_iter().filter(|n| *n | mask != *n).collect()
                }
            })
    }
}

fn common_numbers(numbers: &[u64], idx: u8, default: bool) -> bool {
    let mask = 1 << idx;
    let count = (numbers).into_iter().filter(|n| *n & mask != 0).count();
    if numbers.len() % 2 == 0 && count == numbers.len() / 2 {
        default
    } else if count > numbers.len() / 2 {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: [u64; 12] = [
        0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000, 0b11001,
        0b00010, 0b01010,
    ];

    #[test]
    fn test_example_part1() {
        let solver = Day3Solver {
            size: 5,
            numbers: EXAMPLE.to_vec(),
        };
        assert_eq!(solver.solve1(), Some(198));
    }

    #[test]
    fn test_example_part2() {
        let solver = Day3Solver {
            size: 5,
            numbers: EXAMPLE.to_vec(),
        };
        assert_eq!(solver.solve2(), Some(230));
    }
}

fn main() {
    solve_file::<Day3Solver, u64>("day3.txt");
}
