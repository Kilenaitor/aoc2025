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

#[derive(PartialEq)]
enum Square {
    Paper,
    Empty,
}

fn part1() {
    let input =
        fs::read_to_string("src/day4/input.txt").expect("Unable to read input. Is it downloaded?");
    let grid = input
        .trim()
        .split("\n")
        .map(|row| {
            row.chars()
                .map(|point| {
                    if point == '.' {
                        return Square::Empty;
                    }
                    Square::Paper
                })
                .collect::<Vec<Square>>()
        })
        .collect::<Vec<Vec<Square>>>();

    let mut squares_can_access = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] != Square::Paper {
                continue;
            }

            if can_access(&grid, x, y) {
                squares_can_access += 1;
            }
        }
    }

    println!("{}", squares_can_access);
}

fn part2() {
    let input =
        fs::read_to_string("src/day4/input.txt").expect("Unable to read input. Is it downloaded?");
    let mut grid = input
        .trim()
        .split("\n")
        .map(|row| {
            row.chars()
                .map(|point| {
                    if point == '.' {
                        return Square::Empty;
                    }
                    Square::Paper
                })
                .collect::<Vec<Square>>()
        })
        .collect::<Vec<Vec<Square>>>();

    let mut paper_removed = 0;
    loop {
        let paper_removed_before = paper_removed;
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] != Square::Paper {
                    continue;
                }

                if can_access(&grid, x, y) {
                    paper_removed += 1;
                    grid[y][x] = Square::Empty;
                }
            }
        }

        if paper_removed == paper_removed_before {
            break;
        }
    }

    println!("{}", paper_removed);
}

fn can_access(grid: &Vec<Vec<Square>>, x: usize, y: usize) -> bool {
    let ix = x as i32;
    let iy = y as i32;

    let coords_to_check: Vec<(usize, usize)> = Vec::from([
        (iy - 1, ix - 1),
        (iy - 1, ix),
        (iy - 1, ix + 1),
        (iy, ix - 1),
        (iy, ix + 1),
        (iy + 1, ix - 1),
        (iy + 1, ix),
        (iy + 1, ix + 1),
    ])
    .into_iter()
    .filter(|point| {
        point.0 >= 0
            && point.0 < (grid.len() as i32)
            && point.1 >= 0
            && point.1 < (grid[0].len() as i32)
    })
    .map(|point| (point.0 as usize, point.1 as usize))
    .collect();

    let mut num_paper = 0;
    for (y1, x1) in coords_to_check {
        if grid[y1][x1] == Square::Paper {
            num_paper += 1;
        }
    }

    num_paper < 4
}
