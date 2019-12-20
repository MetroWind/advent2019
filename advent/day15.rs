use std::fmt;
use std::vec::Vec;
use std::collections::HashSet;

use crate::intcode::intcode;
use crate::makeIntEnum;

type CoordType = (i32, i32);

makeIntEnum!
{
    MoveDirection
    {
        North = 1,
        South = 2,
        West = 3,
        East = 4,
    } with intcode::ValueType,
    derive(Copy, Clone, PartialEq)
}

impl MoveDirection
{
    pub fn right(&self) -> Self
    {
        match self
        {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    pub fn opposite(&self) -> Self
    {
        match self
        {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }

    pub fn left(&self) -> Self
    {
        self.opposite().right()
    }

    pub fn ofPos(&self, pos: &CoordType) -> CoordType
    {
        match self
        {
            MoveDirection::North => (pos.0, pos.1 - 1),
            MoveDirection::South => (pos.0, pos.1 + 1),
            MoveDirection::East => (pos.0 + 1, pos.1),
            MoveDirection::West => (pos.0 - 1, pos.1),
        }
    }

    pub fn fromPosToPos(from: &CoordType, to: &CoordType) -> Self
    {
        if from.0 == to.0
        {
            if to.1 > from.1
            {
                Self::South
            }
            else
            {
                Self::North
            }
        }
        else
        {
            if to.0 > from.0
            {
                Self::East
            }
            else
            {
                Self::West
            }
        }
    }
}

makeIntEnum!
{
    Response
    {
        Wall = 0,
        Moved = 1,
        Reached = 2,
    } with intcode::ValueType,
    derive(Copy, Clone, PartialEq)
}

fn step(core: &mut intcode::IntCodeComputer, dir: MoveDirection) -> Response
{
    Response::from(core.pipe(Some(dir as intcode::ValueType)).unwrap()).unwrap()
}

fn findPathLength(core: &mut intcode::IntCodeComputer) -> usize
{
    let mut path: Vec<CoordType> = Vec::new();
    let mut pos: CoordType = (0, 0);
    let mut initial_dir = MoveDirection::West;
    let mut probed: HashSet<CoordType> = HashSet::new();
    let mut min_length = usize::max_value();
    loop
    {
        let back_dir = if path.is_empty()
        {
            initial_dir
        }
        else
        {
            MoveDirection::fromPosToPos(&pos, &path.last().unwrap())
        };

        let mut dir = initial_dir;

        loop
        {
            match step(core, dir)
            {
                Response::Wall =>
                {
                    dir = dir.right();
                },

                Response::Moved =>
                {
                    if path.is_empty() || pos != path.last().unwrap()
                    {
                        path.push(pos);
                    }
                    pos = dir.ofPos(&pos);
                    initial_dir = dir.left();

                    if probed.contains(&pos)
                    {
                        // Trigger a backtrack
                        dir = back_dir;
                    }
                    else
                    {
                        break;
                    }
                },

                Response::Reached =>
                {
                    path.push(pos);
                    min_length = min_length.min(path.len());
                },
            };

            if dir == back_dir
            {
                // Dead end. At this point, the current pos has not been pushed yet.
                if path.is_empty()
                {
                    // No path.
                    return min_length;
                }

                step(core, back_dir);
                pos = path.pop().unwrap();
                probed.insert(pos.clone());
                initial_dir = back_dir.left();
                break;
            }
        }

    }
}

pub fn part1(input: &str) -> usize
{
    let mut computer = intcode::IntCodeComputer::new();
    let code = intcode::parse(input);
    computer.loadCode(&code);
    findPathLength(&mut computer)
}

pub fn part2(input: &str) -> u32
{
    0
}
