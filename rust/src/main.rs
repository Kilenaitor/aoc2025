use clap::{Parser, ValueEnum};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Part {
    One,
    Two,
    Both,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    day: u8,
    #[arg(value_enum, default_value_t = Part::Both)]
    part: Part,
}

fn main() {
    let args = Args::parse();

    match args.day {
        1 => day01::day1::run(args.part),
        2 => day02::day2::run(args.part),
        3 => day03::day3::run(args.part),
        4 => day04::day4::run(args.part),
        5 => day05::day5::run(args.part),
        6 => day06::day6::run(args.part),
        7 => day07::day7::run(args.part),
        8 => day08::day8::run(args.part),
        9 => day09::day9::run(args.part),
        10 => day10::day10::run(args.part),
        11 => day11::day11::run(args.part),
        12 => day12::day12::run(args.part),
        _ => panic!("Unsupported part"),
    };
}
