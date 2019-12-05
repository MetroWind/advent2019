use std::vec::Vec;

use crate::intcode;

pub fn part1(input: &str) -> String
{
    let mut codes: Vec<i32> = intcode::parse(&input[..]);
    codes[1] = 12;
    codes[2] = 2;

    intcode::eval(&mut codes);
    codes[0].to_string()
}

pub fn part2(input: &str) -> String
{
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
                return (100 * noun + verb).to_string();
            }
        }
    }
    String::from("No input found.")
}
