use crate::Part;
use std::{fs, str::Chars};

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
    let rows = fs::read_to_string("src/day07/input.txt")
        .unwrap()
        .split("\n")
        .map(|row| row.to_string().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut num_splits = 0;
    let mut beams = vec![0u8; rows.len()-1];
    for i in 0..rows.len() {
        for j in 0..rows[i].len() {
            if rows[i][j] == 'S' {
                beams[j] = 1;
                continue;
            }
            if rows[i][j] == '^' {
                if beams[j] == 1 {
                    num_splits += 1;
                    beams[j - 1] = 1;
                    beams[j] = 0;
                    beams[j + 1] = 1;
                }
            }
        }
        println!("{}", String::from_iter(rows[i].iter()));
        println!("{}", beams.iter().map(|i| i.to_string()).collect::<String>());
    }
    println!("{}", num_splits);
}

fn part2() {
    let rows = fs::read_to_string("src/day07/test.txt")
        .unwrap()
        .split("\n")
        .map(|row| row.to_string().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    println!("part 2");
}
