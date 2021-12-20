use std::{
    collections::{hash_set::Intersection, HashSet},
    io::BufRead,
    str::FromStr,
};

use itertools::{iproduct, Itertools};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Scanner {
    beacons: HashSet<[isize; 3]>,
}

impl<'a> Scanner {
    fn rotations(&'a self) -> impl Iterator<Item = Scanner> + 'a {
        // Pemutations ()
        let rotations = iproduct!(
            (0..3).permutations(3),
            [-1, 1].iter().combinations_with_replacement(3)
        );
        rotations.map(|(position, direction)| {
            let beacons: HashSet<_> = self
                .beacons
                .iter()
                .map(|beacon| {
                    [
                        beacon[position[0]] * direction[0],
                        beacon[position[1]] * direction[1],
                        beacon[position[2]] * direction[2],
                    ]
                })
                .collect();
            Scanner { beacons }
        })
    }

    fn matches(&self, other: &Scanner) -> bool {
        self.beacons
            .iter()
            .map(|b| other.beacons.contains(b))
            .count()
            >= 12
    }

    fn overlapping_beacons(&self, other: &Scanner) -> usize {
        self.beacons.intersection(&other.beacons).count()
    }

    fn offset(&self, offset: [isize; 3]) -> Scanner {
        let beacons = self
            .beacons
            .iter()
            .map(|beacon| {
                [
                    beacon[0] + offset[0],
                    beacon[1] + offset[1],
                    beacon[2] + offset[2],
                ]
            })
            .collect();
        Scanner { beacons }
    }
}

impl FromStr for Scanner {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let beacons = s
            .lines()
            .skip(1)
            .map(|line| {
                let coords = line
                    .split(',')
                    .map(|x| x.parse::<isize>().unwrap())
                    .collect::<Vec<isize>>();
                [coords[0], coords[1], coords[2]]
            })
            .collect();
        Ok(Scanner { beacons })
    }
}

fn get_first_transformed(
    scanners: &Vec<Scanner>,
    positioned_scanners: &Vec<Scanner>,
) -> Option<(usize, Scanner)> {
    // iterate through of all placed scanners (1)
    for ref_scanner in positioned_scanners {
        // Iterate through ref point of beacons
        for b_ref in &ref_scanner.beacons {
            // Iterate through all unplaced scanner rotations
            for (i, scanner) in scanners.iter().enumerate() {
                // Iterate through scanner rotations
                for rscanner in scanner.rotations() {
                    // Iterate through all beacons of rotated scanner
                    for b_scan in &rscanner.beacons {
                        // Get offset from ref point from beacon
                        let offset = [
                            b_scan[0] - b_ref[0],
                            b_scan[1] - b_ref[1],
                            b_scan[2] - b_ref[2],
                        ];

                        let rscanner_offset = rscanner.offset(offset);

                        if rscanner_offset.overlapping_beacons(scanner) >= 12 {
                            return Some((i, rscanner));
                        }

                        // Iterate through offset beacons
                        // If 12 or more match, scanner matches, move to positioned scanners, got to (1)
                        // if positioned_scanners.iter().any(|p| p.matches(&rscanner)) {
                        //     return Some((i, rscanner));
                        // }
                    }
                }
            }
        }
    }
    return None;
}

pub fn star_one(mut input: impl BufRead) -> usize {
    let mut buf = String::new();
    input.read_to_string(&mut buf).unwrap();
    let mut scanners = buf
        .split("\n\n")
        .map(|scanner_section| scanner_section.parse::<Scanner>().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(scanners[0].rotations().count(), 24);

    let mut positioned_scanners = vec![scanners.remove(0)];

    while !scanners.is_empty() {
        if let Some((i, scanner)) = get_first_transformed(&scanners, &positioned_scanners) {
            scanners.remove(i);
            positioned_scanners.push(scanner);
        }
    }

    for (i, s) in positioned_scanners.iter().enumerate() {
        println!("Scanner {}", i);
        for b in &s.beacons {
            println!("{:?}", b);
        }
        println!("");
    }

    let hs = positioned_scanners
        .iter()
        .flat_map(|s| s.beacons.clone())
        .collect::<HashSet<_>>();

    hs.len()
}

pub fn star_two(_input: impl BufRead) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_rotations() {
        let scanner: Scanner = "--- scanner 0 ---
-1,-1,1
-2,-2,2
-3,-3,3
-2,-3,1
5,6,-4
8,0,7"
            .parse()
            .unwrap();

        let expected: Scanner = "--- scanner 0 ---
1,-1,1
2,-2,2
3,-3,3
2,-1,3
-5,4,-6
-8,-7,0"
            .parse()
            .unwrap();
        let r: Vec<_> = scanner.rotations().collect();
        assert!(r.contains(&scanner));
        assert!(r.contains(&expected));
    }

    #[test]
    fn test_offset() {
        let scanner: Scanner = "--- scanner 0 ---
-1,-1,1
-2,-2,2"
            .parse()
            .unwrap();

        let expected: Scanner = "--- scanner 0 ---
0,1,4
-1,0,5"
            .parse()
            .unwrap();
        assert_eq!(scanner.offset([1, 2, 3]), expected);
    }

    #[ignore = "reason"]
    #[test]
    fn test_overlap() {
        let scanner1: Scanner = "--- scanner 0 ---
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
459,-707,401"
            .parse()
            .unwrap();

        let scanner2: Scanner = "--- scanner 1 ---
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
553,889,-390"
            .parse()
            .unwrap();

            assert_eq!(scanner1.overlapping_beacons(&scanner2), 12)
    }

    #[test]
    fn test_star_one() {
        let input = b"--- scanner 0 ---
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
30,-46,-14";
        assert_eq!(star_one(Cursor::new(input)), 79);
    }

    #[test]
    fn test_star_two() {
        let input = b"";
        assert_eq!(star_two(Cursor::new(input)), 3993);
    }
}
