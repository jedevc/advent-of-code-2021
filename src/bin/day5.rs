use advent_of_code_2021::solver::{solve_file, Solver};

struct Day5Solver {
    vents: Vec<Line>,
    max_size: usize,
}

#[derive(Clone, Debug, PartialEq)]
struct Line {
    source: (usize, usize),
    dest: (usize, usize),
}

impl Solver<u64> for Day5Solver {
    fn new(problem: &str) -> Self {
        let vents: Vec<Line> = problem
            .split("\n")
            .map(|line| {
                let (source, dest) = line.split_once(" -> ").unwrap();
                let (sx, sy) = source.split_once(",").unwrap();
                let (sx, sy) = (sx.parse().unwrap(), sy.parse().unwrap());
                let (dx, dy) = dest.split_once(",").unwrap();
                let (dx, dy) = (dx.parse().unwrap(), dy.parse().unwrap());

                Line {
                    source: (sx, sy),
                    dest: (dx, dy),
                }
            })
            .collect();
        let max_size: usize = 1 + vents.iter().fold(0, |acc, vent| {
            let idxs = [acc, vent.source.0, vent.source.1, vent.dest.0, vent.dest.1];
            idxs.into_iter().reduce(usize::max).unwrap()
        });
        Self { vents, max_size }
    }

    fn solve1(&self) -> Option<u64> {
        let mut grid = Grid::new(self.max_size);
        for vent in &self.vents {
            grid.fill(vent.source, vent.dest, false);
        }

        Some(grid.count() as u64)
    }

    fn solve2(&self) -> Option<u64> {
        let mut grid = Grid::new(self.max_size);
        for vent in &self.vents {
            grid.fill(vent.source, vent.dest, true);
        }
        Some(grid.count() as u64)
    }
}

struct Grid {
    data: Vec<Vec<u64>>,
}

impl Grid {
    fn new(size: usize) -> Grid {
        Grid {
            data: vec![vec![0; size]; size],
        }
    }

    fn fill(&mut self, (sx, sy): (usize, usize), (dx, dy): (usize, usize), allow_diagonal: bool) {
        let xdiff = usize::max(sx, dx) - usize::min(sx, dx);
        let ydiff = usize::max(sy, dy) - usize::min(sy, dy);
        if xdiff == 0 {
            // vertical
            for y in usize::min(sy, dy)..usize::max(sy, dy) + 1 {
                self.data[sx][y] += 1;
            }
        } else if ydiff == 0 {
            // horizontal
            for x in usize::min(sx, dx)..usize::max(sx, dx) + 1 {
                self.data[x][sy] += 1;
            }
        } else if xdiff == ydiff {
            // diagonal
            if allow_diagonal {
                let (offset_x, offset_y): (isize, isize) = if sx < dx {
                    if sy < dy {
                        (1, 1)
                    } else {
                        (1, -1)
                    }
                } else {
                    if sy < dy {
                        (-1, 1)
                    } else {
                        (-1, -1)
                    }
                };

                for i in 0..xdiff + 1 {
                    let (offset_x, offset_y) = (offset_x * i as isize, offset_y * i as isize);
                    let x = sx as isize + offset_x;
                    let y = sy as isize + offset_y;
                    self.data[x as usize][y as usize] += 1;
                }
            }
        } else {
            panic!("cannot connect points!");
        }
    }

    fn count(&self) -> usize {
        self.data
            .iter()
            .map(|line| line.iter().filter(|overlap| **overlap >= 2).count())
            .sum()
    }
}

#[cfg(test)]
mod day5tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_PARSE: &'static str = indoc! {"
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
    "};
    static EXAMPLE_VENTS: [Line; 10] = [
        Line {
            source: (0, 9),
            dest: (5, 9),
        },
        Line {
            source: (8, 0),
            dest: (0, 8),
        },
        Line {
            source: (9, 4),
            dest: (3, 4),
        },
        Line {
            source: (2, 2),
            dest: (2, 1),
        },
        Line {
            source: (7, 0),
            dest: (7, 4),
        },
        Line {
            source: (6, 4),
            dest: (2, 0),
        },
        Line {
            source: (0, 9),
            dest: (2, 9),
        },
        Line {
            source: (3, 4),
            dest: (1, 4),
        },
        Line {
            source: (0, 0),
            dest: (8, 8),
        },
        Line {
            source: (5, 5),
            dest: (8, 2),
        },
    ];

    #[test]
    fn test_parse() {
        let solver = Day5Solver::new(EXAMPLE_PARSE.trim());
        assert_eq!(solver.vents, EXAMPLE_VENTS.to_vec());
        assert_eq!(solver.max_size, 10);
    }

    #[test]
    fn test_example_part1() {
        let solver = Day5Solver {
            vents: EXAMPLE_VENTS.to_vec(),
            max_size: 10,
        };
        assert_eq!(solver.solve1(), Some(5));
    }

    #[test]
    fn test_example_part2() {
        let solver = Day5Solver {
            vents: EXAMPLE_VENTS.to_vec(),
            max_size: 10,
        };
        assert_eq!(solver.solve2(), Some(12));
    }
}

fn main() {
    solve_file::<Day5Solver, u64>("day5.txt");
}
