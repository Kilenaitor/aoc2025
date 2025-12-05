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
    let input =
        fs::read_to_string("src/day3/input.txt").expect("Unable to read input. Is it downloaded?");
    let banks = input
        .trim()
        .split("\n")
        .map(|bank| {
            bank.chars()
                .map(|battery| battery.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect::<Vec<Vec<u8>>>();

    let mut joltage = 0u64;
    for bank in banks {
        let mut highest_value = 0u64;

        for tens in 0..(bank.len() - 1) {
            for ones in (tens + 1)..bank.len() {
                let value: u64 = (bank[tens] * 10 + bank[ones]).into();
                if value > highest_value {
                    highest_value = value;
                }
            }
        }

        joltage += highest_value;
    }

    println!("{}", joltage);
}
fn part2() {
    let input =
        fs::read_to_string("src/day3/input.txt").expect("Unable to read input. Is it downloaded?");
    let banks = input
        .trim()
        .split("\n")
        .map(|bank| {
            bank.chars()
                .map(|battery| battery.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect::<Vec<Vec<usize>>>();

    let mut joltage = 0usize;
    for bank in banks {
        let mut bank_joltage = 0usize;
        let mut digits_remaining = 12;
        let mut start = 0usize;
        let mut end = bank.len() - digits_remaining;

        while digits_remaining > 0 {
            let index = first_biggest_in_range(&bank, start, end);
            bank_joltage *= 10;
            bank_joltage += bank[index];
            start = index + 1;
            end += 1;
            digits_remaining -= 1;
        }

        joltage += bank_joltage;
    }

    println!("{}", joltage);
}

fn first_biggest_in_range(numbers: &Vec<usize>, start: usize, end: usize) -> usize {
    if numbers[start] == 9 {
        return start;
    }

    let mut largest_index = start;
    for i in start..=end {
        if numbers[i] == 9 {
            return i;
        }

        if numbers[i] > numbers[largest_index] {
            largest_index = i;
        }
    }

    largest_index
}
