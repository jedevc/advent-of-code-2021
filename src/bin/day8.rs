use advent_of_code_2021::solver::{solve_file, Solver};

use itertools::Itertools;

struct Day8Solver {
    puzzles: Vec<Puzzle>,
}

impl Solver<u64> for Day8Solver {
    fn new(problem: &str) -> Self {
        let puzzles = problem.split("\n").map(Puzzle::new).collect();
        Self { puzzles }
    }

    fn solve1(&self) -> Option<u64> {
        let result = self
            .puzzles
            .iter()
            .flat_map(Puzzle::solve)
            .filter(|n| *n == 1 || *n == 4 || *n == 7 || *n == 8)
            .count();
        Some(result as u64)
    }

    fn solve2(&self) -> Option<u64> {
        let result = self.puzzles.iter().map(Puzzle::solve_n).sum();
        Some(result)
    }
}

struct Puzzle {
    samples: Vec<String>,
    target: Vec<String>,
}

impl Puzzle {
    fn new(problem: &str) -> Self {
        let (samples, target) = problem.split_once(" | ").unwrap();
        let samples = samples.split_whitespace().map(str::to_string).collect();
        let target = target.split_whitespace().map(str::to_string).collect();
        Self { samples, target }
    }

    fn solve(&self) -> Vec<u8> {
        let collection = vec![
            "abcefg".to_string(),
            "cf".to_string(),
            "acdeg".to_string(),
            "acdfg".to_string(),
            "bcdf".to_string(),
            "abdfg".to_string(),
            "abdefg".to_string(),
            "acf".to_string(),
            "abcdefg".to_string(),
            "abcdfg".to_string(),
        ];

        let basis: Vec<char> = "abcdefg".chars().collect();
        let result = basis
            .clone()
            .into_iter()
            .permutations(basis.len())
            .find(|reorder| {
                self.samples.iter().all(|sample| {
                    collection
                        .iter()
                        .any(|valid| translate(&basis, reorder, sample) == *valid)
                })
            })
            .unwrap();

        self.target
            .iter()
            .map(|target| {
                collection
                    .iter()
                    .position(|valid| translate(&basis, &result, target) == *valid)
                    .unwrap() as u8
            })
            .collect()
    }

    fn solve_n(&self) -> u64 {
        self.solve()
            .into_iter()
            .fold(0, |acc, n| acc * 10 + n as u64)
    }
}

fn translate(basis: &[char], reorder: &[char], target: &str) -> String {
    target
        .chars()
        .map(|ch| {
            basis
                .iter()
                .nth(reorder.iter().position(|x| *x == ch).unwrap())
                .unwrap()
        })
        .sorted()
        .collect()
}

#[cfg(test)]
mod day8tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE_DATA: &'static str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";
    static EXAMPLE_SAMPLES: [&'static str; 10] = [
        "be", "cfbegad", "cbdgef", "fgaecd", "cgeb", "fdcge", "agebfd", "fecdb", "fabcd", "edb",
    ];
    static EXAMPLE_TARGET: [&'static str; 4] = ["fdgacbe", "cefdb", "cefbgd", "gcbe"];

    static FULL_EXAMPLE: &'static str = indoc!(
        "
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    "
    );

    #[test]
    fn test_puzzle_parse() {
        let puzzle = Puzzle::new(EXAMPLE_DATA);
        assert_eq!(puzzle.samples, EXAMPLE_SAMPLES.to_vec());
        assert_eq!(puzzle.target, EXAMPLE_TARGET.to_vec());
    }

    #[test]
    fn test_puzzle_solve() {
        let puzzle = Puzzle {
            samples: EXAMPLE_SAMPLES.map(str::to_string).to_vec(),
            target: EXAMPLE_TARGET.map(str::to_string).to_vec(),
        };
        assert_eq!(puzzle.solve(), [8, 3, 9, 4]);
    }

    #[test]
    fn test_example_part1() {
        let solver = Day8Solver::new(FULL_EXAMPLE.trim());
        assert_eq!(solver.solve1(), Some(26));
    }

    #[test]
    fn test_example_part2() {
        let solver = Day8Solver::new(FULL_EXAMPLE.trim());
        assert_eq!(solver.solve2(), Some(61229));
    }
}

fn main() {
    solve_file::<Day8Solver, u64>("day8.txt");
}
