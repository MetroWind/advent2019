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

    let codes: Vec<i32> = intcode::parse(&input[..]);

    for noun in 0..100
    {
        for verb in 0..100
        {
            let mut mem = codes.clone();
            mem[1] = noun;
            mem[2] = verb;
            intcode::eval(&mut mem);

            if mem[0] == 19690720
            {
                println!("{}", 100 * noun + verb);
                return;
            }
        }
    }
}
