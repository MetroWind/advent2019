use std::vec::Vec;
use std::str::FromStr;
use std::fmt;
use std::collections::HashMap;

use crate::intcode;

enum ParserState
{
    Begin,
    Head,
    Arg,
    NotAllowed,
    End,
}

#[derive(Clone)]
enum Argument
{
    Position(String),
    Immediate(intcode::ValueType),
    Label(String),
    // Internal use for “call”. It’s an address that the instruction
    // writes to. Just the address number in position mode.
    ImmediatePosition(intcode::ValueType),
}

#[derive(Clone)]
enum StatementType
{
    Label,
    Code,
    Empty,
}

impl FromStr for Argument
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String>
    {

        if s.find(char::is_whitespace).is_some()
        {
            return Err(format!("Invalid argument: {}", s));
        }

        if let Ok(num) = s.parse::<intcode::ValueType>()
        {
            Ok(Argument::Immediate(num))
        }
        else if s.starts_with(":")
        {
            let name: String = s[1..].to_string();
            Ok(Argument::Label(name))
        }
        else
        {
            Ok(Argument::Position(String::from(s)))
        }
    }
}

impl fmt::Display for Argument
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self
        {
            Argument::Position(s) => write!(f, "Position({})", s),
            Argument::Immediate(x) => write!(f, "Immediate({})", x),
            Argument::Label(s) => write!(f, "Label({})", s),
            Argument::ImmediatePosition(x) => write!(f, "ImmediatePosition({})", x),
        }
    }
}

#[derive(Clone)]
pub struct Statement
{
    the_type: StatementType,
    address: usize,
    arguments: Vec<Argument>,
    head: String,
}

impl Statement
{
    pub fn len(&self) -> usize
    {
        match self.the_type
        {
            // This can get complicated later.
            StatementType::Code =>
            {
                if self.head == "call"
                {
                    3 * 4 + 3   // 3 adds and 1 jmpt
                }
                else if self.head == "rtn"
                {
                    2 * 4 + 3   // 2 adds and 1 jmpt
                }
                else
                {
                    self.arguments.len() + 1
                }
            },
            _ => 0,
        }
    }

    fn mode(&self) -> intcode::ValueType
    {
        let mut base: intcode::ValueType = 100;
        let mut mode: intcode::ValueType = 0;
        for arg in &self.arguments
        {
            mode += match arg
            {
                Argument::Immediate(_) | Argument::Label(_) => base * 1,
                Argument::Position(_) | Argument::ImmediatePosition(_) => base * 0,
            };
            base *= 10;
        }
        mode
    }

    pub fn opCode(&self) -> Result<intcode::ValueType, String>
    {
        let mode = self.mode();

        let code: intcode::ValueType = mode + match &(self.head)[..]
        {
            "add" =>
            {
                if self.arguments.len() != 3
                {
                    return Err(String::from("Add should have 3 arguments"));
                }
                1
            },
            "mult" =>
            {
                if self.arguments.len() != 3
                {
                    return Err(String::from("Mult should have 3 arguments"));
                }
                2
            },
            "input" =>
            {
                if self.arguments.len() != 1
                {
                    return Err(String::from("Input should have 1 argument"));
                }
                3
            },
            "output" =>
            {
                if self.arguments.len() != 1
                {
                    return Err(String::from("Output should have 1 argument"));
                }
                4
            },
            "jmpt" =>
            {
                if self.arguments.len() != 2
                {
                    return Err(String::from("Jmpt should have 2 arguments"));
                }
                5
            },
            "jmpf" =>
            {
                if self.arguments.len() != 2
                {
                    return Err(String::from("Jmpf should have 2 arguments"));
                }
                6
            },
            "less" =>
            {
                if self.arguments.len() != 3
                {
                    return Err(String::from("Less should have 3 arguments"));
                }
                7
            },
            "eq" =>
            {
                if self.arguments.len() != 3
                {
                    return Err(String::from("Eq should have 3 arguments"));
                }
                8
            },
            "halt" =>
            {
                if self.arguments.len() != 0
                {
                    return Err(String::from("Halt doesn't take arguments"));
                }
                99
            },
            _ => { return Err(format!("Unknown instruction: {}", self.head)); },
        };

        Ok(code)
    }
}

impl fmt::Display for Statement
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self.the_type
        {
            StatementType::Label => write!(f, "{}. {}:", self.address, self.head),
            _ =>
            {
                let arg_str: String = self.arguments.iter().map(|arg| arg.to_string())
                    .collect::<Vec<String>>().join(", ");
                write!(f, "{}. {} {}", self.address, self.head, arg_str)
            }
        }
    }
}

