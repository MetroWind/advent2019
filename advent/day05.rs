use crate::intcode::intcode;

pub fn part1(input: &str) -> String
{
    let code = intcode::parse(input);
    let mut computer = intcode::IntCodeComputer::new();
    let input: Vec<intcode::ValueType> = vec![1];
    computer.loadCode(&code);
    computer.eval(Some(&input));
    format!("{:?}", computer.output)
}

pub fn part2(input: &str) -> String
{
    let code = intcode::parse(input);
    let mut computer = intcode::IntCodeComputer::new();
    let input: Vec<intcode::ValueType> = vec![5];
    computer.loadCode(&code);
    computer.eval(Some(&input));
    format!("{:?}", computer.output)
}
