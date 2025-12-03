use clap::{Parser, ValueEnum};

mod day1;
mod day2;
mod day3;

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
        1 => day1::day1::run(args.part),
        2 => day2::day2::run(args.part),
        3 => day3::day3::run(args.part),
        _ => panic!("Unsupported part"),
    };
}
