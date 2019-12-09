use crate::intcode::intcode;

pub fn part1(input: &str) -> intcode::ValueType
{
    let mut computer = intcode::IntCodeComputer::new();
    computer.loadCode(&intcode::parse(input));
    computer.eval(Some(&vec![1]));
    assert_eq!(computer.output.len(), 1);
    computer.output[0]
}

pub fn part2(input: &str) -> intcode::ValueType
{
    let mut computer = intcode::IntCodeComputer::new();
    computer.loadCode(&intcode::parse(input));
    computer.eval(Some(&vec![2]));
    assert_eq!(computer.output.len(), 1);
    computer.output[0]
}
