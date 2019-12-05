#![allow(non_snake_case)]

use std::vec::Vec;

pub fn parse(code: &str) -> Vec<i32>
{
    code.split(',').map(
        |part|
        {
            let op = part.trim().parse::<i32>();
            if op.is_err()
            {
                panic!("Cannot convert '{}'.", part);
            }
            op.unwrap()
        }).collect()
}

enum ArgMode
{
    Position,
    Immediate,
}

impl ArgMode
{
    pub fn fromDigit(x: u8) -> Result<ArgMode, ()>
    {
        match x
        {
            0 => Ok(ArgMode::Position),
            1 => Ok(ArgMode::Immediate),
            _ => Err(()),
        }
    }
}

struct OpCode
{
    arg_count: u8,
    code: u8,
    arg_modes: Vec<ArgMode>,
}

impl OpCode
{
    fn fromInt(code: i32) -> Result<OpCode, ()>
    {
        if code < 0
        {
            return Err(());
        }

        let code2: u8 =  (code % 100) as u8;
        let mut code_modes = code / 100;
        let arg_count: u8 = match code2
        {
            1 | 2 => 3,
            3 | 4 => 1,
            5 | 6 => 2,
            7 | 8 => 3,
            99 => 0,
            _ => { return Err(()); },
        };

        let modes: Vec<ArgMode> = (0..arg_count).map(
            |_|
            {
                let mode = ArgMode::fromDigit((code_modes % 10) as u8)
                    .expect(&format!("Invalid arg mode in code {}", code)[..]);
                code_modes /= 10;
                mode
            }).collect();
        Ok(OpCode
           {
               arg_count: arg_count,
               code: code2,
               arg_modes: modes,
           })
    }
}

pub struct IntCodeComputer
{
    pub mem: Vec<i32>,
    cursor: usize,
    halt: bool,
    input: Vec<i32>,
    cursor_input: usize,
    pub output: Vec<i32>,
}

impl IntCodeComputer
{
    pub fn new() -> IntCodeComputer
    {
        IntCodeComputer
        {
            mem: vec![],
            cursor: 0,
            halt: false,
            input: vec![],
            cursor_input: 0,
            output: vec![],
        }
    }

    pub fn eval(&mut self, codes: &Vec<i32>, input: Option<&Vec<i32>>)
    {
        self.mem = codes.clone();
        if input.is_some()
        {
            self.input = input.unwrap().clone();
        }

        loop
        {
            let code: OpCode = OpCode::fromInt(self.mem[self.cursor])
                .expect(&format!("Invalid opcode {}", self.mem[self.cursor])[..]);
            match code.code
            {
                99 => { self.evalHalt(&code); },
                1 => { self.evalAdd(&code); },
                2 => { self.evalMult(&code); },
                3 => { self.evalInput(&code); },
                4 => { self.evalOutput(&code); },
                5 => { self.evalJmpTrue(&code); },
                6 => { self.evalJmpFalse(&code); },
                7 => { self.evalLess(&code); },
                8 => { self.evalEqual(&code); },
                _ => unreachable!(),
            };

            if self.halt
            {
                break;
            }
        }
    }

    fn getArg(&self, code: &OpCode, index: u8) -> i32
    {
        match code.arg_modes[index as usize]
        {
            ArgMode::Position =>
            {
                let addr = self.mem[self.cursor + (index as usize) + 1] as usize;
                self.mem[addr]
            },
            ArgMode::Immediate => self.mem[self.cursor + (index as usize) + 1],
        }
    }

    fn skip(&mut self, code: &OpCode)
    {
        self.cursor += (code.arg_count as usize) + 1;
    }

    fn evalHalt(&mut self, _: &OpCode)
    {
        self.halt = true;
    }

    fn evalAdd(&mut self, code: &OpCode)
    {
        let lhs = match code.arg_modes[0]
        {
            ArgMode::Position =>
            {
                let lhs_addr = self.mem[self.cursor+1] as usize;
                self.mem[lhs_addr]
            },
            ArgMode::Immediate => self.mem[self.cursor+1],
        };

        let rhs = match code.arg_modes[1]
        {
            ArgMode::Position =>
            {
                let rhs_addr = self.mem[self.cursor+2] as usize;
                self.mem[rhs_addr]
            }
            ArgMode::Immediate => self.mem[self.cursor + 2],
        };

        let result_addr = self.mem[self.cursor+3] as usize;
        self.mem[result_addr] = lhs + rhs;
        self.skip(code);
    }

    fn evalMult(&mut self, code: &OpCode)
    {
        let lhs = match code.arg_modes[0]
        {
            ArgMode::Position =>
            {
                let lhs_addr = self.mem[self.cursor+1] as usize;
                self.mem[lhs_addr]
            },
            ArgMode::Immediate => self.mem[self.cursor+1],
        };

        let rhs = match code.arg_modes[1]
        {
            ArgMode::Position =>
            {
                let rhs_addr = self.mem[self.cursor+2] as usize;
                self.mem[rhs_addr]
            }
            ArgMode::Immediate => self.mem[self.cursor + 2],
        };

        let result_addr = self.mem[self.cursor+3] as usize;
        self.mem[result_addr] = lhs * rhs;
        self.skip(code);
    }

    fn evalInput(&mut self, code: &OpCode)
    {
        let result_addr = self.mem[self.cursor+1] as usize;
        self.mem[result_addr] = self.input[self.cursor_input];
        self.cursor_input += 1;
        self.skip(code);
    }

    fn evalOutput(&mut self, code: &OpCode)
    {
        match code.arg_modes[0]
        {
            ArgMode::Position =>
            {
                let addr = self.mem[self.cursor + 1] as usize;
                self.output.push(self.mem[addr]);
            },
            ArgMode::Immediate =>
            {
                self.output.push(self.mem[self.cursor + 1]);
            },
        };
        self.skip(code);
    }

    fn evalJmpTrue(&mut self, code: &OpCode)
    {
        let arg = self.getArg(code, 0);

        if arg != 0
        {
            self.cursor = self.getArg(code, 1) as usize;
            println!("True jumped to {}.", self.cursor);
        }
        else
        {
            self.skip(code);
        }
    }

    fn evalJmpFalse(&mut self, code: &OpCode)
    {
        let arg = self.getArg(code, 0);

        if arg == 0
        {
            self.cursor = self.getArg(code, 1) as usize;
            println!("False jumped to {}.", self.cursor);
        }
        else
        {
            self.skip(code);
        }
    }

    fn evalLess(&mut self, code: &OpCode)
    {
        let lhs = self.getArg(code, 0);
        let rhs = self.getArg(code, 1);
        let result_addr = self.mem[self.cursor + 3] as usize;

        self.mem[result_addr] = if lhs < rhs {1} else {0};
        self.skip(code);
    }

    fn evalEqual(&mut self, code: &OpCode)
    {
        let lhs = self.getArg(code, 0);
        let rhs = self.getArg(code, 1);
        let result_addr = self.mem[self.cursor + 3] as usize;

        self.mem[result_addr] = if lhs == rhs {1} else {0};
        self.skip(code);
    }
}
