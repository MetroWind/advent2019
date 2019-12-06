use std::vec::Vec;
use std::str::FromStr;
use std::fmt::Display

enum ParserState
{
    Begin,
    Head,
    Arg,
    NotAllowed,
}

enum Argument
{
    Position(String),
    Immediate(i32),
}

enum StatementType
{
    Label,
    Code,
    Empty,
}

impl FromStr for Argument
{
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
        else
        {
            Ok(Argument::Position(String::from(s)))
        }
    }
}

impl Display for Argument
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self
        {
            Position(s) => write!(f, "Position({})", s),
            Immediate(x) => write!(f, "Immediate({})", x),
        }
    }
}

struct Statement
{
    the_type: StatementType,
    address: usize,
    arguments: Vec<Argument>,
    head: String,
}

impl Display for Statement
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self.the_type
        {
            StatementType::Label => write!(f, "{}. {}:", self.address, self.head),
            _ =>
            {
                let arg_str: String = Arguments.iter().map(|arg| arg.to_string())
                    .collect().join(", ");
                write!(f, "{}. {} {}", self.address, self.head, arg_str)
            }
        }
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
                cursor += 1;
            },
            ParserState::Head =>
            {
                if c.is_whitespace()
                {
                    statement.head = word.collect();
                    word.clear();
                    state = ParserState::Arg;
                    statement.the_type = StatementType::Code
                }
                else if c == ':'
                {
                    if word.is_empty()
                    {
                        return Err("Empty label");
                    }

                    statement.head = word.collect();
                    word.clear();
                    state = ParserState::NotAllowed;
                    statement.the_type = StatementType::Label
                }
                else if c == '#'
                {
                    statement.head = word.collect();
                    word.clear();
                    statement.the_type = StatementType::Code;
                    state = ParserState::End;
                }
                else
                {
                    word.push(c);
                }
                cursor += 1;
            },
            ParserState::Arg =>
            {
                if c == ',' || c == '#'
                {
                    if word.is_empty()
                    {
                        return Err("Empty argument");
                    }
                    let arg_str = word.collect().trim();
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
                cursor += 1;
            }
            ParserState::End =>
            {
                break;
            }
            ParserState::NotAllowed =>
            {
                if !(c.is_whitespace()) && c != '#'
                {
                    Err("Invalid statement");
                }
            }
        };
    }
    Ok(statement)
}

fn parse(source: &str) -> Result<Vec<Statement>, String>
{
    let mut result: Vec<Statement> = vec![];

    for line in source.lines()
    {
        let statement = parseLine(line)?;

        result.push(statement);
    }
    Ok(result)
}
