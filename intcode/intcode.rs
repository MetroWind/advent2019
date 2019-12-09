#![allow(non_snake_case)]

use std::vec::Vec;

pub type ValueType = i64;

pub fn parse(code: &str) -> Vec<ValueType>
{
    code.split(',').map(
        |part|
        {
            let op = part.trim().parse::<ValueType>();
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
    Relative,
}

impl ArgMode
{
    pub fn fromDigit(x: u8) -> Result<ArgMode, ()>
    {
        match x
        {
            0 => Ok(ArgMode::Position),
            1 => Ok(ArgMode::Immediate),
            2 => Ok(ArgMode::Relative),
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
    fn fromInt(code: ValueType) -> Result<OpCode, ()>
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
            9 => 1,
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
    pub mem: Vec<ValueType>,
    cursor: usize,
    halt: bool,
    input: Vec<ValueType>,
    cursor_input: usize,
    pub output: Vec<ValueType>,
    offset: ValueType,

    one_input: ValueType,
    one_output: ValueType,
}

impl IntCodeComputer
{
    pub fn new() -> IntCodeComputer
    {
        IntCodeComputer
        {
            mem: vec![0; 8192],
            cursor: 0,
            halt: false,
            input: vec![],
            cursor_input: 0,
            output: vec![],
            offset: 0,

            one_input: 0,
            one_output: 0,
        }
    }

    #[allow(dead_code)]
    pub fn reset(&mut self)
    {
        self.mem = vec![0; 8192];
        self.cursor = 0;
        self.halt = false;
        self.input.clear();
        self.cursor_input = 0;
        self.output.clear();
        self.offset = 0;

        self.one_input = 0;
        self.one_output = 0;
    }

    pub fn loadCode(&mut self, code: &Vec<ValueType>)
    {
        for i in 0..code.len()
        {
            self.mem[i] = code[i];
        }
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
            9 => { self.evalOffset(code); }
            _ => unreachable!(),
        };
    }

    pub fn eval(&mut self, input: Option<&Vec<ValueType>>)
    {
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
    pub fn consumeSingleInput(&mut self, input: ValueType)
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
    pub fn pipe(&mut self, input: ValueType) -> Option<ValueType>
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

    fn getAddress(&self, code: &OpCode, index: u8) -> usize
    {
        match code.arg_modes[index as usize]
        {
            ArgMode::Position =>
                self.mem[self.cursor + (index as usize) + 1] as usize,
            ArgMode::Immediate => self.cursor + (index as usize) + 1,
            ArgMode::Relative =>
            {
                let addr_base = self.mem[self.cursor + (index as usize) + 1];
                (addr_base + self.offset) as usize
            },
        }
    }

    fn getArg(&self, code: &OpCode, index: u8) -> ValueType
    {
        self.mem[self.getAddress(code, index)]
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

        let result_addr = self.getAddress(code, 2);
        self.mem[result_addr] = lhs + rhs;
        self.skip(code);
    }

    fn evalMult(&mut self, code: &OpCode)
    {
        let lhs = self.getArg(code, 0);
        let rhs = self.getArg(code, 1);

        let result_addr = self.getAddress(code, 2);
        self.mem[result_addr] = lhs * rhs;
        self.skip(code);
    }

    fn evalInput(&mut self, code: &OpCode)
    {
        let result_addr = self.getAddress(code, 0);
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
        let result_addr = self.getAddress(code, 2);
        self.mem[result_addr] = if lhs < rhs {1} else {0};
        self.skip(code);
    }

    fn evalEqual(&mut self, code: &OpCode)
    {
        let lhs = self.getArg(code, 0);
        let rhs = self.getArg(code, 1);
        let result_addr = self.getAddress(code, 2);
        self.mem[result_addr] = if lhs == rhs {1} else {0};
        self.skip(code);
    }

    fn evalOffset(&mut self, code: &OpCode)
    {
        self.offset += self.getArg(code, 0);
        self.skip(code);
    }
}

// ========== Tests =================================================>

#[test]
fn testAdd()
{
    let mut computer = IntCodeComputer::new();
    computer.loadCode(&vec![1,0,0,0,99]);
    computer.eval(None);
    assert_eq!(computer.mem[..5].to_vec(), vec![2,0,0,0,99]);
}

#[test]
fn testMult()
{
    let mut computer = IntCodeComputer::new();

    computer.loadCode(&vec![2,3,0,3,99]);
    computer.eval(None);
    assert_eq!(computer.mem[..5].to_vec(), vec![2,3,0,6,99]);

    computer.reset();
    computer.loadCode(&vec![2,4,4,5,99,0]);
    computer.eval(None);
    assert_eq!(computer.mem[..6].to_vec(), vec![2,4,4,5,99,9801]);
}

#[test]
fn testBasic()
{
    let mut computer = IntCodeComputer::new();
    computer.loadCode(&vec![1,1,1,4,99,5,6,0,99]);
    computer.eval(None);
    let expected = vec![30,1,1,4,2,5,6,0,99];
    assert_eq!(computer.mem[..expected.len()].to_vec(), expected);
}

#[test]
fn testIO()
{
    let mut computer = IntCodeComputer::new();
    computer.loadCode(&vec![3,0,99]);
    computer.eval(Some(&vec![10]));
    assert_eq!(computer.mem[..3].to_vec(), vec![10,0,99]);

    computer.reset();
    computer.loadCode(&vec![3,0,3,1,99]);
    computer.eval(Some(&vec![10,20]));
    assert_eq!(computer.mem[..5].to_vec(), vec![10,20,3,1,99]);

    computer.reset();
    computer.loadCode(&vec![4,0,99]);
    computer.eval(None);
    assert_eq!(computer.output, vec![4]);

    computer.reset();
    computer.loadCode(&vec![4,0,4,4,99]);
    computer.eval(None);
    assert_eq!(computer.output, vec![4,99]);
}

#[test]
fn testModes()
{
    let mut computer = IntCodeComputer::new();
    computer.loadCode(&vec![1002,4,3,4,33]);
    computer.eval(None);
    assert_eq!(computer.mem[..5].to_vec(), vec![1002,4,3,4,99]);

    computer.reset();
    computer.loadCode(&vec![1101,100,-1,4,0]);
    computer.eval(None);
    assert_eq!(computer.mem[..5].to_vec(), vec![1101,100,-1,4,99]);
}

#[test]
fn testConditional()
{
    let mut computer = IntCodeComputer::new();
    computer.loadCode(&vec![3,9,8,9,10,9,4,9,99,-1,8]);
    computer.eval(Some(&vec![8]));
    assert_eq!(computer.output, vec![1]);

    computer.reset();
    computer.loadCode(&vec![3,9,7,9,10,9,4,9,99,-1,8]);
    computer.eval(Some(&vec![7]));
    assert_eq!(computer.output, vec![1]);

    computer.reset();
    computer.loadCode(&vec![3,3,1108,-1,8,3,4,3,99]);
    computer.eval(Some(&vec![7]));
    assert_eq!(computer.output, vec![0]);

    computer.reset();
    computer.loadCode(&vec![3,3,1107,-1,8,3,4,3,99]);
    computer.eval(Some(&vec![8]));
    assert_eq!(computer.output, vec![0]);
}

#[test]
fn testJump()
{
    let mut computer = IntCodeComputer::new();
    computer.loadCode(&vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9]);
    computer.eval(Some(&vec![0]));
    assert_eq!(computer.output, vec![0]);

    computer.reset();
    computer.loadCode(&vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9]);
    computer.eval(Some(&vec![-1]));
    assert_eq!(computer.output, vec![1]);

    computer.reset();
    computer.loadCode(&vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1]);
    computer.eval(Some(&vec![0]));
    assert_eq!(computer.output, vec![0]);

    computer.reset();
    computer.loadCode(&vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1]);
    computer.eval(Some(&vec![1234]));
    assert_eq!(computer.output, vec![1]);
}

