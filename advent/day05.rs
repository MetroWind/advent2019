use crate::intcode;

pub fn part1(input: &str) -> String
{
    let code = intcode::parse(input);
    let mut computer = intcode::IntCodeComputer::new();
    let input: Vec<i32> = vec![1];
    computer.eval(&code, Some(&input));
    format!("{:?}", computer.output)
}

pub fn part2(input: &str) -> String
{
    let code = intcode::parse(input);
    let mut computer = intcode::IntCodeComputer::new();
    let input: Vec<i32> = vec![5];
    computer.eval(&code, Some(&input));
    format!("{:?}", computer.output)
}
