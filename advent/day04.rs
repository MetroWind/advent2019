use std::vec::Vec;

fn parseInput(input: &str) -> (u32, u32)
{
    let mut iter = input.split("-");
    let mut result: (u32, u32) = (0, 0);
    result.0 = iter.next().expect("Invalid input")
        .parse().expect("Failed to convert min.");
    result.1 = iter.next().expect("Invalid input")
        .parse().expect("Failed to convert max.");
    result
}

pub fn part1(input: &str) -> u32
{
    fn isValid(x: u32, min: u32, max: u32) -> bool
    {
        if !(min <= x && x <= max)
        {
            return false;
        }

        let ss = x.to_string();
        let s: Vec<u8> = ss.bytes().collect();

        if s.len() != 6
        {
            return false;
        }

        let mut repeated = false;
        for i in 1..6
        {
            if s[i] < s[i-1]
            {
                return false;
            }

            if s[i] == s[i-1]
            {
                repeated = true;
            }
        }

        repeated
    }

    let mut count: u32 = 0;
    let range = parseInput(input);
    for x in range.0..range.1+1
    {
        if isValid(x, range.0, range.1)
        {
            count += 1;
        }
    }
    count
}

pub fn part2(input: &str) -> u32
{
    fn isValid(x: u32, min: u32, max: u32) -> bool
    {
        if !(min <= x && x <= max)
        {
            return false;
        }

        let ss = x.to_string();
        let s: Vec<u8> = ss.bytes().collect();

        if s.len() != 6
        {
            return false;
        }

        let mut repeated = s[0];
        let mut repeat_count = 1;
        let mut two_repeat = false;
        for i in 1..6
        {
            if s[i] < s[i-1]
            {
                return false;
            }

            if s[i] == repeated
            {
                repeat_count += 1;
            }
            else
            {
                if repeat_count == 2
                {
                    two_repeat = true;
                }

                repeat_count = 1;
                repeated = s[i];
            }
        }

        if repeat_count == 2
        {
            two_repeat = true;
        }
        two_repeat
    }

    let mut count: u32 = 0;
    let range = parseInput(input);
    for x in range.0..range.1+1
    {
        if isValid(x, range.0, range.1)
        {
            count += 1;
        }
    }
    count
}

#[test]
fn testPart1()
{
    assert_eq!(part1("111111-111111"), 1);
    assert_eq!(part1("223450-223450"), 0);
    assert_eq!(part1("123789-123789"), 0);
}

#[test]
fn testPart2()
{
    assert_eq!(part2("112233-112233"), 1);
    assert_eq!(part2("123444-123444"), 0);
    assert_eq!(part2("111122-111122"), 1);
}
