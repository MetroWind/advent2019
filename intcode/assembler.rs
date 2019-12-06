#![allow(non_snake_case)]

use std::io::{self, prelude::*};
use std::vec::Vec;

mod assembly;

fn main() -> Result<(), String>
{
    let mut input_raw = String::new();
    io::stdin().lock().read_to_string(&mut input_raw)
        .map_err(|_| String::from("Failed to read input"))?;
    let input = input_raw.trim();

    let statements = assembly::parse(&input[..])?;
    let code = assembly::assemble(&statements)?;

    let output: String = code.iter().map(|x| x.to_string())
        .collect::<Vec<String>>().join(",");
    println!("{}", output);
    Ok(())
}
