use std::io::{self, prelude::*};

fn fuel_inner(mass: i32) -> i32
{
    let result = mass / 3 - 2;
    if result < 0
    {
        0
    }
    else
    {
        result
    }
}

fn fuel(mass: i32) -> i32
{
    let addon = fuel_inner(mass);
    if addon == 0
    {
        0
    }
    else
    {
        addon + fuel(addon)
    }
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