impl fmt::Debug for Statement
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}", self.to_string())
    }
}

fn parseLine(line: &str, address: usize) -> Result<Statement, String>
{
    let chars: Vec<char> = line.chars().chain(vec![';']).collect();

    let mut state = ParserState::Begin;
    let mut cursor: usize = 0;
    let mut word: Vec<char> = vec![];
    let mut statement = Statement
    {
        the_type: StatementType::Code,
        address: address,
        arguments: vec![],
        head: String::new(),
    };

    while cursor < chars.len()
    {
        let c = chars[cursor];

        match state
        {
            ParserState::Begin =>
            {
                if c == ';'
                {
                    statement.the_type = StatementType::Empty;
                    state = ParserState::End;
                }
                else if !c.is_whitespace()
                {
                    state = ParserState::Head;
                    word.push(c);
                }
            },
            ParserState::Head =>
            {
                if c.is_whitespace()
                {
                    statement.head = word.iter().collect();
                    word.clear();
                    state = ParserState::Arg;
                    statement.the_type = StatementType::Code
                }
                else if c == ':'
                {
                    if word.is_empty()
                    {
                        return Err(String::from("Empty label"));
                    }

                    statement.head = word.iter().collect();
                    word.clear();
                    state = ParserState::NotAllowed;
                    statement.the_type = StatementType::Label
                }
                else if c == ';'
                {
                    statement.head = word.iter().collect();
                    word.clear();
                    statement.the_type = StatementType::Code;
                    state = ParserState::End;
                }
                else
                {
                    word.push(c);
                }
            },
            ParserState::Arg =>
            {
                if c == ',' || c == ';'
                {
                    if word.is_empty()
                    {
                        return Err(String::from("Empty argument"));
                    }
                    let arg_str = String::from(word.iter().collect::<String>().trim());
                    statement.arguments.push(arg_str.parse()?);
                    word.clear();

                    if c == ';'
                    {
                        state = ParserState::End;
                    }
                }
                else
                {
                    word.push(c);
                }
            }
            ParserState::End =>
            {
                break;
            }
            ParserState::NotAllowed =>
            {
                if !(c.is_whitespace()) && c != ';'
                {
                    return Err(String::from("Invalid statement"));
                }
            }
        };
        cursor += 1;
    }
    Ok(statement)
}

pub fn parse(source: &str) -> Result<Vec<Statement>, String>
{
    let mut result: Vec<Statement> = vec![];
    let mut address: usize = 0;

    for line in source.lines()
    {
        let statement: Statement = parseLine(line, address)?;
        address += statement.len();
        result.push(statement);
    }
    Ok(result)
}

