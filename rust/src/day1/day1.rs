use crate::Part;
use regex::Regex;
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
    let rotations = fs::read_to_string("src/day1/input.txt")
        .expect("Unable to read part1 input. Is it downloaded?");
    let re = Regex::new(r"([L,R])(\d+)").unwrap();

    let mut dial: i32 = 50;
    let mut num_zeroes = 0;
    for rotation in rotations.lines() {
        let (_, [direction, amount_str]) = re.captures(rotation).unwrap().extract();
        let amount = amount_str.parse::<i32>().unwrap();
        dial = if direction == "L" {
            dial - amount
        } else {
            dial + amount
        };
        dial = dial.rem_euclid(100);
        if dial == 0 {
            num_zeroes += 1;
        }
    }
    println!("Num zeroes: {:?}", num_zeroes);
}

fn part2() {
    let rotations = fs::read_to_string("src/day1/input.txt")
        .expect("Unable to read part1 input. Is it downloaded?");
    let re = Regex::new(r"([L,R])(\d+)").unwrap();

    let mut dial: i32 = 50;
    let mut num_zeroes = 0;
    for rotation in rotations.lines() {
        let (_, [direction, amount_str]) = re.captures(rotation).unwrap().extract();
        let dial_before = dial;
        let mut amount = amount_str.parse::<i32>().unwrap();

        // Full turns
        num_zeroes += amount / 100;
        amount %= 100;

        if amount == 0 {
            continue;
        }

        dial = if direction == "L" {
            dial - amount
        } else {
            dial + amount
        };

        if dial >= 100 || (dial_before > 0 && dial <= 0) {
            num_zeroes += 1;
        }

        dial = dial.rem_euclid(100);
    }
    println!("Num zeroes: {:?}", num_zeroes);
}
