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
    pub input: Vec<i32>,
    cursor_input: usize,
    pub output: Vec<i32>,

    one_input: i32,
    one_output: i32,
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

            one_input: 0,
            one_output: 0,
        }
    }

    #[allow(dead_code)]
    pub fn reset(&mut self)
    {
        self.mem.clear();
        self.cursor = 0;
        self.halt = false;
        self.input.clear();
        self.cursor_input = 0;
        self.output.clear();

        self.one_input = 0;
        self.one_output = 0;
    }

    fn getNextOpCode(&mut self) -> OpCode
    {
        OpCode::fromInt(self.mem[self.cursor])
            .expect(&format!("Invalid opcode {}", self.mem[self.cursor])[..])
    }

    fn step(&mut self, code: &OpCode)
    {
        match code.code
        {
            99 => { self.evalHalt(code); },
            1 => { self.evalAdd(code); },
            2 => { self.evalMult(code); },
            3 => { self.evalInput(code); },
            4 => { self.evalOutput(code); },
            5 => { self.evalJmpTrue(code); },
            6 => { self.evalJmpFalse(code); },
            7 => { self.evalLess(code); },
            8 => { self.evalEqual(code); },
            _ => unreachable!(),
        };
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
            let code: OpCode = self.getNextOpCode();

            if code.code == 3    // input
            {
                self.one_input = self.input[self.cursor_input];
                self.cursor_input += 1;
            }

            self.step(&code);

            if code.code == 4    // Output
            {
                self.output.push(self.one_output);
            }

            if self.halt
            {
                break;
            }

            // println!("{:?}", self.mem);
        }
    }

    // As soon as the computer hits the next input, do the input and
    // pause there. Doesn’t write to `self.input’. Also return when
    // halted.
    #[allow(dead_code)]
    pub fn consumeSingleInput(&mut self, input: i32)
    {
        self.one_input = input;
        loop
        {
            let code: OpCode = self.getNextOpCode();
            self.step(&code);

            if code.code == 3    // Input
            {
                return;
            }

            if self.halt
            {
                return;
            }
        }
    }

    // Work in pipe mode. Ever time this function calls, it takes 1
    // input, runs the computer until there’s an output, and returns
    // that output. The computer then pause at the current state. This
    // function can be then called to process further inputs. If the
    // computer halts in the process, returns None.
    //
    // The code is directly taken from memory.
    #[allow(dead_code)]
    pub fn pipe(&mut self, input: i32) -> Option<i32>
    {
        self.one_input = input;

        loop
        {
            let code: OpCode = self.getNextOpCode();

            self.step(&code);
            if code.code == 4    // Output
            {
                return Some(self.one_output);
            }

            if self.halt
            {
                return None;
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
        let lhs = self.getArg(code, 0);
        let rhs = self.getArg(code, 1);

        let result_addr = self.mem[self.cursor+3] as usize;
        self.mem[result_addr] = lhs + rhs;
        self.skip(code);
    }

    fn evalMult(&mut self, code: &OpCode)
    {
        let lhs = self.getArg(code, 0);
        let rhs = self.getArg(code, 1);

        let result_addr = self.mem[self.cursor+3] as usize;
        self.mem[result_addr] = lhs * rhs;
        self.skip(code);
    }

    fn evalInput(&mut self, code: &OpCode)
    {
        let result_addr = self.mem[self.cursor+1] as usize;
        self.mem[result_addr] = self.one_input;
        self.skip(code);
    }

    fn evalOutput(&mut self, code: &OpCode)
    {
        self.one_output = self.getArg(code, 0);
        self.skip(code);
    }

    fn evalJmpTrue(&mut self, code: &OpCode)
    {
        let arg = self.getArg(code, 0);

        if arg != 0
        {
            self.cursor = self.getArg(code, 1) as usize;
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

// ========== Tests =================================================>

#[test]
fn testAdd()
{
    let mut computer = IntCodeComputer::new();

    computer.eval(&vec![1,0,0,0,99], None);
    assert_eq!(computer.mem, vec![2,0,0,0,99]);
}

#[test]
fn testMult()
{
    let mut computer = IntCodeComputer::new();

    computer.eval(&vec![2,3,0,3,99], None);
    assert_eq!(computer.mem, vec![2,3,0,6,99]);

    computer.reset();
    computer.eval(&vec![2,4,4,5,99,0], None);
    assert_eq!(computer.mem, vec![2,4,4,5,99,9801]);
}

#[test]
fn testBasic()
{
    let mut computer = IntCodeComputer::new();
    computer.eval(&vec![1,1,1,4,99,5,6,0,99], None);
    assert_eq!(computer.mem, vec![30,1,1,4,2,5,6,0,99]);
}

#[test]
fn testIO()
{
    let mut computer = IntCodeComputer::new();
    computer.eval(&vec![3,0,99], Some(&vec![10]));
    assert_eq!(computer.mem, vec![10,0,99]);

    computer.reset();
    computer.eval(&vec![3,0,3,1,99], Some(&vec![10,20]));
    assert_eq!(computer.mem, vec![10,20,3,1,99]);

    computer.reset();
    computer.eval(&vec![4,0,99], None);
    assert_eq!(computer.output, vec![4]);

    computer.reset();
    computer.eval(&vec![4,0,4,4,99], None);
    assert_eq!(computer.output, vec![4,99]);
}

#[test]
fn testModes()
{
    let mut computer = IntCodeComputer::new();
    computer.eval(&vec![1002,4,3,4,33], None);
    assert_eq!(computer.mem, vec![1002,4,3,4,99]);

    computer.reset();
    computer.eval(&vec![1101,100,-1,4,0], None);
    assert_eq!(computer.mem, vec![1101,100,-1,4,99]);
}

#[test]
fn testConditional()
{
    let mut computer = IntCodeComputer::new();
    computer.eval(&vec![3,9,8,9,10,9,4,9,99,-1,8], Some(&vec![8]));
    assert_eq!(computer.output, vec![1]);

    computer.reset();
    computer.eval(&vec![3,9,7,9,10,9,4,9,99,-1,8], Some(&vec![7]));
    assert_eq!(computer.output, vec![1]);

    computer.reset();
    computer.eval(&vec![3,3,1108,-1,8,3,4,3,99], Some(&vec![7]));
    assert_eq!(computer.output, vec![0]);

    computer.reset();
    computer.eval(&vec![3,3,1107,-1,8,3,4,3,99], Some(&vec![8]));
    assert_eq!(computer.output, vec![0]);
}

#[test]
fn testJump()
{
    let mut computer = IntCodeComputer::new();
    computer.eval(&vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9],
                  Some(&vec![0]));
    assert_eq!(computer.output, vec![0]);

    computer.reset();
    computer.eval(&vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9],
                  Some(&vec![-1]));
    assert_eq!(computer.output, vec![1]);

    computer.reset();
    computer.eval(&vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1],
                  Some(&vec![0]));
    assert_eq!(computer.output, vec![0]);

    computer.reset();
    computer.eval(&vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1],
                  Some(&vec![1234]));
    assert_eq!(computer.output, vec![1]);
}

#[test]
fn testPipe()
{
    let mut computer = IntCodeComputer::new();
    computer.mem = vec![3,20,1001,20,1,21,4,21,99,10,0,0,0,0,0,0,0,0,0,0,0,0];
    assert_eq!(computer.pipe(10).unwrap(), 11);

    computer.reset();
    computer.mem = vec![3,28,1001,28,1,29,4,29,3,28,1001,28,2,29,4,29,99,18,0,0,0,0,0,0,0,0,0,0,0,0];
    assert_eq!(computer.pipe(10).unwrap(), 11);
    assert_eq!(computer.pipe(11).unwrap(), 13);
    assert!(computer.pipe(0).is_none());
}
