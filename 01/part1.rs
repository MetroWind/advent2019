use std::io::{self, prelude::*};

fn fuel(mass: i32) -> i32
{
    return mass / 3 - 2;
}

fn main()
{
    // for line in io::stdin().lock().lines()
    let result: i32 = io::stdin().lock().lines().map(
        |line|
        {
            if line.is_err()
            {
                panic!("Failed to read line.");
            }

            let the_line = line.unwrap();

            let input = i32::from_str_radix(&the_line[..], 10);
            if input.is_err()
            {
                panic!("Cannot convert '{}'.", the_line);
            }
            fuel(input.unwrap())
        }).sum();
    println!("{}", result);
}
