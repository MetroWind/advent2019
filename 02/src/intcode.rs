#![allow(non_snake_case)]

use std::vec::Vec;

fn evalAdd(codes: &mut Vec<i32>, cursor: usize)
{
    let lhs_addr = codes[cursor+1] as usize;
    let rhs_addr = codes[cursor+2] as usize;
    let result_addr = codes[cursor+3] as usize;
    codes[result_addr] = codes[lhs_addr] + codes[rhs_addr];
}

fn evalMult(codes: &mut Vec<i32>, cursor: usize)
{
    let lhs_addr = codes[cursor+1] as usize;
    let rhs_addr = codes[cursor+2] as usize;
    let result_addr = codes[cursor+3] as usize;
    codes[result_addr] = codes[lhs_addr] * codes[rhs_addr];
}

pub fn parse(code: &str) -> Vec<i32>
{
    code.trim().split(',').map(
        |part|
        {
            let op = part.parse::<i32>();
            if op.is_err()
            {
                panic!("Cannot convert '{}'.", part);
            }
            op.unwrap()
        }).collect()
}

pub fn eval(codes: &mut Vec<i32>)
{
    let mut cursor: usize = 0;
    let mut halt = false;

    loop
    {
        let code: i32 = codes[cursor];
        match code
        {
            99 => { halt = true; },
            1 => { evalAdd(codes, cursor); },
            2 => { evalMult(codes, cursor); },
            _ => { panic!("Invalid opcode at {}: {}.", cursor, code); },
        };

        if halt
        {
            break;
        }
        else
        {
            cursor += 4;
        }
    }
}
