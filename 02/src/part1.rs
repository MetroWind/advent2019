use std::io::{self, prelude::*};
use std::vec::Vec;

mod intcode;

fn main()
{
    let mut input = String::new();
    if io::stdin().lock().read_to_string(&mut input).is_err()
    {
        panic!("Failed to read input.");
    }

    let mut codes: Vec<i32> = intcode::parse(&input[..]);
    codes[1] = 12;
    codes[2] = 2;

    intcode::eval(&mut codes);
    println!("{}", codes[0]);
}
