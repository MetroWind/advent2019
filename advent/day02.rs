use std::vec::Vec;

use crate::intcode::intcode;

pub fn part1(input: &str) -> i32
{
    let mut computer = intcode::IntCodeComputer::new();
    let mut codes: Vec<i32> = intcode::parse(&input[..]);
    codes[1] = 12;
    codes[2] = 2;

    computer.eval(&codes, None);
    computer.mem[0]
}

pub fn part2(input: &str) -> String
{
    let codes: Vec<i32> = intcode::parse(&input[..]);

    for noun in 0..100
    {
        for verb in 0..100
        {
            let mut computer = intcode::IntCodeComputer::new();
            let mut mem = codes.clone();
            mem[1] = noun;
            mem[2] = verb;
            computer.eval(&mem, None);

            if computer.mem[0] == 19690720
            {
                return (100 * noun + verb).to_string();
            }
        }
    }
    String::from("No input found.")
}
