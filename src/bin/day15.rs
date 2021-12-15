use advent_of_code_2021::solver::{solve_file, Solver};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap};

struct Day15Solver {
    risks: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Solver<usize> for Day15Solver {
    fn new(problem: &str) -> Self {
        let risks: Vec<Vec<u8>> = problem
            .split("\n")
            .map(|line| {
                line.chars()
                    .map(|ch| ch.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();

        let width = risks.len();
        let height = risks[0].len();
        Self {
            risks,
            width,
            height,
        }
    }

    fn solve1(&self) -> Option<usize> {
        let result = path(&self.risks, self.width, self.height);
        let result = result
            .iter()
            .skip(1)
            .map(|(x, y)| self.risks[*x][*y] as usize)
            .sum();
        Some(result)
    }

    fn solve2(&self) -> Option<usize> {
        let mut risks: Vec<Vec<u8>> = self
            .risks
            .iter()
            .map(|line| line.iter().cycle().take(line.len() * 5).cloned().collect())
            .cycle()
            .take(self.risks.len() * 5)
            .collect();
        for i in 0..5 {
            for j in 0..5 {
                for x in 0..self.width {
                    for y in 0..self.height {
                        let dx = i * self.width + x;
                        let dy = j * self.height + y;

                        let mut score = risks[dx][dy] + i as u8 + j as u8;
                        while score > 9 {
                            score -= 9;
                        }
                        risks[dx][dy] = score;
                    }
                }
            }
        }

        let result = path(&risks, self.width * 5, self.height * 5);
        let result = result
            .iter()
            .skip(1)
            .map(|(x, y)| risks[*x][*y] as usize)
            .sum();
        Some(result)
    }
}

fn path(risks: &Vec<Vec<u8>>, width: usize, height: usize) -> Vec<(usize, usize)> {
    let source = (0, 0);
    let dest = (width - 1, height - 1);

    let mut visited = BTreeMap::new();
    let mut candidates = BinaryHeap::new();
    candidates.push(Candidate {
        point: source,
        cost: 0,
        previous: None,
    });

    loop {
        let current = candidates.pop().unwrap();

        if visited.contains_key(&current.point) {
            continue;
        }
        visited.insert(current.point, current.clone());

        if current.point == dest {
            // found!
            let mut path = vec![];
            let mut current = &current;

            while let Some(previous) = &current.previous {
                path.push(current.point);
                current = visited.get(previous).unwrap();
            }
            path.push(current.point);
            path.reverse();

            return path;
        }

        let mut new = vec![];
        let (x, y) = current.point;
        if x > 0 {
            new.push(Candidate {
                point: (x - 1, y),
                cost: current.cost + risks[x - 1][y] as usize,
                previous: Some(current.point),
            });
        }
        if x < width - 1 {
            new.push(Candidate {
                point: (x + 1, y),
                cost: current.cost + risks[x + 1][y] as usize,
                previous: Some(current.point),
            });
        }
        if y > 0 {
            new.push(Candidate {
                point: (x, y - 1),
                cost: current.cost + risks[x][y - 1] as usize,
                previous: Some(current.point),
            });
        }
        if y < height - 1 {
            new.push(Candidate {
                point: (x, y + 1),
                cost: current.cost + risks[x][y + 1] as usize,
                previous: Some(current.point),
            });
        }

        for n in new {
            if visited.get(&n.point) == None {
                candidates.push(n);
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Candidate {
    previous: Option<(usize, usize)>,
    point: (usize, usize),
    cost: usize,
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod day15tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_DATA: &'static str = indoc!(
        "
        1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581
    "
    );

    #[test]
    fn test_example_part1() {
        let solver = Day15Solver::new(EXAMPLE_DATA.trim());
        assert_eq!(solver.solve1(), Some(40));
    }

    #[test]
    fn test_example_part2() {
        let solver = Day15Solver::new(EXAMPLE_DATA.trim());
        assert_eq!(solver.solve2(), Some(315));
    }
}

fn main() {
    solve_file::<Day15Solver, usize>("day15.txt");
}
