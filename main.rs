use std::io::{self, prelude::*};
use std::time::Instant;

extern crate clap;
use clap::value_t;

mod advent
{
    pub mod advent;
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
}

mod wires;
mod intcode;

fn getDay(x: u8) -> Box<dyn advent::advent::Solution>
{
    match x
    {
        1 => Box::new(advent::day01::Day01 {}),
        2 => Box::new(advent::day02::Day02 {}),
        3 => Box::new(advent::day03::Day03 {}),
        4 => Box::new(advent::day04::Day04 {}),
        _ => panic!("Unknow day: {}", x),
    }
}

fn main()
{
    let args = clap::App::new("Advent of Code 2019")
        .version("0.1.0")
        .author("MetroWind <chris.corsair@gmail.com>")
        .arg(clap::Arg::with_name("DAY")
             .help("The day (1--25)")
             .required(true)
             .index(1))
        .arg(clap::Arg::with_name("PART")
             .required(true)
             .possible_values(&["1", "2"])
             .help("Which part to run. (1 or 2)")
             .index(2))
        .get_matches();

    let day = clap::value_t!(args, "DAY", u8).expect("Invalid day");
    let part = clap::value_t!(args, "PART", u8).unwrap();

    let mut input_raw = String::new();
    io::stdin().lock().read_to_string(&mut input_raw).expect("Failed to read input");
    let input = input_raw.trim();

    let begin = Instant::now();
    match part
    {
        1 => println!("{}", getDay(day).part1(input)),
        2 => println!("{}", getDay(day).part2(input)),
        _ => unreachable!(),
    }
    let duration = begin.elapsed();
    println!("Run time: {}ms",
             (duration.as_secs() as f64
              + duration.subsec_nanos() as f64 * 1e-9) * 1000.0);
}
