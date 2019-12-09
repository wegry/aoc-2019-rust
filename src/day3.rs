use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::prelude::*;

fn inclusive_range_from_pair<T: Ord>(x: T, y: T) -> std::ops::RangeInclusive<T> {
    if x > y {
        y..=x
    } else {
        x..=y
    }
}

#[derive(Clone, Copy, Debug, PartialOrd, Ord, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance_from_center(self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn distance_from(self, other: Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn point_on_line(self, start: Point, end: Point) -> bool {
        if start.x == end.x && end.x == self.x {
            inclusive_range_from_pair(start.y, end.y).contains(&self.y)
        } else if start.y == end.y && end.y == self.y {
            inclusive_range_from_pair(start.x, end.x).contains(&self.x)
        } else {
            false
        }
    }
}

fn parse_move(move_vector: &str, previous_point: Point) -> Point {
    let chars = move_vector.chars().collect::<Vec<char>>();
    let direction = chars.get(0).unwrap();
    let distance = chars[1..]
        .iter()
        .collect::<String>()
        .parse::<i32>()
        .unwrap();

    let (x_change, y_change) = match direction {
        'U' => (0, distance),
        'D' => (0, -distance),
        'R' => (distance, 0),
        'L' => (-distance, 0),
        other => {
            panic!("{} is not a valid direction", other);
        }
    };

    Point {
        x: previous_point.x + x_change,
        y: previous_point.y + y_change,
    }
}

fn parse_line(line: &str) -> Vec<Point> {
    line.split(',')
        .fold(vec![Point { x: 0, y: 0 }], |mut acc, curr| {
            let next_move = parse_move(curr, acc[acc.len() - 1]);

            acc.push(next_move);
            acc
        })
        .iter()
        .filter(|Point { x, y }| !(*x == 0 && *y == 0))
        .cloned()
        .collect()
}

fn parse_lines(lines: &str) -> Vec<Vec<Point>> {
    lines.lines().map(parse_line).collect()
}

fn wire_points(wire: Vec<Point>) -> BTreeSet<Point> {
    let mut covered_points = BTreeSet::new();

    for window in wire.windows(2) {
        if let [p1, p2] = window {
            if p1.x == p2.x {
                let range = if p1.y < p2.y {
                    p1.y..=p2.y
                } else {
                    p2.y..=p1.y
                };

                for y in range {
                    covered_points.insert(Point { x: p1.x, y });
                }
            } else if p1.y == p2.y {
                let range = if p1.x < p2.x {
                    p1.x..=p2.x
                } else {
                    p2.x..=p1.x
                };

                for x in range {
                    covered_points.insert(Point { x, y: p1.y });
                }
            } else {
                panic!("{:?} {:?} don't work", p1, p2)
            }
        }
    }

    covered_points
}

fn intersections(wires: Vec<Vec<Point>>) -> Vec<Point> {
    let all_points = wires
        .iter()
        .flat_map(|wire| wire_points(wire.clone()))
        .collect::<Vec<Point>>();

    let covered_points = all_points.iter().fold(BTreeMap::new(), |mut acc, curr| {
        *acc.entry(curr).or_insert(0) += 1;
        acc
    });

    covered_points
        .iter()
        .filter_map(|(Point { x, y }, &count)| {
            if count > 1 {
                println!("{}, {} -> {}", x, y, count);
                return Some(Point { x: *x, y: *y });
            }

            None
        })
        .collect()
}

fn part_1(wires: Vec<Vec<Point>>) -> Point {
    *intersections(wires)
        .iter()
        .min_by(|p1, p2| p1.distance_from_center().cmp(&p2.distance_from_center()))
        .unwrap()
}

fn part_2(wires: Vec<Vec<Point>>) -> i32 {
    let crossed = intersections(wires.clone());
    let mut distances = BTreeMap::new();

    for intersection in crossed {
        for wire in wires.clone() {
            let mut with_center = vec![Point { x: 0, y: 0 }];
            with_center.extend(wire);

            with_center
                .windows(2)
                .scan(0, |steps, curr| match &curr {
                    [p1, p2] => {
                        if intersection.point_on_line(*p1, *p2) {
                            distances
                                .entry(intersection)
                                .or_insert_with(|| vec![])
                                .push(*steps + p1.distance_from(intersection));
                            None
                        } else {
                            Some(*steps + p1.distance_from(*p2))
                        }
                    }
                    _ => {
                        println!("{:?}", curr);

                        panic!("Shouldn't get here");
                    }
                })
                .for_each(|_| {});
        }
    }

    println!("Distances {:?}", distances);

    let (min_point, min_distances) = distances
        .iter()
        .min_by(|(_, distances1), (_, distances2)| {
            let all_distance1: i32 = distances1.iter().sum();
            let all_distance2: i32 = distances2.iter().sum();

            all_distance1.cmp(&all_distance2)
        })
        .unwrap();

    println!(
        "Min Point {:?} and min distances {:?}",
        min_point, min_distances
    );

    min_distances.iter().sum()
}

pub fn challenge() -> Result<(), std::io::Error> {
    let mut file = File::open("./data/day-3")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("\nDay 3");
    let wires: Vec<Vec<Point>> = parse_lines(&contents);
    // println!("Part 1: {:?}", part_1(wires.clone()).distance_from_center());

    println!("Part 2");
    //     assert_eq!(
    //         part_2(parse_lines(
    //             "R75,D30,R83,U83,L12,D49,R71,U7,L72
    // U62,R66,U55,R34,D71,R55,D58,R83"
    //         )),
    //         610
    //     );

    println!("{:?}", part_2(wires));

    Ok(())
}
