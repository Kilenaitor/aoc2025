use crate::Part;
use num::integer::sqrt;
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
    let ranges = fs::read_to_string("src/day2/input.txt")
        .expect("Unable to read input. Is it downloaded?")
        .trim()
        .split(",")
        .map(|range| {
            let mut numbers = range.split("-");
            (
                numbers.next().unwrap().to_string(),
                numbers.next().unwrap().to_string(),
            )
        })
        .collect::<Vec<(String, String)>>();

    let mut invalid_total = 0;
    for range in ranges.into_iter() {
        let (low, high) = range;

        // Can't have a sequence repeated twice if odd number
        if low.len() % 2 != 0 {
            // Can't have a sequence repeated twice if odd number
            if high.len() % 2 != 0 {
                // They also have same number of digits, so it's
                // impossible to have an even number of digits
                if high.len() - low.len() == 0 {
                    continue;
                }
            }
        }

        for num in to_u64(low)..=to_u64(high) {
            let num_digits = num.ilog10() + 1;
            if num_digits % 2 != 0 {
                continue;
            }
            let divisor = 10u64.pow(num_digits / 2);
            let first_half = num / divisor;
            let second_half = num % divisor;
            if first_half == second_half {
                invalid_total += num;
            }
        }
    }

    println!("Invalid total: {:?}", invalid_total);
}

fn to_u64(str: String) -> u64 {
    str.parse::<u64>().unwrap()
}

fn part2() {
    let ranges = fs::read_to_string("src/day2/input.txt")
        .expect("Unable to read input. Is it downloaded?")
        .trim()
        .split(",")
        .map(|range| {
            let mut numbers = range.split("-");
            (
                numbers.next().unwrap().to_string(),
                numbers.next().unwrap().to_string(),
            )
        })
        .collect::<Vec<(String, String)>>();

    let mut invalid_total = 0;
    for range in ranges.into_iter() {
        let (low, high) = range;

        for num in to_u64(low)..=to_u64(high) {
            if is_invalid(num) {
                println!("{}", num);
                invalid_total += num;
            }
        }
    }

    println!("Invalid total: {:?}", invalid_total);
}

fn is_invalid(num: u64) -> bool {
    let num_digits = num.ilog10() + 1;
    if num_digits == 1 {
        // Can't have a pattern of 1
        return false;
    }

    let mut largest_group_size = 1;
    for i in 2..=sqrt(num_digits) {
        if num_digits % i == 0 {
            // The first digit we get that's divisible
            // is going to be the smallest non-1 factor,
            // which means the opposite of it will be our
            // largest group size. We can stop here.
            largest_group_size = num_digits / i;
            break;
        }
    }

    let mut group_size = largest_group_size;
    while group_size > 1 {
        let mask = 10u64.pow(group_size);
        let mut remaining = num;
        let group_val = remaining % mask;
        remaining /= mask;

        let mut is_match = true;
        while is_match && remaining > 0 {
            let comparison = remaining % mask;
            is_match = comparison == group_val;
            remaining /= mask;
        }

        if is_match {
            return true;
        }

        group_size -= 1;
    }

    let mut remaining = num;
    let digit = remaining % 10;
    remaining /= 10;
    while remaining > 0 {
        let compare = remaining % 10;
        if compare != digit {
            return false;
        }
        remaining /= 10;
    }

    true
}
