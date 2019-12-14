use crate::intcode::intcode;

#[macro_export]
macro_rules! makeEnum
{
    {
        $name:ident
        {
            $( $sub_name:ident $( = $x:literal )? ,)+
        } with $underlying_type:ty,
        $( derive( $traits:tt ) )?
    } => {$( #[derive( $traits )] )?
          enum $name
          {
              $( $sub_name $( = $x )? ,)+
          }

          impl $name
          {
              pub fn from(x: $underlying_type) -> Self
              {
                  match x
                  {
                      $( x if x == Self::$sub_name as $underlying_type => Self::$sub_name, )+
                      _ => unreachable!()
                  }
              }
          }
    }
}

makeEnum!
{
    TileType
    {
        Empty,
        Wall,
    } with i32,
    derive(Copy, Clone)

}
// ////
enum TileType
{
    Empty = 0,
    Wall = 1,
    Block = 2,
    Paddle = 3,
    Ball = 4,
}

impl TileType
{
    pub fn from(x: intcode::ValueType) -> Self
    {
        match x
        {
            x if x == Self::Empty as intcode::ValueType => Self::Empty,
            x if x == Self::B as intcode::ValueType => x,
            x if x == Self::C as intcode::ValueType => x,
            _ => ...
}    }
}

pub fn part1(input: &str) -> usize
{
}

pub fn part2(input: &str) -> usize
{
}
