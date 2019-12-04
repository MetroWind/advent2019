use std::vec::Vec;

fn isValid(x: u32, min: u32, max: u32) -> bool
{
    if !(min <= x && x <= max)
    {
        return false;
    }

    let ss = x.to_string();
    let mut s: Vec<u8> = ss.bytes().collect();

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

fn main()
{
    let mut count = 0;
    for x in 124075..580769+1
    {
        if isValid(x, 124075, 580769)
        {
            count += 1;
        }
    }
    println!("{}", count);
}