pub fn assemble(statements_raw: &Vec<Statement>) -> Result<Vec<intcode::ValueType>, String>
{
    let mut code: Vec<intcode::ValueType> = vec![];
    if statements_raw.is_empty()
    {
        return Ok(code);
    }

    let mut address_labels: HashMap<&str, usize> = HashMap::new();
    let mut address_vars: HashMap<&str, usize> = HashMap::new();
    let last_statem = statements_raw.last().unwrap();
    let code_size = last_statem.address + last_statem.len();
    let stack_ptr_addr = code_size as intcode::ValueType;
    let stack_size: usize = 10;
    // The 1 is for the stack pointer.
    let data_start: usize = code_size + stack_size + 1;

    let mut has_function = false;

    // First pass, expand function calls and returns.
    let mut statements: Vec<Statement> = vec![];

    for statement in statements_raw
    {
        if statement.head == "call"
        {
            has_function = true;
            // Find the correct place in stack and write it to stack pointer.
            let stack_statem: Statement = Statement
            {
                the_type: StatementType::Code,
                address: statement.address,
                arguments: vec![Argument::ImmediatePosition(stack_ptr_addr),
                                Argument::Immediate(1),
                                Argument::ImmediatePosition(stack_ptr_addr)],
                head: String::from("add"),
            };

            // The next command after the following will write the
            // return address to the correct place in stack, but it
            // doesn’t known what that address is. Tell it.
            let write_return_addr_statem: Statement = Statement
            {
                the_type: StatementType::Code,
                address: statement.address + stack_statem.len(),
                arguments: vec![Argument::ImmediatePosition(stack_ptr_addr),
                                Argument::Immediate(0),
                                Argument::ImmediatePosition(
                                    (statement.address + stack_statem.len() + 4 + 3)
                                        as intcode::ValueType)],
                head: String::from("add"),
            };

            // Write the return address to the correct place in stack.
            let return_addr_statem = Statement
            {
                the_type: StatementType::Code,
                address: statement.address + stack_statem.len()
                    + write_return_addr_statem.len(),
                arguments: vec![Argument::Immediate(
                    (statement.address + statement.len()) as intcode::ValueType),
                                Argument::Immediate(0),
                                // This one doesn’t matter, it’ll be
                                // overwritten by the previous
                                // instruction.
                                Argument::ImmediatePosition(0)],
                head: String::from("add"),
            };

            // Now we can jump to function.
            let jmp_statem = Statement
            {
                the_type: StatementType::Code,
                address: statement.address + stack_statem.len()
                    + write_return_addr_statem.len() + return_addr_statem.len(),
                arguments: vec![Argument::Immediate(1),
                                statement.arguments[0].clone()],
                head: String::from("jmpt"),
            };

            statements.push(stack_statem);
            statements.push(write_return_addr_statem);
            statements.push(return_addr_statem);
            statements.push(jmp_statem);
        }

        else if statement.head == "rtn"
        {
            // Find the current stack head, and tell the jmpt command about it.
            let return_addr_statem = Statement
            {
                the_type: StatementType::Code,
                address: statement.address,
                arguments: vec![Argument::ImmediatePosition(stack_ptr_addr),
                                Argument::Immediate(0),
                                Argument::ImmediatePosition(
                                    (statement.address + 4 + 4+ 2) as intcode::ValueType)],
                head: String::from("add"),
            };

            // Decrease stack head by 1.
            let stack_ptr_update_statem = Statement
            {
                the_type: StatementType::Code,
                address: statement.address + return_addr_statem.len(),
                arguments: vec![Argument::ImmediatePosition(stack_ptr_addr),
                                Argument::Immediate(-1),
                                Argument::ImmediatePosition(stack_ptr_addr)],
                head: String::from("add"),
            };

            let jmp_statem = Statement
            {
                the_type: StatementType::Code,
                address: statement.address + return_addr_statem.len()
                    + stack_ptr_update_statem.len(),
                arguments: vec![Argument::Immediate(1),
                                // This one doesn’t matter, it’ll be
                                // overwritten by the previous
                                // previous instruction.
                                Argument::ImmediatePosition(0)],
                head: String::from("jmpt"),
            };

            statements.push(return_addr_statem);
            statements.push(stack_ptr_update_statem);
            statements.push(jmp_statem);
        }

        else
        {
            statements.push(statement.clone());
        }
    }

    // First pass, find all the lables.
    for statement in &statements
    {
        // println!("{}", statement);
        match statement.the_type
        {
            StatementType::Label =>
            {
                if address_labels.contains_key(&statement.head[..])
                {
                    return Err(format!("Duplicated lable: {}", statement.head));
                }
                address_labels.insert(&statement.head, statement.address);
            },
            _ => {}
        }
    }

    // Second pass, fill in addresses.
    for statement in &statements
    {
        match statement.the_type
        {
            StatementType::Code =>
            {
                code.push(statement.opCode()?);
                for arg in &statement.arguments
                {
                    match arg
                    {
                        Argument::Immediate(x) =>
                        {
                            code.push(x.clone());
                        },
                        Argument::Position(var) =>
                        {
                            if address_vars.contains_key(&var[..])
                            {
                                code.push(address_vars.get(&var[..]).unwrap().clone() as intcode::ValueType);
                            }
                            else
                            {
                                let addr = address_vars.len() + data_start;
                                address_vars.insert(&var[..], addr);
                                code.push(addr as intcode::ValueType);
                            }
                        },
                        Argument::Label(label) =>
                        {
                            if address_labels.contains_key(&label[..])
                            {
                                code.push(address_labels.get(&label[..])
                                          .unwrap().clone() as intcode::ValueType);
                            }
                            else
                            {
                                return Err(format!("Undefined label: {}", label));
                            }
                        },
                        Argument::ImmediatePosition(x) =>
                        {
                            code.push(x.clone());
                        },
                    }
                }
            },
            _ => {},
        }
    }

    if has_function
    {
        // Stack pointer
        code.push((code_size + 1) as intcode::ValueType);

        for _ in 0..stack_size
        {
            code.push(0);
        }
    }

    // Initialize variables
    for _ in 0..address_vars.len()
    {
        code.push(0);
    }
    Ok(code)
}
