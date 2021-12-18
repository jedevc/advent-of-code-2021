use std::ops::Add;
use std::str;

use advent_of_code_2021::solver::{solve_file, Solver};

struct Day18Solver {
    snails: Vec<Snail>,
}

impl Solver<u64> for Day18Solver {
    fn new(problem: &str) -> Self {
        let snails = problem.split("\n").map(Snail::parse).collect();
        Self { snails }
    }

    fn solve1(&self) -> Option<u64> {
        let result = self
            .snails
            .clone()
            .into_iter()
            .reduce(|left, right| {
                let mut result = &left + &right;
                result.reduce_all();
                result
            })
            .unwrap();
        Some(result.magnitude())
    }

    fn solve2(&self) -> Option<u64> {
        let mut max_magnitude = 0;
        for snail1 in &self.snails {
            for snail2 in &self.snails {
                let mut result = snail1 + snail2;
                result.reduce_all();
                max_magnitude = max_magnitude.max(result.magnitude());
            }
        }
        Some(max_magnitude)
    }
}

#[derive(Debug, PartialEq, Clone)]
enum SnailItem {
    Open,
    Close,
    Number(u8),
}

#[derive(Debug, PartialEq, Clone)]
struct Snail {
    items: Vec<SnailItem>,
}

impl Snail {
    fn parse(s: &str) -> Self {
        let s: Vec<char> = s.chars().collect();
        let mut i = 0;

        let mut items = vec![];
        while i < s.len() {
            match s[i] {
                '[' => items.push(SnailItem::Open),
                ']' => items.push(SnailItem::Close),
                ',' => {}
                '0'..='9' => {
                    let mut j = i + 1;
                    while j < s.len() && s[j] >= '0' && s[j] <= '9' {
                        j += 1;
                    }
                    items.push(SnailItem::Number(
                        s[i..j].iter().collect::<String>().parse().unwrap(),
                    ));
                    i = j - 1;
                }
                _ => unreachable!(),
            }
            i += 1;
        }
        Self { items }
    }

    fn magnitude(&self) -> u64 {
        let mut stack = vec![];
        let mut i = 0;
        while i < self.items.len() {
            match self.items[i] {
                SnailItem::Number(n) => stack.push(n as u64),
                SnailItem::Close => {
                    let next = 2 * stack.pop().unwrap() + 3 * stack.pop().unwrap();
                    stack.push(next);
                }
                _ => {}
            }
            i += 1;
        }

        assert_eq!(stack.len(), 1);
        stack[0]
    }

    fn reduce_all(&mut self) {
        let mut repeat = true;
        while repeat {
            repeat = self.reduce();
        }
    }

    fn reduce(&mut self) -> bool {
        // explode
        let mut count = 0;
        for i in 0..self.items.len() {
            match self.items[i] {
                SnailItem::Open => count += 1,
                SnailItem::Close => count -= 1,
                SnailItem::Number(n) if count > 4 => {
                    // explode to left
                    for j in (0..i).rev() {
                        if let SnailItem::Number(m) = self.items[j] {
                            self.items[j] = SnailItem::Number(n + m);
                            break;
                        }
                    }

                    // shift to next
                    self.items.remove(i);
                    let n = if let SnailItem::Number(n) = self.items[i] {
                        n
                    } else {
                        // our input will never nest like this
                        unreachable!();
                    };

                    // explode to right
                    for j in i + 1..self.items.len() {
                        if let SnailItem::Number(m) = self.items[j] {
                            self.items[j] = SnailItem::Number(n + m);
                            break;
                        }
                    }
                    self.items.remove(i);

                    // remove open and close
                    self.items.remove(i);
                    self.items.remove(i - 1);

                    // add 0
                    self.items.insert(i - 1, SnailItem::Number(0));

                    return true;
                }
                _ => {}
            }
        }

        // split
        for i in 0..self.items.len() {
            if let SnailItem::Number(n) = self.items[i] {
                if n >= 10 {
                    // split numbers
                    let half = n / 2;
                    self.items[i] = SnailItem::Number(half);
                    self.items.insert(i + 1, SnailItem::Number(n - half));

                    // insert brackets
                    self.items.insert(i, SnailItem::Open);
                    self.items.insert(i + 3, SnailItem::Close);

                    return true;
                }
            }
        }

        false
    }
}

impl Add for &Snail {
    type Output = Snail;

    fn add(self, other: Self) -> Snail {
        let mut items = vec![];
        items.push(SnailItem::Open);
        items.extend_from_slice(&self.items);
        items.extend_from_slice(&other.items);
        items.push(SnailItem::Close);

        Snail { items }
    }
}

#[cfg(test)]
mod day18tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse() {
        let snail = Snail::parse("[1,2]");
        assert_eq!(
            snail.items,
            vec![
                SnailItem::Open,
                SnailItem::Number(1),
                SnailItem::Number(2),
                SnailItem::Close
            ]
        );
        let snail = Snail::parse("[[1,2],3]");
        assert_eq!(
            snail.items,
            vec![
                SnailItem::Open,
                SnailItem::Open,
                SnailItem::Number(1),
                SnailItem::Number(2),
                SnailItem::Close,
                SnailItem::Number(3),
                SnailItem::Close
            ]
        );
    }

    #[test]
    fn test_explode() {
        let mut snail = Snail::parse("[1,2]");
        assert!(!snail.reduce());

        let mut snail = Snail::parse("[[[[[9,8],1],2],3],4]");
        assert!(snail.reduce());
        assert_eq!(snail, Snail::parse("[[[[0,9],2],3],4]"));

        let mut snail = Snail::parse("[7,[6,[5,[4,[3,2]]]]]");
        assert!(snail.reduce());
        assert_eq!(snail, Snail::parse("[7,[6,[5,[7,0]]]]"));

        let mut snail = Snail::parse("[[6,[5,[4,[3,2]]]],1]");
        assert!(snail.reduce());
        assert_eq!(snail, Snail::parse("[[6,[5,[7,0]]],3]"));

        let mut snail = Snail::parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        assert!(snail.reduce());
        assert_eq!(snail, Snail::parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"));

        let mut snail = Snail::parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        assert!(snail.reduce());
        assert_eq!(snail, Snail::parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"));
    }

    #[test]
    fn test_add() {
        let left = Snail::parse("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let right = Snail::parse("[1,1]");
        let mut result = &left + &right;
        assert_eq!(
            result,
            Snail::parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]")
        );
        assert!(result.reduce());
        assert_eq!(result, Snail::parse("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]"));
        assert!(result.reduce());
        assert_eq!(result, Snail::parse("[[[[0,7],4],[15,[0,13]]],[1,1]]"));
        assert!(result.reduce());
        assert_eq!(result, Snail::parse("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"));
        assert!(result.reduce());
        assert_eq!(
            result,
            Snail::parse("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]")
        );
        assert!(result.reduce());
        assert_eq!(result, Snail::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));

        let mut result = &left + &right;
        result.reduce_all();
        assert_eq!(result, Snail::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
    }

    static EXAMPLE_DATA: &'static str = indoc!(
        "
        [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
    "
    );

    #[test]
    fn test_example_part1() {
        let solver = Day18Solver::new(EXAMPLE_DATA.trim());
        assert_eq!(solver.solve1(), Some(4140));
    }

    #[test]
    fn test_example_part2() {
        let solver = Day18Solver::new(EXAMPLE_DATA.trim());
        assert_eq!(solver.solve2(), Some(3993));
    }
}

fn main() {
    solve_file::<Day18Solver, u64>("day18.txt");
}
