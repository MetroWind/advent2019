#![allow(non_snake_case)]

use std::io::{self, prelude::*};
use std::vec::Vec;
use std::env;
use std::process::exit;
use std::fs;

mod intcode;

fn usage(name: &String)
{
    println!("Usage: {name} FILE

Run load intcode from FILE and run it. Take input from stdin (one
number per line) and output to stdout. If there's no input, you need
to press ctrl+d.", name=name);
}

fn main() -> Result<(), String>
{
    let args: Vec<String> = env::args().collect();
    if args.len() != 2
    {
        usage(&args[0]);
        exit(1);
    }

    if args[1] == "-h" || args[1] == "--help"
    {
        usage(&args[0]);
        return Ok(());
    }

    let source_raw = fs::read_to_string(&args[1])
        .map_err(|_| format!("Failed to read file {}.", args[1]))?;
    let source = source_raw.trim();

    let mut input_raw = String::new();
    io::stdin().lock().read_to_string(&mut input_raw).expect("Failed to read input");
    let mut computer = intcode::IntCodeComputer::new();

    computer.input = input_raw.trim().lines().map(
        |line|
        {
            if let Ok(x) = line.parse::<i32>()
            {
                x
            }
            else
            {
                panic!("Invalid input: {}", line);
            }
        }).collect();

    let codes: Vec<i32> = intcode::parse(&source[..]);

    computer.eval(&codes, None);

    for x in computer.output
    {
        println!("{}", x);
    }

    Ok(())
}
