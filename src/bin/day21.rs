use advent_of_code_2021::solver::{solve_file, Solver};
use std::collections::HashMap;

struct Day21Solver {
    start_positions: (u64, u64),
}

impl Solver<u64> for Day21Solver {
    fn new(problem: &str) -> Self {
        let (p1, p2) = problem.split_once("\n").unwrap();
        let p1 = p1.split_whitespace().last().unwrap();
        let p2 = p2.split_whitespace().last().unwrap();

        let p1 = p1.parse().unwrap();
        let p2 = p2.parse().unwrap();

        Self {
            start_positions: (p1, p2),
        }
    }

    fn solve1(&self) -> Option<u64> {
        let (mut p1, mut p2) = self.start_positions;
        p1 -= 1;
        p2 -= 1;

        let (mut s1, mut s2) = (0, 0);

        let mut dice = 0;
        let mut turn = true;

        while s1 < 1000 && s2 < 1000 {
            let mut total = 0;
            total += (dice) % 100 + 1;
            total += (dice + 1) % 100 + 1;
            total += (dice + 2) % 100 + 1;
            dice += 3;

            if turn {
                p1 += total;
                p1 %= 10;
                s1 += p1 + 1;
            } else {
                p2 += total;
                p2 %= 10;
                s2 += p2 + 1;
            }

            turn = !turn;
        }

        Some(dice * u64::min(s1, s2))
    }

    fn solve2(&self) -> Option<u64> {
        #[derive(PartialEq, Eq, Hash, Debug)]
        struct TableEntry {
            p1: u64,
            p2: u64,
            s1: u64,
            s2: u64,
            turn: bool,
        }

        let mut table = HashMap::new();
        table.insert(
            TableEntry {
                p1: self.start_positions.0,
                p2: self.start_positions.1,
                s1: 0,
                s2: 0,
                turn: false,
            },
            1,
        );

        for s1 in 0..=30 {
            for s2 in 0..=30 {
                for p1 in 1..=10 {
                    for p2 in 1..=10 {
                        for turn in &[true, false] {
                            if (*turn && s1 < p1) || (!turn && s2 < p2) {
                                // impossible!
                                continue;
                            }
                            if (*turn && s1 - p1 >= 21) || (!turn && s2 - p2 >= 21) {
                                // game stops after hitting 21!
                                continue;
                            }

                            let mut total: u64 = 0;
                            for i in 1..=3 {
                                for j in 1..=3 {
                                    for k in 1..=3 {
                                        let sum = i + j + k;
                                        total += if *turn {
                                            let old_p1 =
                                                if p1 == sum { 10 } else { (p1 + 10 - sum) % 10 };
                                            table
                                                .get(&TableEntry {
                                                    p1: old_p1,
                                                    p2,
                                                    s1: s1 - p1,
                                                    s2,
                                                    turn: !turn,
                                                })
                                                .unwrap_or(&0)
                                        } else {
                                            let old_p2 =
                                                if p2 == sum { 10 } else { (p2 + 10 - sum) % 10 };
                                            table
                                                .get(&TableEntry {
                                                    p1,
                                                    p2: old_p2,
                                                    s1,
                                                    s2: s2 - p2,
                                                    turn: !turn,
                                                })
                                                .unwrap_or(&0)
                                        };
                                    }
                                }
                            }
                            if total != 0 {
                                table.insert(
                                    TableEntry {
                                        p1,
                                        p2,
                                        s1,
                                        s2,
                                        turn: *turn,
                                    },
                                    total,
                                );
                            }
                        }
                    }
                }
            }
        }

        let mut total1 = 0;
        let mut total2 = 0;
        for s1 in 21..=30 {
            for s2 in 0..=20 {
                for p1 in 1..=10 {
                    for p2 in 1..=10 {
                        total1 += table
                            .get(&TableEntry {
                                s1,
                                s2,
                                p1,
                                p2,
                                turn: true,
                            })
                            .unwrap_or(&0);

                        total2 += table
                            .get(&TableEntry {
                                s1: s2,
                                s2: s1,
                                p1,
                                p2,
                                turn: false,
                            })
                            .unwrap_or(&0);
                    }
                }
            }
        }

        Some(u64::max(total1, total2))
    }
}

#[cfg(test)]
mod day21tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_DATA: &'static str = indoc!(
        "
        Player 1 starting position: 4
        Player 2 starting position: 8
    "
    );

    #[test]
    fn test_parse() {
        let solver = Day21Solver::new(EXAMPLE_DATA.trim());
        assert_eq!(solver.start_positions, (4, 8));
    }

    #[test]
    fn test_example_part1() {
        let solver = Day21Solver {
            start_positions: (4, 8),
        };
        assert_eq!(solver.solve1(), Some(739785));
    }

    #[test]
    fn test_example_part2() {
        let solver = Day21Solver {
            start_positions: (4, 8),
        };
        assert_eq!(solver.solve2(), Some(444356092776315));
    }
}

fn main() {
    solve_file::<Day21Solver, u64>("day21.txt");
}
