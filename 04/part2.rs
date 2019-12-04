use std::vec::Vec;

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
