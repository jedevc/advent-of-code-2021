#![feature(test)]

use advent_of_code_2021::solver::{solve_file, Solver};

struct Day4Solver {
    draws: Vec<u64>,
    boards: Vec<Board>,
}

#[derive(Clone, Debug)]
struct Board {
    data: [[Option<u64>; 5]; 5],
}

impl Board {
    fn new(contents: [[u64; 5]; 5]) -> Self {
        let mut data = [[None; 5]; 5];
        for i in 0..5 {
            for j in 0..5 {
                data[i][j] = Some(contents[i][j]);
            }
        }
        Self { data }
    }

    fn new_vec(contents: Vec<Vec<u64>>) -> Self {
        let mut data = [[None; 5]; 5];
        for i in 0..5 {
            for j in 0..5 {
                data[i][j] = Some(contents[i][j]);
            }
        }
        Self { data }
    }

    fn unmarked(&self) -> u64 {
        // calculate sum of unmarked numbers in board, used as part of scoring
        let mut sum = 0;
        for i in 0..5 {
            for j in 0..5 {
                if let Some(value) = self.data[i][j] {
                    sum += value;
                }
            }
        }
        sum
    }

    fn mark(&mut self, value: u64) -> bool {
        for i in 0..5 {
            for j in 0..5 {
                if self.data[i][j] == Some(value) {
                    // mark item as used
                    self.data[i][j] = None;

                    // scan horizontally to see if row is complete
                    let mut won = true;
                    for di in 0..5 {
                        if self.data[di][j] != None {
                            won = false;
                            break;
                        }
                    }
                    if won {
                        return true;
                    }

                    // scan vertically to see if column is complete
                    let mut won = true;
                    for dj in 0..5 {
                        if self.data[i][dj] != None {
                            won = false;
                            break;
                        }
                    }
                    return won;
                }
            }
        }
        false
    }
}

impl Solver<u64> for Day4Solver {
    fn new(problem: &str) -> Self {
        let mut lines = problem.split("\n").filter(|line| line.len() > 0);

        let draws: Vec<u64> = lines
            .next()
            .unwrap()
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect();
        let board_lines: Vec<Vec<u64>> = lines
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect()
            })
            .collect();

        let boards: Vec<Board> = (0..board_lines.len())
            .step_by(5)
            .map(|i| Board::new_vec(board_lines[i..i + 5].to_vec()))
            .collect();
        Self { draws, boards }
    }

    fn solve1(&self) -> Option<u64> {
        let mut boards = self.boards.clone();
        for x in &self.draws {
            for board in &mut boards {
                if board.mark(*x) {
                    // first board with a line wins
                    return Some(x * board.unmarked());
                }
            }
        }

        None
    }

    fn solve2(&self) -> Option<u64> {
        let mut boards: Vec<Option<Board>> =
            self.boards.clone().into_iter().map(Option::Some).collect();
        let mut in_play = boards.len();

        for x in &self.draws {
            for i in 0..boards.len() {
                if let Some(board) = &mut boards[i] {
                    if board.mark(*x) {
                        if in_play == 1 {
                            // last board with a line wins
                            return Some(x * board.unmarked());
                        } else {
                            boards[i] = None;
                            in_play -= 1;
                        }
                    }
                }
            }
        }

        None
    }
}

impl Day4Solver {}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use advent_of_code_2021::solver::{load_file, solve};
    use test::Bencher;

    static EXAMPLE_DRAWS: [u64; 27] = [
        7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3,
        26, 1,
    ];
    static EXAMPLE_BOARDS: [[[u64; 5]; 5]; 3] = [
        [
            [22, 13, 17, 11, 0],
            [8, 2, 23, 4, 24],
            [21, 9, 14, 16, 7],
            [6, 10, 3, 18, 5],
            [1, 12, 20, 15, 19],
        ],
        [
            [3, 15, 0, 2, 22],
            [9, 18, 13, 17, 5],
            [19, 8, 7, 25, 23],
            [20, 11, 10, 24, 4],
            [14, 21, 16, 12, 6],
        ],
        [
            [14, 21, 17, 24, 4],
            [10, 16, 15, 9, 19],
            [18, 8, 23, 26, 20],
            [22, 11, 13, 6, 5],
            [2, 0, 12, 3, 7],
        ],
    ];

    #[test]
    fn test_example_part1() {
        let solver = Day4Solver {
            draws: EXAMPLE_DRAWS.to_vec(),
            boards: EXAMPLE_BOARDS.map(Board::new).to_vec(),
        };
        assert_eq!(solver.solve1(), Some(4512));
    }

    #[test]
    fn test_example_part2() {
        let solver = Day4Solver {
            draws: EXAMPLE_DRAWS.to_vec(),
            boards: EXAMPLE_BOARDS.map(Board::new).to_vec(),
        };
        assert_eq!(solver.solve2(), Some(1924));
    }

    #[bench]
    fn bench_solve_file(b: &mut Bencher) {
        let problem = load_file("day4.txt");
        b.iter(|| solve::<Day4Solver, u64>(&problem))
    }
}

fn main() {
    solve_file::<Day4Solver, u64>("day4.txt");
}
