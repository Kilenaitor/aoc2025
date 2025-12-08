use crate::Part;
use std::{cmp::Ordering, collections::HashSet, fs, sync::Arc};

pub fn run(part: Part) {
    match part {
        Part::One => {
            part1();
        }
        Part::Two => {
            part2();
        }
        Part::Both => {
            part1();
            part2();
        }
    };
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
struct Distance {
    p1: Arc<Point>,
    p2: Arc<Point>,
    d: f64,
}

fn part1() {
    let points = fs::read_to_string("src/day08/input.txt")
        .unwrap()
        .trim()
        .split("\n")
        .map(|row| {
            let mut iter = row
                .trim()
                .splitn(3, ",")
                .map(|coordinate| coordinate.parse::<i64>().unwrap());

            Arc::new(Point {
                x: iter.next().unwrap(),
                y: iter.next().unwrap(),
                z: iter.next().unwrap(),
            })
        })
        .collect::<Vec<Arc<Point>>>();

    let mut distances: Vec<Distance> = vec![];
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let distance = (((points[i].x - points[j].x).pow(2)
                + (points[i].y - points[j].y).pow(2)
                + (points[i].z - points[j].z).pow(2)) as f64)
                .sqrt();
            distances.push(Distance {
                p1: points[i].clone(),
                p2: points[j].clone(),
                d: distance,
            });
        }
    }

    distances.sort_by(|d1, d2| {
        if d2.d == d1.d {
            Ordering::Equal
        } else if d2.d > d1.d {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let mut num_connections = 1000;
    let mut circuits: Vec<HashSet<Arc<Point>>> = vec![];
    for distance in &distances {
        match (
            circuits
                .iter_mut()
                .position(|circuit| circuit.contains(&distance.p1)),
            circuits
                .iter_mut()
                .position(|circuit| circuit.contains(&distance.p2)),
        ) {
            (Some(c1_pos), Some(c2_pos)) => {
                if c1_pos == c2_pos {
                    // No-op. They're already in the same circuit.
                } else {
                    let new_circuit = circuits[c1_pos]
                        .union(&circuits[c2_pos])
                        .cloned()
                        .collect::<HashSet<Arc<Point>>>();
                    circuits[c1_pos] = new_circuit;
                    circuits.swap_remove(c2_pos);
                }
            }
            (Some(c1_pos), None) => {
                circuits[c1_pos].insert(distance.p2.clone());
            }
            (None, Some(c2_pos)) => {
                circuits[c2_pos].insert(distance.p1.clone());
            }
            (None, None) => {
                let mut circuit = HashSet::new();
                circuit.insert(distance.p1.clone());
                circuit.insert(distance.p2.clone());
                circuits.push(circuit);
            }
        }

        num_connections -= 1;
        if num_connections <= 0 {
            break;
        }
    }

    let mut circuit_sizes = circuits
        .into_iter()
        .map(|circuit| circuit.len())
        .collect::<Vec<usize>>();
    circuit_sizes.sort();

    let mut total = circuit_sizes.pop().unwrap();
    total *= circuit_sizes.pop().unwrap();
    total *= circuit_sizes.pop().unwrap();

    println!("{:?}", total);
}

fn part2() {
    let points = fs::read_to_string("src/day08/input.txt")
        .unwrap()
        .trim()
        .split("\n")
        .map(|row| {
            let mut iter = row
                .trim()
                .splitn(3, ",")
                .map(|coordinate| coordinate.parse::<i64>().unwrap());

            Arc::new(Point {
                x: iter.next().unwrap(),
                y: iter.next().unwrap(),
                z: iter.next().unwrap(),
            })
        })
        .collect::<Vec<Arc<Point>>>();

    let mut distances: Vec<Distance> = vec![];
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let distance = (((points[i].x - points[j].x).pow(2)
                + (points[i].y - points[j].y).pow(2)
                + (points[i].z - points[j].z).pow(2)) as f64)
                .sqrt();
            distances.push(Distance {
                p1: points[i].clone(),
                p2: points[j].clone(),
                d: distance,
            });
        }
    }

    distances.sort_by(|d1, d2| {
        if d2.d == d1.d {
            Ordering::Equal
        } else if d2.d > d1.d {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let mut last_inserted_xes: (i64, i64) = (0, 0);
    let mut circuits: Vec<HashSet<Arc<Point>>> = vec![];
    for distance in &distances {
        last_inserted_xes = (distance.p1.x, distance.p2.x);
        match (
            circuits
                .iter_mut()
                .position(|circuit| circuit.contains(&distance.p1)),
            circuits
                .iter_mut()
                .position(|circuit| circuit.contains(&distance.p2)),
        ) {
            (Some(c1_pos), Some(c2_pos)) => {
                if c1_pos == c2_pos {
                    // No-op. They're already in the same circuit.
                } else {
                    let new_circuit = circuits[c1_pos]
                        .union(&circuits[c2_pos])
                        .cloned()
                        .collect::<HashSet<Arc<Point>>>();
                    circuits[c1_pos] = new_circuit;
                    circuits.swap_remove(c2_pos);
                }
            }
            (Some(c1_pos), None) => {
                circuits[c1_pos].insert(distance.p2.clone());
            }
            (None, Some(c2_pos)) => {
                circuits[c2_pos].insert(distance.p1.clone());
            }
            (None, None) => {
                let mut circuit = HashSet::new();
                circuit.insert(distance.p1.clone());
                circuit.insert(distance.p2.clone());
                circuits.push(circuit);
            }
        }

        if circuits.len() == 1 {
            if circuits[0].len() == points.len() {
                println!("{}", distance.p1.x * distance.p2.x);
                return;
            }
        }
    }

    if circuits.len() == 1 {
        if circuits[0].len() == points.len() {
            println!("{:?}", last_inserted_xes.0 * last_inserted_xes.1);
            return;
        }
    }

    println!("Never connected the full circuit");
}
