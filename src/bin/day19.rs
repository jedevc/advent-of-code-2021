use std::collections::HashSet;

use advent_of_code_2021::solver::{solve_file, Solver};

struct Day19Solver {
    scanners: Vec<Scanner>,
}

impl Solver<u64> for Day19Solver {
    fn new(problem: &str) -> Self {
        let scanners = problem
            .split("\n\n")
            .map(|scanner| {
                let beacons = scanner
                    .split("\n")
                    .skip(1)
                    .map(|line| {
                        let points: Vec<&str> = line.split(",").collect();
                        let x = points[0].parse().unwrap();
                        let y = points[1].parse().unwrap();
                        let z = points[2].parse().unwrap();
                        Beacon { x, y, z }
                    })
                    .collect();
                Scanner { beacons }
            })
            .collect();
        Self { scanners }
    }

    fn solve1(&self) -> Option<u64> {
        let solves = self.solve();

        // find unique points
        let mut results = HashSet::new();
        for solve in &solves {
            for beacon in &solve.scanner.beacons {
                results.insert(*beacon);
            }
        }

        Some(results.len() as u64)
    }

    fn solve2(&self) -> Option<u64> {
        let solves = self.solve();

        // find the lowest manhattan distance between scanners
        let mut best = 0;
        for i in &solves {
            for j in &solves {
                let (ax, ay, az) = i.offset;
                let (bx, by, bz) = j.offset;
                let manhattan = i32::abs(ax - bx) + i32::abs(ay - by) + i32::abs(az - bz);
                best = best.max(manhattan);
            }
        }

        Some(best as u64)
    }
}

impl Day19Solver {
    fn solve(&self) -> Vec<Solve> {
        // track the found scanners (with known relative positions)
        let mut found = vec![Solve {
            scanner: self.scanners[0].clone(),
            offset: (0, 0, 0),
        }];

        // track the un-found scanners (with unknown positions)
        let mut not_found: Vec<&Scanner> = self.scanners.iter().skip(1).collect();

        while not_found.len() > 0 {
            // select an un-found scanner, and attempt to find it
            let target = not_found.pop().unwrap();
            let mut added = false;

            'search: for i in 0..found.len() {
                for mut rotation in target.rotations() {
                    // for every possible rotation of the target scanner, attempt to fit it to a
                    // found scanner
                    if let Some(offset) = found[i].scanner.intersect(&rotation) {
                        // translate all the points in the rotated target scanner relative to the
                        // one we fit it to, and mark it as found
                        rotation.translate(offset);
                        found.push(Solve {
                            scanner: rotation,
                            offset,
                        });

                        added = true;
                        break 'search;
                    }
                }
            }

            // it's possible that we can't properly fit the scanner at all, so add it back to the
            // collection for later
            if !added {
                not_found.insert(0, target);
            }
        }

        found
    }
}

struct Solve {
    offset: (i32, i32, i32),
    scanner: Scanner,
}

#[derive(Clone)]
struct Scanner {
    beacons: Vec<Beacon>,
}

impl Scanner {
    fn translate(&mut self, (dx, dy, dz): (i32, i32, i32)) {
        for beacon in self.beacons.iter_mut() {
            beacon.x += dx;
            beacon.y += dy;
            beacon.z += dz;
        }
    }

    fn intersect(&self, other: &Scanner) -> Option<(i32, i32, i32)> {
        let beacons: HashSet<Beacon> = self.beacons.iter().cloned().collect();

        // consider each pair of co-ordinates and assume those two beacons are the same
        for i in &self.beacons {
            for j in &other.beacons {
                let offset = (i.x - j.x, i.y - j.y, i.z - j.z);
                let mut other = other.clone();
                other.translate(offset);

                let count = other
                    .beacons
                    .iter()
                    .filter(|beacon| beacons.contains(beacon))
                    .count();
                if count >= 12 {
                    return Some(offset);
                }
            }
        }

        None
    }