#[test]
fn testPipe()
{
    let mut computer = IntCodeComputer::new();
    computer.loadCode(&vec![3,20,1001,20,1,21,4,21,99,10,0,0,0,0,0,0,0,0,0,0,0,0]);
    assert_eq!(computer.pipe(10).unwrap(), 11);

    computer.reset();
    computer.loadCode(&vec![3,28,1001,28,1,29,4,29,3,28,1001,28,2,29,4,29,99,
                           18,0,0,0,0,0,0,0,0,0,0,0,0]);
    assert_eq!(computer.pipe(10).unwrap(), 11);
    assert_eq!(computer.pipe(11).unwrap(), 13);
    assert!(computer.pipe(0).is_none());
}

#[test]
fn testOffset()
{
    let mut computer = IntCodeComputer::new();

    let code = vec![109, 1, 1201, -1, 1, 0, 99];
    computer.loadCode(&code);
    computer.eval(None);
    assert_eq!(computer.mem[0], 110 as ValueType);

    computer.reset();
    let code = vec![109, 1, 1201, -1, 1, 0, 204, -1, 99];
    computer.loadCode(&code);
    computer.eval(None);
    assert_eq!(computer.output, vec![110]);

    computer.reset();
    let code = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
    computer.loadCode(&code);
    computer.eval(None);
    assert_eq!(computer.output, code);
}

#[test]
fn testLong()
{
    let mut computer = IntCodeComputer::new();
    let code = vec![1102,34915192,34915192,7,4,7,99,0];
    computer.loadCode(&code);
    computer.eval(None);
    assert_eq!(computer.output[0].to_string().len(), 16);
}
