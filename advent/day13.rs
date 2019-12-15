use std::stringify;
use std::fmt;
use std::cmp::PartialEq;
use std::collections::HashMap;

use crate::intcode::intcode;

#[macro_export]
macro_rules! makeIntEnum
{
    {
        $name:ident
        {
            $( $sub_name:ident $( = $x:literal )? ,)+
        } with $underlying_type:ty,
        $( derive( $($traits:ident),+ ) )?
    } => {
        $( #[derive(fmt::Debug, $($traits),+) ] )?
        enum $name
        {
            $( $sub_name $( = $x )? ,)+
        }

        impl $name
        {
            pub fn from(x: $underlying_type) -> Result<Self, String>
            {
                match x
                {
                    $( x if x == (Self::$sub_name as $underlying_type) => Ok(Self::$sub_name), )+
                        _ => Err(format!("Unknown convertion from {} to {}", x, stringify!($name))),
                }
            }
        }

        impl fmt::Display for $name
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
            {
                let represent = match self
                {
                    $( Self::$sub_name => stringify!($sub_name), )+
                };
                write!(f, "{}", represent)
            }
        }
    }
}


makeIntEnum!
{
    TileType
    {
        Empty = 0,
        Wall = 1,
        Block = 2,
        Paddle = 3,
        Ball = 4,
    } with intcode::ValueType,
    derive(Copy, Clone, PartialEq)
}

fn runGame(code: &Vec<intcode::ValueType>) -> HashMap<(intcode::ValueType, intcode::ValueType), TileType>
{
    let mut computer = intcode::IntCodeComputer::new();
    let mut field = HashMap::new();
    computer.loadCode(code);
    loop
    {
        let x = if let Some(output) = computer.pipe(None)
        {
            output
        }
        else
        {
            return field;
        };

        let y = computer.pipe(None).unwrap();
        let tile = TileType::from(computer.pipe(None).unwrap()).expect("Unknown tile");
        field.insert((x, y), tile);
    }
}

fn runGameWithScore(code: &Vec<intcode::ValueType>) -> intcode::ValueType
{
    let mut computer = intcode::IntCodeComputer::new();
    let mut field = HashMap::new();
    computer.loadCode(code);
    computer.mem[0] = 2;
    let mut ball_x: intcode::ValueType = -1;
    let mut pad_x: intcode::ValueType = -1;
    let mut score = 0;
    let mut input: Option<intcode::ValueType> = None;
    loop
    {
        let x = if let Some(output) = computer.pipe(input)
        {
            output
        }
        else
        {
            return score;
        };

        let y = computer.pipe(input).unwrap();

        if x == -1 && y == 0
        {
            score = computer.pipe(input).unwrap();
        }
        else
        {
            let tile = TileType::from(computer.pipe(input).unwrap())
                .expect("Unknown tile");
            field.insert((x, y), tile);

            if tile == TileType::Paddle
            {
                pad_x = x;
            }
            else if tile == TileType::Ball
            {
                ball_x = x;
            }

            if pad_x != -1 && ball_x != -1
            {
                if pad_x < ball_x
                {
                    input = Some(1);
                }
                else if pad_x > ball_x
                {
                    input = Some(-1);
                }
                else
                {
                    input = Some(0);
                }
            }
        }
    }
}

pub fn part1(input: &str) -> usize
{
    let code = intcode::parse(input);
    let field = runGame(&code);

    let mut count = 0;
    for (_, tile) in field
    {
        if tile == TileType::Block
        {
            count += 1;
        }
    }
    count
}

pub fn part2(input: &str) -> intcode::ValueType
{
    let code = intcode::parse(input);
    runGameWithScore(&code)
}
