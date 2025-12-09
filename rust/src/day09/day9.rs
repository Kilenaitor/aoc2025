use crate::Part;
use std::fs;

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

fn part1() {
    let points = fs::read_to_string("src/day09/input.txt")
        .unwrap()
        .trim()
        .split("\n")
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
        })
        .collect::<Vec<(i64, i64)>>();

    let mut max_area: i64 = 0;
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let area =
                ((points[i].0 - points[j].0).abs() + 1) * ((points[i].1 - points[j].1).abs() + 1);
            if area > max_area {
                max_area = area;
            }
        }
    }

    println!("{}", max_area)
}

fn part2() {
    let input = fs::read_to_string("src/day09/input.txt")
        .unwrap()
        .trim()
        .split_once("\n\n")
        .map(|i| (i.0.to_string(), i.1.to_string()))
        .unwrap();

    println!("part 2");
}
