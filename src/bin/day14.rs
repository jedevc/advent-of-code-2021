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
        let result: Vec<char> = self.template.chars().collect();
        let result: Vec<char> = (0..steps).fold(result, |acc, i| { println!("{}", i); self.step(&acc) });
        let histogram = result.iter().fold(HashMap::new(), |mut acc, ch| {
            let counter = acc.entry(ch).or_insert(0);
            *counter += 1;
            acc
        });

        let (_, min_count) = histogram
            .iter()
            .min_by(|(_, v1), (_, v2)| v1.cmp(v2))
            .unwrap();
        let (_, max_count) = histogram
            .iter()
            .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
            .unwrap();
        max_count - min_count
    }

    fn step(&self, template: &[char]) -> Vec<char> {
        let mut new = vec![];
        for (a, b) in template.iter().zip(template.iter().skip(1)) {
            new.push(*a);
            if let Some(c) = self.table.get(&(*a, *b)) {
                new.push(*c);
            }
        }
        new.push(*template.last().unwrap());
        new
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
