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
    let input = fs::read_to_string("src/day5/input.txt")
        .unwrap()
        .trim()
        .split_once("\n\n")
        .map(|i| (i.0.to_string(), i.1.to_string()))
        .unwrap();
    let ranges = input
        .0
        .split("\n")
        .map(|range| {
            range
                .trim()
                .split_once("-")
                .map(|num| (num.0.parse::<u64>().unwrap(), num.1.parse::<u64>().unwrap()))
        })
        .map(|range| range.unwrap())
        .collect::<Vec<(u64, u64)>>();
    let ingredient_ids = input
        .1
        .split("\n")
        .map(|ingredient| ingredient.trim().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut num_fresh = 0;
    for ingredient_id in ingredient_ids {
        for (low, high) in &ranges {
            if ingredient_id >= *low && ingredient_id <= *high {
                num_fresh += 1;
                break;
            }
        }
    }
    println!("{:?}", num_fresh);
}

fn part2() {
    let mut input = fs::read_to_string("src/day5/input.txt")
        .unwrap()
        .trim()
        .split_once("\n\n")
        .unwrap()
        .0
        .split("\n")
        .map(|range| {
            range
                .trim()
                .split_once("-")
                .map(|num| (num.0.parse::<u64>().unwrap(), num.1.parse::<u64>().unwrap()))
        })
        .map(|range| range.unwrap())
        .collect::<Vec<(u64, u64)>>();

    // Sorting the ranges by their opening value
    // so we don't need to scan through the collapsed
    // list.
    input.sort_by(|a, b| a.0.cmp(&b.0));

    let mut ranges: Vec<(u64, u64)> = vec![];

    for i in 0..input.len() {
        if ranges.len() == 0 {
            ranges.push(input[i]);
            continue;
        }

        let candidate_range = input[i];
        let highest_range_index = ranges.len() - 1;
        let highest_range = &mut ranges[highest_range_index];
        if candidate_range.0 >= highest_range.0 && candidate_range.0 <= highest_range.1 {
            // We have an overlap!
            highest_range.1 = if candidate_range.1 > highest_range.1 {
                candidate_range.1
            } else {
                highest_range.1
            };
        } else {
            ranges.push(candidate_range);
        }
    }

    let mut distinct_ids = 0;
    for (low, high) in ranges {
        distinct_ids += high - low + 1;
    }

    println!("{}", distinct_ids);
}
