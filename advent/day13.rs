use std::stringify;
use std::string::String;
use std::fmt;
use std::fmt::Write as FmtWrite;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

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

fn drawField(field: &HashMap<(intcode::ValueType, intcode::ValueType), TileType>) -> String
{
    let mut svg = String::new();
    let tile_size = 16;
    writeln!(svg, r#"<svg xmlns="http://www.w3.org/2000/svg">"#).expect("Failed to write SVG.");
    for (loc, tile) in field
    {
        match tile
        {
            TileType::Wall =>
            {
                writeln!(svg, r#"<rect x="{x}" y="{y}" width="{size}" height="{size}" fill="{color}" stroke="none" />"#,
                         x=loc.0 * tile_size, y=loc.1 * tile_size, size=tile_size,
                         color="grey").expect("Failed to write SVG.");
            },

            TileType::Block =>
            {
                writeln!(svg, r#"<rect x="{x}" y="{y}" width="{size}" height="{size}" fill="{color}" stroke="none" />"#,
                         x=loc.0 * tile_size, y=loc.1 * tile_size,
                         size=tile_size, color="blue").expect("Failed to write SVG.");
            },

            TileType::Ball =>
            {
                writeln!(svg, r#"<circle cx="{x}" cy="{y}" r="{size}" fill="{color}" stroke="none" />"#,
                         x=loc.0 * tile_size + tile_size / 2,
                         y=loc.1 * tile_size + tile_size / 2,
                         size=tile_size / 2, color="red").expect("Failed to write SVG.");
            },

            TileType::Paddle =>
            {
                writeln!(svg, r#"<rect x="{x}" y="{y}" width="{size}" height="{size}" fill="{color}" stroke="none" />"#,
                         x=loc.0 * tile_size, y=loc.1 * tile_size, size=tile_size, color="black")
                    .expect("Failed to write SVG.");
            },

            TileType::Empty => {},
        }
    }

    writeln!(svg, "</svg>").expect("Failed to write SVG.");
    svg
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
    let mut frame = 0;
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
                let mut output = File::create(format!("frames/{:05}.svg", frame))
                    .unwrap();
                write!(output, "{}", drawField(&field)).expect("Failed to write SVG.");
                frame += 1;
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
