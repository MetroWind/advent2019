use std::vec::Vec;
use std::str::FromStr;
use std::fmt;

enum ParserState
{
    Begin,
    Head,
    Arg,
    NotAllowed,
    End,
}

enum Argument
{
    Position(String),
    Immediate(i32),
    Label(String),
}

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

        if let Ok(num) = s.parse::<i32>()
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
        }
    }
}

pub struct Statement
{
    the_type: StatementType,
    address: usize,
    arguments: Vec<Argument>,
    head: String,
}

impl Statement
{
    fn len(&self) -> usize
    {
        match self.the_type
        {
            // This can get complicated later.
            StatementType::Code => self.arguments.len() + 1,
            _ => 0,
        }
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
    let chars: Vec<char> = line.chars().chain(vec!['#']).collect();

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
                if c == '#'
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
                else if c == '#'
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
                if c == ',' || c == '#'
                {
                    if word.is_empty()
                    {
                        return Err(String::from("Empty argument"));
                    }
                    let arg_str = String::from(word.iter().collect::<String>().trim());
                    statement.arguments.push(arg_str.parse()?);
                    word.clear();

                    if c == '#'
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
                if !(c.is_whitespace()) && c != '#'
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
