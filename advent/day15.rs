use std::fmt;
use std::vec::Vec;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::io::Write;

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

fn drawState(filename: &Path, pos: &CoordType, walls: &HashSet<CoordType>,
             path: &Vec<CoordType>, target: &Option<CoordType>,
             probed: &HashSet<CoordType>)
{
    let cell_size = 4;
    let mut svg = fs::File::create(filename).unwrap();
    let drawBox = |f: &mut fs::File, at: &CoordType, color: &str|
    {
        writeln!(f, r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" stroke="none" />""#, at.0 * cell_size, at.1 * cell_size, cell_size, cell_size, color).unwrap();
    };
    writeln!(svg, r#"<svg viewBox="{} {} {} {}" xmlns="http://www.w3.org/2000/svg">"#,
             -25 * cell_size, -25 * cell_size, 50 * cell_size, 50 * cell_size)
        .unwrap();
    writeln!(svg, r#"<rect x="{}" y="{}" width="{}" height="{}" fill="black" />"#,
             -25 * cell_size, -25 * cell_size, 50 * cell_size, 50 * cell_size)
        .unwrap();

    for wall in walls
    {
        drawBox(&mut svg, wall, "#ffffff");
    }

    for site in probed
    {
        drawBox(&mut svg, site, "#a4b0be");
    }

    for site in path
    {
        drawBox(&mut svg, site, "#1e90ff");
    }

    if let Some(point) = target
    {
        drawBox(&mut svg, &point, "#2ed573");
    }

    drawBox(&mut svg, pos, "#ff4757");

    writeln!(svg, "</svg>");
}


fn step(core: &mut intcode::IntCodeComputer, dir: MoveDirection) -> Response
{
    Response::from(core.pipe(Some(dir as intcode::ValueType)).unwrap()).unwrap()
}

struct ProbeResult
{
    dir: Option<MoveDirection>,
    reached: bool,
    found_walls: HashSet<CoordType>,
}

fn findForwardNext(core: &mut intcode::IntCodeComputer, pos: &CoordType,
                   path: &Vec<CoordType>, probed: &HashSet<CoordType>)
                   -> ProbeResult
{
    let mut walls: HashSet<CoordType> = HashSet::new();
    if path.is_empty()
    {
        for i in 1..=4
        {
            let dir = MoveDirection::from(i).unwrap();
            let new_pos = dir.ofPos(&pos);
            match step(core, dir)
            {
                Response::Wall =>
                {
                    walls.insert(new_pos.clone());
                },
                Response::Moved =>
                {
                    step(core, dir.opposite());
                    if !probed.contains(&new_pos)
                    {
                        return ProbeResult
                        {
                            dir: Some(dir),
                            reached: false,
                            found_walls: walls,
                        };
                    }
                },
                Response::Reached =>
                {
                    step(core, dir.opposite());
                        return ProbeResult
                        {
                            dir: Some(dir),
                            reached: true,
                            found_walls: walls,
                        };
                },
            }
        }
        return ProbeResult
        {
            dir: None,
            reached: false,
            found_walls: walls,
        };
    }
    else
    {
        let back_dir = MoveDirection::fromPosToPos(pos, path.last().unwrap());
        let mut dir = back_dir.right();
        while dir != back_dir
        {
            let new_pos = dir.ofPos(&pos);
            match step(core, dir)
            {
                Response::Wall =>
                {
                    walls.insert(new_pos.clone());
                },
                Response::Moved =>
                {
                    step(core, dir.opposite());
                    if !probed.contains(&new_pos)
                    {
                        return ProbeResult
                        {
                            dir: Some(dir),
                            reached: false,
                            found_walls: walls,
                        };
                    }
                },
                Response::Reached =>
                {
                    step(core, dir.opposite());
                    if !probed.contains(&new_pos)
                    {
                        return ProbeResult
                        {
                            dir: Some(dir),
                            reached: true,
                            found_walls: walls,
                        };
                    }
                },
            }
            dir = dir.right();
        }
        return ProbeResult
        {
            dir: None,
            reached: false,
            found_walls: walls,
        };
    }
}

enum BotState
{
    Forward,
    Backward,
}

fn findPathLength(core: &mut intcode::IntCodeComputer) -> usize
{
    let mut path: Vec<CoordType> = Vec::new(); // Doesn’t include current pos.
    let mut pos: CoordType = (0, 0);
    let mut initial_dir = MoveDirection::West;
    let mut probed: HashSet<CoordType> = HashSet::new();
    let mut walls: HashSet<CoordType> = HashSet::new();
    let mut min_length = usize::max_value();
    let mut frame_idx = 0;
    let mut target: Option<CoordType> = None;
    let mut state = BotState::Forward;

    let svg_dir = Path::new("frames");
    if !svg_dir.exists()
    {
        fs::create_dir_all(svg_dir);
    }

    loop
    {
        drawState(&svg_dir.join(format!("{:05}.svg", frame_idx)),
                  &pos, &walls, &path, &target, &probed);

        match state
        {
            BotState::Forward =>
            {
                let result = findForwardNext(core, &pos, &path, &probed);
                walls.extend(result.found_walls.iter());
                if let Some(dir) = result.dir
                {
                    let new_pos = dir.ofPos(&pos);
                    if result.reached
                    {
                        min_length = min_length.min(path.len());
                        target = Some(new_pos);
                    }
                    step(core, dir);
                    path.push(pos);
                    pos = new_pos;
                }
                else
                {
                    state = BotState::Backward;
                }

            },

            BotState::Backward =>
            {
                if path.is_empty()
                {
                    // No path.
                    return min_length;
                }

                let back_dir = MoveDirection::fromPosToPos(&pos, path.last().unwrap());
                probed.insert(pos.clone());
                step(core, back_dir);
                pos = path.pop().unwrap();
                state = BotState::Forward;
            },
        }
        frame_idx += 1;
    }
}

pub fn part1(input: &str) -> usize
{
    let mut computer = intcode::IntCodeComputer::new();
    let code = intcode::parse(input);
    computer.loadCode(&code);
    findPathLength(&mut computer) + 1
}

pub fn part2(input: &str) -> u32
{
    0
}
