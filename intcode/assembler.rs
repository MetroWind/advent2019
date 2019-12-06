use std::io::{self, prelude::*};

mod assembly;

fn main() -> Result<(), String>
{
    let mut input_raw = String::new();
    io::stdin().lock().read_to_string(&mut input_raw)
        .map_err(|_| String::from("Failed to read input"))?;
    let input = input_raw.trim();

    for s in assembly::parse(&input[..])?
    {
        println!("{:?}", s);
    }

    Ok(())
}
