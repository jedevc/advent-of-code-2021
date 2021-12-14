use advent_of_code_2021::solver::{solve_file, Solver};
use std::collections::HashMap;

struct Day14Solver {
    template: String,
    table: HashMap<(char, char), char>,
}

impl Solver<u64> for Day14Solver {
    fn new(problem: &str) -> Self {
        let (template, table) = problem.split_once("\n\n").unwrap();
        let template = template.to_string();
        let table = table
            .split("\n")
            .map(|line| {
                let (lhs, rhs) = line.split_once(" -> ").unwrap();
                let lhs: Vec<char> = lhs.chars().collect();
                let rhs: Vec<char> = rhs.chars().collect();
                assert_eq!(lhs.len(), 2);
                assert_eq!(rhs.len(), 1);
                ((lhs[0], lhs[1]), rhs[0])
            })
            .collect();
        Self { template, table }
    }

    fn solve1(&self) -> Option<u64> {
        Some(self.solve(10))
    }

    fn solve2(&self) -> Option<u64> {
        Some(self.solve(40))
    }
}

impl Day14Solver {
    fn solve(&self, steps: usize) -> u64 {
        let pairs = self.template.chars().zip(self.template.chars().skip(1));
        let pairs = pairs.fold(HashMap::new(), |mut acc, k| {
            let counter = acc.entry(k).or_insert(0);
            *counter += 1;
            acc
        });
        let first = self.template.chars().next().unwrap();
        let last = self.template.chars().last().unwrap();

        let result = (0..steps).fold(pairs, |acc, _| self.step(&acc));

        let histogram = result
            .iter()
            .fold(HashMap::new(), |mut acc, ((a, b), count)| {
                let counter = acc.entry(a).or_insert(0);
                *counter += count;
                let counter = acc.entry(b).or_insert(0);
                *counter += count;
                acc
            });

        let (min_ch, min_count) = histogram
            .iter()
            .min_by(|(_, v1), (_, v2)| v1.cmp(v2))
            .unwrap();
        let (max_ch, max_count) = histogram
            .iter()
            .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
            .unwrap();
        let (mut min_count, mut max_count) = (min_count / 2, max_count / 2);
        if **min_ch == first || **min_ch == last {
            min_count += 1;
        }
        if **max_ch == first || **max_ch == last {
            max_count += 1;
        }

        max_count - min_count
    }

    fn step(&self, pairs: &HashMap<(char, char), u64>) -> HashMap<(char, char), u64> {
        let mut result = HashMap::new();
        for ((a, b), count) in pairs {
            if let Some(c) = self.table.get(&(*a, *b)) {
                let counter = result.entry((*a, *c)).or_insert(0);
                *counter += count;
                let counter = result.entry((*c, *b)).or_insert(0);
                *counter += count;
            }
        }
        result
    }
}

#[cfg(test)]
mod day14tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_DATA: &'static str = indoc!(
        "
        NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C
    "
    );

    #[test]
    fn test_example_part1() {
        let solver = Day14Solver::new(EXAMPLE_DATA.trim());
        assert_eq!(solver.solve1(), Some(1588));
    }

    #[test]
    fn test_example_part2() {
        let solver = Day14Solver::new(EXAMPLE_DATA.trim());
        assert_eq!(solver.solve2(), Some(2188189693529));
    }
}

fn main() {
    solve_file::<Day14Solver, u64>("day14.txt");
}
