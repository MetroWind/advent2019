use std::stringify;
use std::fmt;
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
    derive(Copy, Clone)
}

pub fn part1(input: &str) -> usize
{
    0
}

pub fn part2(input: &str) -> usize
{
    0
}
