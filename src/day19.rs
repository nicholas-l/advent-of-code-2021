use std::{collections::HashSet, fmt::Display, io::BufRead, str::FromStr};

use itertools::Itertools;
use nalgebra::{Matrix3, Point3, Vector3};

type Beacon = Point3<isize>;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Scanner {
    number: usize,
    beacons: Vec<Beacon>,
}

fn mh_distance(p1: &Beacon, other: &Beacon) -> usize {
    p1.iter()
        .zip(other.iter())
        .map(|(c1, c2)| (c1 - c2).unsigned_abs())
        .sum()
}

fn get_rotations() -> impl Iterator<Item = Matrix3<isize>> {
    [-1, 1].into_iter().flat_map(|x| {
        [-1, 1].into_iter().flat_map(move |y| {
            (0..3).permutations(3).map(move |indexes| {
                let mut x_axis = Vector3::zeros();
                x_axis[indexes[0]] = x;

                let mut y_axis = Vector3::zeros();
                y_axis[indexes[1]] = y;

                // the z axis is always the result of x.cross(y), this way we don't have invalid orientations
                let z_axis = x_axis.cross(&y_axis);

                Matrix3::from_columns(&[x_axis, y_axis, z_axis])
            })
        })
    })
}

impl Display for Scanner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Scanner {}", self.number)?;
        for b in &self.beacons {
            writeln!(f, "{:?}", b)?;
        }
        Ok(())
    }
}

impl FromStr for Scanner {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let number = {
            let l = lines.next().unwrap();
            l.split_whitespace()
                .nth(2)
                .unwrap()
                .parse::<usize>()
                .unwrap()
        };
        let beacons = lines
            .map(|line| {
                let coords = line
                    .trim()
                    .split(',')
                    .map(|x| x.parse::<isize>().unwrap())
                    .collect::<Vec<_>>();
                Point3::from([coords[0], coords[1], coords[2]])
            })
            .collect();
        Ok(Scanner { number, beacons })
    }
}

fn distance_from_to(beacons: &[Beacon]) -> Vec<(Beacon, HashSet<usize>)> {
    beacons
        .iter()
        .map(|p1| {
            let distances: HashSet<usize> = beacons.iter().map(|p2| mh_distance(p1, p2)).collect();
            (*p1, distances)
        })
        .collect()
}

fn get_matching_points(
    scanners: &[Scanner],
    positioned_distances: &[(Beacon, HashSet<usize>)],
    threshold: usize,
) -> Option<(usize, Beacon, Beacon, usize)> {
    for (reference_point, reference_distances) in positioned_distances {
        for (i, scanner) in scanners.iter().enumerate() {
            let distances = distance_from_to(&scanner.beacons);
            for (j, (scanner_point, beacon_distances)) in distances.into_iter().enumerate() {
                let count = reference_distances.intersection(&beacon_distances).count();
                if count >= threshold {
                    return Some((i, *reference_point, scanner_point, j));
                }
            }
        }
    }
    None
}

fn get_orientation(
    positioned_beacons: &HashSet<Beacon>,
    ref_point: &Beacon,
    scanner: &Scanner,
    scanner_point_i: usize,
    threshold: usize,
) -> Option<(HashSet<Beacon>, Vector3<isize>)> {
    for rotation in get_rotations() {
        let rotated_point = rotation * scanner.beacons[scanner_point_i];

        let translation = ref_point - rotated_point;
        assert_eq!(
            &(rotation * scanner.beacons[scanner_point_i] + translation),
            ref_point
        );

        let translated_beacons: HashSet<Beacon> = scanner
            .beacons
            .iter()
            .map(|b| rotation * b + translation)
            .collect();

        if positioned_beacons.intersection(&translated_beacons).count() >= threshold {
            // println!("Found rotation for scanner {}", scanner.number);
            // println!("{:?}", translated_beacons);
            return Some((translated_beacons, -translation));
        }
        // println!("\n\n");
    }
    // println!("No match for scanner {} using ", scanner.number);
    None
}

