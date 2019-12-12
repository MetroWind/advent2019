#![allow(non_snake_case)]

use std::io::{self, prelude::*};
use std::time::Instant;
use std::env;
use std::process::exit;

mod wires;
mod vector;
mod intcode
{
    pub mod intcode;
}

mod ratio;

extern crate meta;

meta::importAdvent!();
meta::defineRun!();

fn usage(name: &String)
{
    println!("Usage: {name} DAY PART

Example: {name} 4 1  # Run day 4 part 1.", name=name)
}

fn main()
{
    let args: Vec<String> = env::args().collect();
    if args.len() != 3
    {
        usage(&args[0]);
        exit(1);
    }

    let day: u8 = args[1].parse().unwrap_or_else(|_| {usage(&args[0]); 0});
    let part: u8 = args[2].parse().unwrap_or_else(|_| {usage(&args[0]); 0});

    let mut input_raw = String::new();
    io::stdin().lock().read_to_string(&mut input_raw).expect("Failed to read input");
    let input = input_raw.trim();

    let begin = Instant::now();
    let output = run(day, part, input);
    let duration = begin.elapsed();

    println!("{}", output);
    println!("Run time: {}ms",
             (duration.as_secs() as f64
              + duration.subsec_nanos() as f64 * 1e-9) * 1000.0);
}
