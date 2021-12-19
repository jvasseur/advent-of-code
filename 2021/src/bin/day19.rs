use advent_of_code_2021::{parse, read};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::character::complete::newline;
use nom::character::complete::u8;
use nom::multi::many0;
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;
use std::collections::HashSet;

#[macro_use]
extern crate lazy_static;

fn scanner_parser(input: &str) -> IResult<&str, Scanner> {
    let (input, (_, index, _, points)) =
        tuple((tag("--- scanner "), u8, tag(" ---\n"), many0(point_parser)))(input)?;

    Ok((
        input,
        Scanner {
            index,
            points: points.into_iter().collect(),
        },
    ))
}

fn point_parser(input: &str) -> IResult<&str, Point> {
    let (input, (x, _, y, _, z, _)) = tuple((i32, tag(","), i32, tag(","), i32, newline))(input)?;

    Ok((input, (x, y, z)))
}

type Point = (i32, i32, i32);

type Rotation = [[i32; 3]; 3];

static IDENTITY: Rotation = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];

static ROTATE_X: Rotation = [[1, 0, 0], [0, 0, -1], [0, 1, 0]];

static ROTATE_Y: Rotation = [[0, 0, 1], [0, 1, 0], [-1, 0, 0]];

static ROTATE_Z: Rotation = [[0, -1, 0], [1, 0, 0], [0, 0, 1]];

lazy_static! {
    static ref ROTATIONS: HashSet<Rotation> = {
        let mut set = HashSet::new();

        for rx in 0..4 {
            for ry in 0..4 {
                for rz in 0..4 {
                    let mut rotation = IDENTITY;

                    for _ in 0..rx {
                        rotation = product(rotation, ROTATE_X);
                    }

                    for _ in 0..ry {
                        rotation = product(rotation, ROTATE_Y);
                    }

                    for _ in 0..rz {
                        rotation = product(rotation, ROTATE_Z);
                    }

                    set.insert(rotation);
                }
            }
        }

        set
    };
}

fn product(a: Rotation, b: Rotation) -> Rotation {
    let mut c = [[0; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            c[i][j] = (0..3).map(|k| a[i][k] * b[k][j]).sum();
        }
    }

    c
}

fn rotate((x, y, z): &Point, rotation: &Rotation) -> Point {
    (
        rotation[0][0] * x + rotation[0][1] * y + rotation[0][2] * z,
        rotation[1][0] * x + rotation[1][1] * y + rotation[1][2] * z,
        rotation[2][0] * x + rotation[2][1] * y + rotation[2][2] * z,
    )
}

trait Map {
    fn has(&self, point: &Point) -> bool;
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Scanner {
    index: u8,
    points: HashSet<Point>,
}

impl Scanner {
    fn rotations(&self) -> Vec<Self> {
        ROTATIONS
            .iter()
            .map(|rotation| Self {
                index: self.index,
                points: self
                    .points
                    .iter()
                    .map(|point| rotate(point, rotation))
                    .collect(),
            })
            .collect()
    }

    fn translate(&self, translation: Point) -> Translated {
        Translated {
            scanner: self,
            translation,
        }
    }
}

impl Map for Scanner {
    fn has(&self, point: &Point) -> bool {
        self.points.contains(point)
    }
}

struct Translated<'a> {
    scanner: &'a Scanner,
    translation: Point,
}

impl Translated<'_> {
    fn points(&self) -> HashSet<Point> {
        let (dx, dy, dz) = self.translation;

        self.scanner
            .points
            .iter()
            .map(|(x, y, z)| (x + dx, y + dy, z + dz))
            .collect()
    }

    fn intersect(&self, second: &impl Map) -> bool {
        let mut intersection = 0;

        for point in &self.points() {
            if second.has(point) {
                intersection += 1;

                if intersection >= 12 {
                    return true;
                }
            }
        }

        return false;
    }
}

impl Map for Translated<'_> {
    fn has(&self, (x, y, z): &Point) -> bool {
        let (dx, dy, dz) = self.translation;

        self.scanner.has(&(x - dx, y - dy, z - dz))
    }
}

fn solve(input: &[Scanner]) -> (usize, i32) {
    let mut positions = Vec::new();

    let mut remainings = input.to_owned();

    let mut map = remainings.swap_remove(0);

    'main: while remainings.len() > 0 {
        println!("{} remaining", remainings.len());

        for (i, remaining) in remainings.iter().enumerate() {
            println!("testing {}", i);

            for rotation in remaining.rotations() {
                for (x1, y1, z1) in &rotation.points {
                    for (x2, y2, z2) in &map.points {
                        // x1 + xt = x2
                        // xt = x2 - x1
                        let translation = (x2 - x1, y2 - y1, z2 - z1);

                        let translated = rotation.translate(translation);

                        if translated.intersect(&map) {
                            positions.push(translation);
                            map.points.extend(&translated.points());

                            remainings.swap_remove(i);

                            continue 'main;
                        }
                    }
                }
            }
        }

        panic!("Something bad is appenning!");
    }

    (
        map.points.len(),
        positions
            .into_iter()
            .tuple_combinations()
            .map(|(a, b)| (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs())
            .max()
            .unwrap(),
    )
}

fn main() {
    let input = read(19);

    let parsed_input = parse(separated_list0(newline, scanner_parser), &input);

    let (part1, part2) = solve(&parsed_input);

    println!("{}", part1);
    println!("{}", part2);
}

#[cfg(test)]
mod tests {
    use super::product;
    use super::rotate;
    use super::scanner_parser;
    use super::solve;
    use super::Map;
    use super::Scanner;
    use super::ROTATE_X;
    use super::ROTATIONS;
    use nom::character::complete::newline;
    use nom::multi::separated_list0;
    use nom::Finish;
    use std::collections::HashSet;

    static TEXT: &str = "--- scanner 0 ---
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

    #[test]
    fn test_scanner_parser() {
        assert_eq!(
            scanner_parser(
                "--- scanner 0 ---\n-1,-1,1\n-2,-2,2\n-3,-3,3\n-2,-3,1\n5,6,-4\n8,0,7\n"
            ),
            Ok((
                "",
                Scanner {
                    index: 0,
                    points: HashSet::from([
                        (-1, -1, 1),
                        (-2, -2, 2),
                        (-3, -3, 3),
                        (-2, -3, 1),
                        (5, 6, -4),
                        (8, 0, 7),
                    ]),
                }
            ))
        );
    }

    #[test]
    fn test_rotate() {
        assert_eq!(rotate(&(1, 2, 3), &ROTATE_X), (1, -3, 2));
    }

    #[test]
    fn test_product() {
        assert_eq!(
            product(ROTATE_X, ROTATE_X),
            [[1, 0, 0], [0, -1, 0], [0, 0, -1],]
        );
    }

    #[test]
    fn test_rotations() {
        assert_eq!(ROTATIONS.len(), 24);
    }

    #[test]
    fn test_translate_1() {
        let scanner = Scanner {
            index: 0,
            points: HashSet::from([(0, 0, 0), (-4, -5, -6)]),
        };

        let translated = scanner.translate((1, 2, 3));

        assert!(translated.has(&(1, 2, 3)));
        assert_eq!(
            translated.points(),
            HashSet::from([(1, 2, 3), (-3, -3, -3),])
        );
    }

    #[test]
    fn test_solve() {
        let (_, input) = separated_list0(newline, scanner_parser)(TEXT)
            .finish()
            .unwrap();

        assert_eq!(solve(&input), (79, 3621));
    }
}