fn process(mut input: impl BufRead, threshold: usize) -> (HashSet<Beacon>, Vec<Vector3<isize>>) {
    let mut buf = String::new();
    input.read_to_string(&mut buf).unwrap();
    let mut scanners = buf
        .split("\n\n")
        .map(|scanner_section| scanner_section.parse::<Scanner>().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(get_rotations().count(), 24);

    let mut scanner_positions = vec![Vector3::from([0, 0, 0])];
    let mut positioned_beacons: HashSet<_> = scanners.remove(0).beacons.iter().cloned().collect();

    while !scanners.is_empty() {
        // Get all the distances between points that are positioned
        let v: Vec<_> = positioned_beacons.iter().cloned().collect();
        let distances = distance_from_to(&v);

        // Get a positioned point and a point from another scanner that has the same distances to at least 12 other points.
        // Assume these two points are the same beacon
        // and translate/rotate the beacons of the scanner to the translation from two points.
        let (i, ref_point, _scanner_point, scanner_point_i) =
            get_matching_points(&scanners, &distances, threshold)
                .unwrap_or_else(|| panic!("Shit went wrong: {}\n", scanners.len(),));

        let scanner = scanners.remove(i);

        let (translated_beacons, translation) = get_orientation(
            &positioned_beacons,
            &ref_point,
            &scanner,
            scanner_point_i,
            threshold,
        )
        .unwrap();
        positioned_beacons.extend(translated_beacons);
        scanner_positions.push(translation);
    }
    (positioned_beacons, scanner_positions)
}

pub fn star_one(input: impl BufRead) -> usize {
    process(input, 12).0.len()
}

pub fn star_two(input: impl BufRead) -> usize {
    process(input, 12)
        .1
        .iter()
        .combinations(2)
        .max_by_key(|a| {
            a[0].iter()
                .zip(a[1].iter())
                .map(|(c1, c2)| (c1 - c2).unsigned_abs())
                .sum::<usize>()
        })
        .map(|v| {
            v[0].iter()
                .zip(v[1].iter())
                .map(|(c1, c2)| (c1 - c2).unsigned_abs())
                .sum()
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_orientation() {
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
459,-707,401
"
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
553,889,-390
"
        .parse()
        .unwrap();
        let beacons = scanner1.beacons.iter().cloned().collect();
        let expected = HashSet::from([
            Point3::from([534, -1912, 768]),
            Point3::from([432, -2009, 850]),
            Point3::from([459, -707, 401]),
            Point3::from([390, -675, -793]),
            Point3::from([404, -588, -901]),
            Point3::from([528, -643, 409]),
            Point3::from([497, -1838, -617]),
            Point3::from([-537, -823, -458]),
            Point3::from([568, -2007, -577]),
            Point3::from([-447, -329, 318]),
            Point3::from([-687, -1600, 576]),
            Point3::from([-27, -1108, -65]),
            Point3::from([-601, -1648, -643]),
            Point3::from([-518, -1681, -600]),
            Point3::from([408, -1815, 803]),
            Point3::from([-661, -816, -575]),
            Point3::from([-345, -311, 381]),
            Point3::from([-618, -824, -621]),
            Point3::from([-485, -357, 347]),
            Point3::from([423, -701, 434]),
            Point3::from([-635, -1737, 486]),
            Point3::from([-499, -1607, -770]),
            Point3::from([-739, -1745, 668]),
            Point3::from([544, -627, -890]),
            Point3::from([396, -1931, -563]),
        ]);
        // for i in 0..scanner2.beacons.len() {
        //     let r = get_orientation(&beacons, &scanner1.beacons[9], &scanner2, i, 12);
        //     if r.is_some() {
        //         panic!("Blah{i}");
        //     }
        // }
        assert_eq!(
            get_orientation(&beacons, &scanner1.beacons[9], &scanner2, 0, 12)
                .unwrap()
                .0,
            expected
        );
    }

    #[test]
    fn test_orientations() {
        let scanner0 = "--- scanner 0 ---
0,2,0
4,1,0
3,3,0"
            .parse::<Scanner>()
            .unwrap();

        let beacons = scanner0.beacons.iter().cloned().collect();

        let scanner1: Scanner = "--- scanner 1 ---
-1,-1,0
-5,0,0
-2,1,0"
            .parse()
            .unwrap();

        assert_eq!(
            get_orientation(&beacons, &scanner0.beacons[1], &scanner1, 0, 2)
                .unwrap()
                .0,
            HashSet::from([
                Point3::from([0, 2, 0]),
                Point3::from([4, 1, 0]),
                Point3::from([3, 3, 0]),
            ])
        );
    }

    #[ignore]
    #[test]
    fn test_star_one_rot_trans() {
        let v = Point3::from([1, 2, 3]);
        for rotation in get_rotations() {
            println!("{}: {}", rotation, rotation * v);
        }
    }

    #[test]
    fn test_star_one_2d() {
        let input = b"--- scanner 0 ---
0,2,0
4,1,0
3,3,0

--- scanner 1 ---
-1,-1,0
-5,0,0
-2,1,0
";

        assert_eq!(process(Cursor::new(input), 3).0.len(), 3);
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
30,-46,-14
";
        assert_eq!(star_one(Cursor::new(input)), 79);
    }

    #[test]
    fn test_star_two() {
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
30,-46,-14
";
        assert_eq!(star_two(Cursor::new(input)), 3621);
    }
}
