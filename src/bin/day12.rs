use std::collections::HashMap;
use std::fmt;

use advent_of_code_2021::solver::{solve_file, Solver};

struct Day12Solver {
    map: HashMap<Cave, Vec<Cave>>,
}

impl Solver<usize> for Day12Solver {
    fn new(problem: &str) -> Self {
        let conns = problem.split("\n").map(|line| {
            let (start, end) = line.split_once("-").unwrap();
            let (start, end) = (Cave::new(start), Cave::new(end));
            (start, end)
        });
        let mut map: HashMap<Cave, Vec<Cave>> = HashMap::new();
        for (start, end) in conns {
            if map.contains_key(&start) {
                map.get_mut(&start).unwrap().push(end.clone());
            } else {
                map.insert(start.clone(), vec![end.clone()]);
            }

            if map.contains_key(&end) {
                map.get_mut(&end).unwrap().push(start.clone());
            } else {
                map.insert(end.clone(), vec![start.clone()]);
            }
        }

        Self { map }
    }

    fn solve1(&self) -> Option<usize> {
        let paths = self.visit(vec![&Cave::Start], 0);
        Some(paths.len())
    }

    fn solve2(&self) -> Option<usize> {
        let paths = self.visit(vec![&Cave::Start], 1);
        Some(paths.len())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Cave {
    Start,
    End,
    Small(String),
    Large(String),
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cave::Start => write!(f, "start"),
            Cave::End => write!(f, "end"),
            Cave::Small(name) => write!(f, "{}", name),
            Cave::Large(name) => write!(f, "{}", name),
        }
    }
}

impl Cave {
    fn new(name: &str) -> Self {
        match name {
            "start" => Cave::Start,
            "end" => Cave::End,
            name if name.chars().all(char::is_lowercase) => Cave::Small(name.to_string()),
            name if name.chars().all(char::is_uppercase) => Cave::Large(name.to_string()),
            _ => panic!("invalid cave name"),
        }
    }
}

impl Day12Solver {
    fn visit<'a>(&'a self, path: Vec<&'a Cave>, small_visits: usize) -> Vec<Vec<&'a Cave>> {
        assert!(path.len() > 0);

        let mut paths = vec![];
        for next in self.map.get(path.last().unwrap()).unwrap() {
            match next {
                Cave::End => {
                    let mut npath = path.clone();
                    npath.push(&next);
                    paths.push(npath);
                }
                Cave::Large(_) => {
                    let mut npath = path.clone();
                    npath.push(&next);
                    paths.extend_from_slice(&self.visit(npath, small_visits));
                }
                Cave::Small(_) => {
                    if !path.contains(&next) {
                        let mut npath = path.clone();
                        npath.push(&next);
                        paths.extend_from_slice(&self.visit(npath, small_visits));
                    } else if small_visits > 0 {
                        let mut npath = path.clone();
                        npath.push(&next);
                        paths.extend_from_slice(&self.visit(npath, small_visits - 1));
                    }
                }
                _ => {}
            }
        }
        paths
    }
}

#[cfg(test)]
mod day12tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_DATA: &'static str = indoc!(
        "
        start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
    "
    );
    static EXAMPLE_DATA2: &'static str = indoc!(
        "
        dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc
    "
    );
    static EXAMPLE_DATA3: &'static str = indoc!(
        "
        fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW
    "
    );

    #[test]
    fn test_example_part1() {
        let solver = Day12Solver::new(EXAMPLE_DATA.trim());
        assert_eq!(solver.solve1(), Some(10));
        let solver = Day12Solver::new(EXAMPLE_DATA2.trim());
        assert_eq!(solver.solve1(), Some(19));
        let solver = Day12Solver::new(EXAMPLE_DATA3.trim());
        assert_eq!(solver.solve1(), Some(226));
    }

    #[test]
    fn test_example_part2() {
        let solver = Day12Solver::new(EXAMPLE_DATA.trim());
        assert_eq!(solver.solve2(), Some(36));
        let solver = Day12Solver::new(EXAMPLE_DATA2.trim());
        assert_eq!(solver.solve2(), Some(103));
        let solver = Day12Solver::new(EXAMPLE_DATA3.trim());
        assert_eq!(solver.solve2(), Some(3509));
    }
}

fn main() {
    solve_file::<Day12Solver, usize>("day12.txt");
}
