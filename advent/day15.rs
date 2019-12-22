use std::fmt;
use std::vec::Vec;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::io::Write;
use std::fmt::Write as FmtWrite;

use crate::intcode::intcode;
use crate::makeIntEnum;

type CoordType = (intcode::ValueType, intcode::ValueType);
const SVG_CELL_SIZE: intcode::ValueType = 4;

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

    #[allow(dead_code)]
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

#[allow(dead_code)]
fn drawState(filename: &Path, pos: &CoordType, walls: &HashSet<CoordType>,
             path: &Vec<CoordType>, target: &Option<CoordType>,
             probed: &HashSet<CoordType>)
{
    let mut svg = fs::File::create(filename).unwrap();
    let drawBox = |f: &mut fs::File, at: &CoordType, color: &str|
    {
        writeln!(f, r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" stroke="none" />""#, at.0 * SVG_CELL_SIZE, at.1 * SVG_CELL_SIZE, SVG_CELL_SIZE, SVG_CELL_SIZE, color).unwrap();
    };
    writeln!(svg, r#"<svg viewBox="{} {} {} {}" xmlns="http://www.w3.org/2000/svg">"#,
             -25 * SVG_CELL_SIZE, -25 * SVG_CELL_SIZE, 50 * SVG_CELL_SIZE, 50 * SVG_CELL_SIZE)
        .unwrap();
    writeln!(svg, r#"<rect x="{}" y="{}" width="{}" height="{}" fill="black" />"#,
             -25 * SVG_CELL_SIZE, -25 * SVG_CELL_SIZE, 50 * SVG_CELL_SIZE, 50 * SVG_CELL_SIZE)
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

    writeln!(svg, "</svg>").unwrap();
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

#[derive(Copy, Clone, PartialEq)]
enum TileType
{
    Wall,
    Empty,
    Unknown,
}

struct Field
{
    data: Vec<TileType>,
    pub target: CoordType,
    x_bound: (intcode::ValueType, intcode::ValueType),
    y_bound: (intcode::ValueType, intcode::ValueType),
}

fn drawBox(f: &mut String, at: &CoordType, color: &str)
{
    writeln!(f, r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" stroke="none" />"#, at.0 * SVG_CELL_SIZE, at.1 * SVG_CELL_SIZE, SVG_CELL_SIZE, SVG_CELL_SIZE, color).unwrap();
}

impl Field
{
    fn width(&self) -> usize
    {
        (self.x_bound.1 - self.x_bound.0) as usize
    }

    fn height(&self) -> usize
    {
        (self.y_bound.1 - self.y_bound.0) as usize
    }

    fn coord2Index(&self, coord: &CoordType) -> usize
    {
        (coord.1 - self.y_bound.0) as usize * self.width() + (coord.0 - self.x_bound.0) as usize
    }

    fn index2Coord(&self, i: usize) -> CoordType
    {
        ((i % self.width()) as intcode::ValueType + self.x_bound.0,
         (i / self.width()) as intcode::ValueType + self.y_bound.0)
    }

    pub fn at(&self, coord: &CoordType) -> TileType
    {
        self.data[self.coord2Index(coord)]
    }

    pub fn svgViewBox(&self) -> String
    {
        format!(r#"viewBox="{} {} {} {}""#,
                self.x_bound.0 * SVG_CELL_SIZE, self.y_bound.0 * SVG_CELL_SIZE,
                (self.width() + 1) * SVG_CELL_SIZE as usize, (self.height() + 1) * SVG_CELL_SIZE as usize)
    }

    pub fn svgBg(&self) -> String
    {
        format!(r#"<rect x="{}" y="{}" width="{}" height="{}" fill="black" Stoke="none" />"#,
                self.x_bound.0 * SVG_CELL_SIZE, self.y_bound.0 * SVG_CELL_SIZE,
                self.width() * SVG_CELL_SIZE as usize, self.height() * SVG_CELL_SIZE as usize)
    }

    pub fn drawBody(&self) -> String
    {
        let mut svg = String::new();
        for i in 0..self.data.len()
        {
            let coord = self.index2Coord(i);
            if coord == self.target
            {
                drawBox(&mut svg, &coord, "#2ed573");
            }
            else if self.data[i] == TileType::Wall || self.data[i] == TileType::Unknown
            {
                drawBox(&mut svg, &coord, "white");
            }
        }
        svg
    }

    pub fn drawFieldWithAir(&self, aired: &HashSet<CoordType>) -> String
    {
        let mut svg = String::new();
        writeln!(svg, r#"<svg {} xmlns="http://www.w3.org/2000/svg">"#, self.svgViewBox()).unwrap();
        writeln!(svg, "{}", self.svgBg()).unwrap();
        writeln!(svg, "{}", self.drawBody()).unwrap();

        for air in aired
        {
            drawBox(&mut svg, air, "#a4b0be");
        }

        writeln!(svg, "</svg>").unwrap();
        svg
    }

    pub fn from(walls: &HashSet<CoordType>, probed: &HashSet<CoordType>, target: &CoordType) -> Self
    {
        let mut result = Field
        {
            data: vec![],
            x_bound: (0, 0),
            y_bound: (0, 0),
            target: target.clone(),
        };

        for coord in walls
        {
            result.x_bound.0 = coord.0.min(result.x_bound.0);
            result.x_bound.1 = coord.0.max(result.x_bound.1);
            result.y_bound.0 = coord.1.min(result.y_bound.0);
            result.y_bound.1 = coord.1.max(result.y_bound.1);
        }
        result.x_bound.1 += 1;
        result.y_bound.1 += 1;

        result.data.resize(result.width() * result.height(), TileType::Unknown);

        for coord in walls
        {
            let index = result.coord2Index(&coord);
            result.data[index] = TileType::Wall;
        }

        for coord in probed
        {
            let index = result.coord2Index(&coord);
            result.data[index] = TileType::Empty;
        }

        let index = result.coord2Index(target);
        result.data[index] = TileType::Empty;
        result
    }

    pub fn diffuseTime(&self) -> usize
    {
        fn airNext(site: &CoordType, field: &Field, this_round: &HashSet<CoordType>,
                   aired: &HashSet<CoordType>) -> bool
        {
            field.at(site) == TileType::Empty && !this_round.contains(site) && !aired.contains(site)
        }

        let mut aired: HashSet<CoordType> = HashSet::new();
        let mut to_check: HashSet<CoordType> = HashSet::new();
        to_check.insert(self.target.clone());
        let mut time: usize = 0;

        let svg_dir = Path::new("frames");
        if !svg_dir.exists()
        {
            fs::create_dir_all(svg_dir).unwrap();
        }

        loop
        {
            let mut svg = fs::File::create(svg_dir.join(format!("{:03}.svg", time))).unwrap();
            write!(svg, "{}", self.drawFieldWithAir(&aired)).unwrap();

            let mut next_checks: HashSet<CoordType> = HashSet::new();
            for source in &to_check
            {
                aired.insert(source.clone());
                if source.0 > self.x_bound.0
                {
                    let maybe = (source.0 - 1, source.1);
                    if airNext(&maybe, self, &to_check, &aired)
                    {
                        next_checks.insert(maybe);
                    }
                }

                if source.1 > self.y_bound.0
                {
                    let maybe = (source.0, source.1 - 1);
                    if airNext(&maybe, self, &to_check, &aired)
                    {
                        next_checks.insert(maybe);
                    }
                }

                if source.0 < self.x_bound.1 - 1
                {
                    let maybe = (source.0 + 1, source.1);
                    if airNext(&maybe, self, &to_check, &aired)
                    {
                        next_checks.insert(maybe);
                    }
                }

                if source.1 < self.y_bound.1 - 1
                {
                    let maybe = (source.0, source.1 + 1);
                    if airNext(&maybe, self, &to_check, &aired)
                    {
                        next_checks.insert(maybe);
                    }
                }
            }

            if next_checks.is_empty()
            {
                return time;
            }
            else
            {
                to_check = next_checks;
                time += 1;
            }
        }
    }
}

fn findPathLength(core: &mut intcode::IntCodeComputer) -> (Field, usize)
{
    let mut path: Vec<CoordType> = Vec::new(); // Doesn’t include current pos.
    let mut pos: CoordType = (0, 0);
    let mut probed: HashSet<CoordType> = HashSet::new();
    let mut walls: HashSet<CoordType> = HashSet::new();
    let mut min_length = usize::max_value();
    #[allow(unused_variables)]
    let mut frame_idx = 0;
    let mut target: Option<CoordType> = None;
    let mut state = BotState::Forward;

    let svg_dir = Path::new("frames");
    if !svg_dir.exists()
    {
        fs::create_dir_all(svg_dir).unwrap();
    }

    loop
    {
        // drawState(&svg_dir.join(format!("{:05}.svg", frame_idx)),
        //           &pos, &walls, &path, &target, &probed);

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
                    let field = Field::from(&walls, &probed, &target.unwrap());
                    return (field, min_length);
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
    findPathLength(&mut computer).1 + 1
}

pub fn part2(input: &str) -> usize
{
    let mut computer = intcode::IntCodeComputer::new();
    let code = intcode::parse(input);
    computer.loadCode(&code);
    let (field, _) = findPathLength(&mut computer);
    field.diffuseTime()
}