    fn rotations(&self) -> Vec<Scanner> {
        let mut rotations = vec![];
        for i in &[-1, 1] {
            for j in &[-1, 1] {
                for k in &[-1, 1] {
                    rotations.push(Scanner {
                        beacons: self
                            .beacons
                            .iter()
                            .map(|Beacon { x, y, z }| Beacon {
                                x: i * x,
                                y: j * y,
                                z: k * z,
                            })
                            .collect(),
                    });
                    rotations.push(Scanner {
                        beacons: self
                            .beacons
                            .iter()
                            .map(|Beacon { x, y, z }| Beacon {
                                x: i * x,
                                y: j * z,
                                z: k * y,
                            })
                            .collect(),
                    });
                    rotations.push(Scanner {
                        beacons: self
                            .beacons
                            .iter()
                            .map(|Beacon { x, y, z }| Beacon {
                                x: i * y,
                                y: j * x,
                                z: k * z,
                            })
                            .collect(),
                    });
                    rotations.push(Scanner {
                        beacons: self
                            .beacons
                            .iter()
                            .map(|Beacon { x, y, z }| Beacon {
                                x: i * y,
                                y: j * z,
                                z: k * x,
                            })
                            .collect(),
                    });
                    rotations.push(Scanner {
                        beacons: self
                            .beacons
                            .iter()
                            .map(|Beacon { x, y, z }| Beacon {
                                x: i * z,
                                y: j * x,
                                z: k * y,
                            })
                            .collect(),
                    });
                    rotations.push(Scanner {
                        beacons: self
                            .beacons
                            .iter()
                            .map(|Beacon { x, y, z }| Beacon {
                                x: i * z,
                                y: j * y,
                                z: k * x,
                            })
                            .collect(),
                    });
                }
            }
        }

        rotations
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Clone, Copy)]
struct Beacon {
    x: i32,
    y: i32,
    z: i32,
}

#[cfg(test)]
mod day19tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_DATA: &'static str = indoc!(
        "
        --- scanner 0 ---
        404,-588,-901
        528,-643,409
        -838,591,734
        390,-675,-793
        -537,-823,-458
        -485,-357,347
        -345,-311,381
        -661,-816,-575
        -876,649,763
        -618,-824,-621
        553,345,-567
        474,580,667
        -447,-329,318
        -584,868,-557
        544,-627,-890
        564,392,-477
        455,729,728
        -892,524,684
        -689,845,-530
        423,-701,434
        7,-33,-71
        630,319,-379
        443,580,662
        -789,900,-551
        459,-707,401
        
        --- scanner 1 ---
        686,422,578
        605,423,415
        515,917,-361
        -336,658,858
        95,138,22
        -476,619,847
        -340,-569,-846
        567,-361,727
        -460,603,-452
        669,-402,600
        729,430,532
        -500,-761,534
        -322,571,750
        -466,-666,-811
        -429,-592,574
        -355,545,-477
        703,-491,-529
        -328,-685,520
        413,935,-424
        -391,539,-444
        586,-435,557
        -364,-763,-893
        807,-499,-711
        755,-354,-619
        553,889,-390

        --- scanner 2 ---
        649,640,665
        682,-795,504
        -784,533,-524
        -644,584,-595
        -588,-843,648
        -30,6,44
        -674,560,763
        500,723,-460
        609,671,-379
        -555,-800,653
        -675,-892,-343
        697,-426,-610
        578,704,681
        493,664,-388
        -671,-858,530
        -667,343,800
        571,-461,-707
        -138,-166,112
        -889,563,-600
        646,-828,498
        640,759,510
        -630,509,768
        -681,-892,-333
        673,-379,-804
        -742,-814,-386
        577,-820,562

        --- scanner 3 ---
        -589,542,597
        605,-692,669
        -500,565,-823
        -660,373,557
        -458,-679,-417
        -488,449,543
        -626,468,-788
        338,-750,-386
        528,-832,-391
        562,-778,733
        -938,-730,414
        543,643,-506
        -524,371,-870
        407,773,750
        -104,29,83
        378,-903,-323
        -778,-728,485
        426,699,580
        -438,-605,-362
        -469,-447,-387
        509,732,623
        647,635,-688
        -868,-804,481
        614,-800,639
        595,780,-596

        --- scanner 4 ---
        727,592,562
        -293,-554,779
        441,611,-461
        -714,465,-776
        -743,427,-804
        -660,-479,-426
        832,-632,460
        927,-485,-438
        408,393,-506
        466,436,-512
        110,16,151
        -258,-428,682
        -393,719,612
        -211,-452,876
        808,-476,-593
        -575,615,604
        -485,667,467
        -680,325,-822
        -627,-443,-432
        872,-547,-609
        833,512,582
        807,604,487
        839,-516,451
        891,-625,532
        -652,-548,-490
        30,-46,-14
    "
    );

    #[test]
    fn test_example_part1() {
        let solver = Day19Solver::new(EXAMPLE_DATA.trim());
        assert_eq!(solver.solve1(), Some(79));
    }

    #[test]
    fn test_example_part2() {
        let solver = Day19Solver::new(EXAMPLE_DATA.trim());
        assert_eq!(solver.solve2(), Some(3621));
    }
}

fn main() {
    solve_file::<Day19Solver, u64>("day19.txt");
}
