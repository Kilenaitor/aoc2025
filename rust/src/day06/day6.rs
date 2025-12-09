use crate::Part;
use regex::Regex;
use std::{fs, iter::Rev, str::Chars};

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
    let re = Regex::new(r"\s{2,}").unwrap();
    let mut input = fs::read_to_string("src/day06/input.txt")
        .unwrap()
        .trim()
        .split("\n")
        .map(|line| re.replace_all(line, " ").to_string())
        .collect::<Vec<String>>();

    let ops = input
        .pop()
        .unwrap()
        .trim()
        .split(" ")
        .map(|op| op.to_string())
        .collect::<Vec<String>>();
    let numbers = input
        .into_iter()
        .map(|row| {
            row.trim()
                .split(" ")
                .map(|number| number.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let mut totals: Vec<u64> = vec![];
    for i in 0..ops.len() {
        match ops[i].as_str() {
            "*" => {
                let mut total = 1;
                for j in 0..numbers.len() {
                    total *= numbers[j][i];
                }
                totals.push(total);
            }
            "+" => {
                let mut total = 0;
                for j in 0..numbers.len() {
                    total += numbers[j][i];
                }
                totals.push(total);
            }
            _ => panic!("Unexpected operation"),
        };
    }

    println!("{:?}", totals.into_iter().reduce(|t, e| t + e).unwrap());
}

fn part2() {
    let rows = fs::read_to_string("src/day06/input.txt")
        .unwrap()
        .split("\n")
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    let line_len = rows[0].len();
    let mut iters = rows
        .iter()
        .map(|line| line.chars().rev())
        .collect::<Vec<Rev<Chars>>>();

    let mut totals: Vec<u64> = vec![];
    let mut numbers: Vec<u64> = vec![];
    for _ in 0..line_len {
        let mut number = 0u64;
        for j in 0..(iters.len() - 1) {
            let val = iters[j].next().unwrap();
            if val != ' ' {
                number *= 10;
                number += val.to_digit(10).unwrap() as u64;
            }
        }
        if number != 0 {
            numbers.push(number);
        }

        let op = iters[rows.len() - 1].next().unwrap();
        if op != ' ' {
            let total = match op {
                '*' => numbers.into_iter().reduce(|acc, e| acc * e).unwrap(),
                '+' => numbers.into_iter().reduce(|acc, e| acc + e).unwrap(),
                _ => panic!("Unexpected operation!"),
            };
            totals.push(total);
            numbers = vec![];
        }
    }

    println!("{:?}", totals.into_iter().reduce(|t, e| t + e).unwrap());
}
