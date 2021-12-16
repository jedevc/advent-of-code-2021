use advent_of_code_2021::solver::{solve_file, Solver};

struct Day16Solver {
    root: Vec<bool>,
}

impl Solver<u64> for Day16Solver {
    fn new(problem: &str) -> Self {
        let root: Vec<bool> = problem
            .chars()
            .map(|ch| ch.to_digit(16).unwrap() as u8)
            .flat_map(|n| vec![n & 8 != 0, n & 4 != 0, n & 2 != 0, n & 1 != 0])
            .collect();
        Self { root }
    }

    fn solve1(&self) -> Option<u64> {
        let mut parser = Parser::new(&self.root);
        let packet = parser.parse();
        Some(packet.version_sums())
    }

    fn solve2(&self) -> Option<u64> {
        let mut parser = Parser::new(&self.root);
        let packet = parser.parse();
        Some(packet.calculate())
    }
}

struct Parser<'a> {
    position: usize,
    data: &'a [bool],
}

impl<'a> Parser<'a> {
    fn new(data: &'a [bool]) -> Self {
        Self { data, position: 0 }
    }

    fn parse(&mut self) -> Packet {
        self.parse_packet()
    }

    fn parse_packet(&mut self) -> Packet {
        let version = self.parse_uint(3) as u8;
        let type_id = self.parse_uint(3) as u8;

        if type_id == 4 {
            // literal
            let mut chunks = vec![];

            while self.parse_bool() {
                let chunk = self.parse_uint(4) as u8;
                chunks.push(chunk);
            }

            let chunk = self.parse_uint(4) as u8;
            chunks.push(chunk);

            Packet {
                version,
                data: PacketData::Literal { chunks },
            }
        } else {
            // operator
            let subpackets = if self.parse_bool() {
                // sub-packet length
                let length = self.parse_uint(11);
                (0..length).map(|_| self.parse_packet()).collect()
            } else {
                // total bit length
                let length = self.parse_uint(15) as usize;
                let target = self.position + length;

                let mut subpackets = vec![];
                while self.position < target {
                    subpackets.push(self.parse_packet())
                }
                assert_eq!(target, self.position);
                subpackets
            };

            let op = match type_id {
                0 => Operator::Sum,
                1 => Operator::Product,
                2 => Operator::Minimum,
                3 => Operator::Maximum,
                5 => Operator::Greater,
                6 => Operator::Less,
                7 => Operator::Equal,
                _ => panic!("invalid operator"),
            };

            Packet {
                version,
                data: PacketData::Operator { op, subpackets },
            }
        }
    }

    fn parse_uint(&mut self, n: usize) -> u64 {
        assert!(n <= 64);

        let contents = self.parse_data(n);
        contents
            .iter()
            .fold(0, |acc, on| acc << 1 | if *on { 1 } else { 0 })
    }

    fn parse_data(&mut self, n: usize) -> Vec<bool> {
        assert!(self.position < self.data.len());

        let start = self.position;
        let end = usize::min(self.position + n, self.data.len());
        let mut data = self.data[start..end].to_vec();
        while data.len() < n {
            data.push(false);
        }

        self.position += n;
        data
    }

    fn parse_bool(&mut self) -> bool {
        self.position += 1;
        self.data[self.position - 1]
    }
}

struct Packet {
    version: u8,
    data: PacketData,
}

enum PacketData {
    Literal {
        chunks: Vec<u8>,
    },
    Operator {
        op: Operator,
        subpackets: Vec<Packet>,
    },
}

enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    Greater,
    Less,
    Equal,
}

impl Packet {
    fn version_sums(&self) -> u64 {
        self.version as u64
            + match &self.data {
                PacketData::Literal { .. } => 0,
                PacketData::Operator { subpackets, .. } => {
                    subpackets.iter().map(Packet::version_sums).sum()
                }
            }
    }

    fn calculate(&self) -> u64 {
        match &self.data {
            PacketData::Literal { chunks } => chunks.iter().fold(0, |acc, n| acc << 4 | *n as u64),
            PacketData::Operator { op, subpackets } => {
                let mut results = subpackets.iter().map(Packet::calculate);
                match *op {
                    Operator::Sum => results.sum(),
                    Operator::Product => results.product(),
                    Operator::Minimum => results.min().unwrap(),
                    Operator::Maximum => results.max().unwrap(),
                    Operator::Greater => (results.next() > results.next()) as u64,
                    Operator::Less => (results.next() < results.next()) as u64,
                    Operator::Equal => (results.next() == results.next()) as u64,
                }
            }
        }
    }
}

#[cfg(test)]
mod day16tests {
    use super::*;

    #[test]
    fn test_new() {
        let solver = Day16Solver::new("03AF");
        assert_eq!(
            solver.root,
            vec![
                false, false, false, false, false, false, true, true, true, false, true, false,
                true, true, true, true
            ]
        );
    }

    #[test]
    fn test_example_part1() {
        let solver = Day16Solver::new("8A004A801A8002F478");
        assert_eq!(solver.solve1(), Some(16));
        let solver = Day16Solver::new("620080001611562C8802118E34");
        assert_eq!(solver.solve1(), Some(12));
        let solver = Day16Solver::new("C0015000016115A2E0802F182340");
        assert_eq!(solver.solve1(), Some(23));
        let solver = Day16Solver::new("A0016C880162017C3686B18A3D4780");
        assert_eq!(solver.solve1(), Some(31));
    }

    #[test]
    fn test_example_part2() {
        let solver = Day16Solver::new("C200B40A82");
        assert_eq!(solver.solve2(), Some(3));
        let solver = Day16Solver::new("04005AC33890");
        assert_eq!(solver.solve2(), Some(54));
        let solver = Day16Solver::new("880086C3E88112");
        assert_eq!(solver.solve2(), Some(7));
        let solver = Day16Solver::new("CE00C43D881120");
        assert_eq!(solver.solve2(), Some(9));
        let solver = Day16Solver::new("D8005AC2A8F0");
        assert_eq!(solver.solve2(), Some(1));
        let solver = Day16Solver::new("F600BC2D8F");
        assert_eq!(solver.solve2(), Some(0));
        let solver = Day16Solver::new("9C005AC2F8F0");
        assert_eq!(solver.solve2(), Some(0));
        let solver = Day16Solver::new("9C0141080250320F1802104A08");
        assert_eq!(solver.solve2(), Some(1));
    }
}

fn main() {
    solve_file::<Day16Solver, u64>("day16.txt");
}
