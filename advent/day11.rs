use std::vec::Vec;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;

use crate::intcode::intcode;

#[derive(Copy, Clone)]
enum PaintColor
{
    Black = 0,
    White = 1,
}

impl PaintColor
{
    fn from(color: intcode::ValueType) -> Result<Self, String>
    {
        match color
        {
            0 => Ok(Self::Black),
            1 => Ok(Self::White),
            _ => Err(format!("Unknown color: {}", color)),
        }
    }
}

// Using screen coord, +x is right, +y is down.
#[derive(Copy, Clone)]
enum Direction
{
    Up, Right, Down, Left,
}

#[derive(Copy, Clone)]
enum Turn
{
    Left, Right,
}

impl Turn
{
    fn from(turn: intcode::ValueType) -> Result<Self, String>
    {
        match turn
        {
            0 => Ok(Self::Left),
            1 => Ok(Self::Right),
            _ => Err(format!("Unknow turn direction: {}", turn)),
        }
    }
}

impl Direction
{
    pub fn turn_with_code(&self, dir: intcode::ValueType) -> Result<Self, String>
    {
        let turn_dir = Turn::from(dir)?;
        Ok(
        match self
        {
            Self::Up => match turn_dir
            { Turn::Left => Self::Left, Turn::Right => Self::Right, },
            Self::Right => match turn_dir
            { Turn::Left => Self::Up, Turn::Right => Self::Down, },
            Self::Down => match turn_dir
            { Turn::Left => Self::Right, Turn::Right => Self::Left, },
            Self::Left => match turn_dir
            { Turn::Left => Self::Down, Turn::Right => Self::Up, },
        })
    }
}

struct PaintBot
{
    pub canvas: Vec<PaintColor>,
    location: (usize, usize),
    core: intcode::IntCodeComputer,
    direction: Direction,
    pub painted: HashSet<(usize, usize)>,
}

impl PaintBot
{
    pub const WIDTH: usize = 100;
    pub const HEIGHT: usize = 100;

    pub fn withCode(code: &Vec<intcode::ValueType>) -> Self
    {
        let mut result = Self
        {
            canvas: vec![PaintColor::Black; Self::WIDTH * Self::HEIGHT],
            location: (Self::WIDTH / 2, Self::HEIGHT / 2),
            core: intcode::IntCodeComputer::new(),
            direction: Direction::Up,
            painted: HashSet::new(),
        };
        result.core.loadCode(code);
        result
    }

    fn coord2Index(coord: &(usize, usize)) -> usize
    {
        coord.1 * Self::WIDTH + coord.0
    }

    pub fn index2Coord(index: usize) -> (usize, usize)
    {
        (index % Self::WIDTH, index / Self::WIDTH)
    }

    fn step<F, R>(&mut self, do_what: &mut F) -> Result<(), String>
        where F: FnMut(&(usize, usize), &PaintColor) -> R
    {
        let current_color = self.canvas[Self::coord2Index(&self.location)]
            as intcode::ValueType;
        let new_color_code = match self.core.pipe(Some(current_color))
        {
            Some(output) => output,
            None => { return Ok(()); },
        };

        let new_color = PaintColor::from(new_color_code)?;
        self.canvas[Self::coord2Index(&self.location)] = new_color;
        self.painted.insert(self.location);
        do_what(&self.location, &new_color);

        let direction_code = match self.core.pipe(None)
        {
            Some(output) => output,
            None => { return Ok(()); },
        };

        self.direction = self.direction.turn_with_code(direction_code)?;

        match self.direction
        {
            Direction::Up =>
            {
                self.location = (self.location.0, self.location.1 - 1);
            },
            Direction::Right =>
            {
                self.location = (self.location.0 + 1, self.location.1);
            },
            Direction::Down =>
            {
                self.location = (self.location.0, self.location.1 + 1);
            },
            Direction::Left =>
            {
                self.location = (self.location.0 - 1, self.location.1);
            },
        }
        Ok(())
    }

    pub fn run<F, R>(&mut self, mut do_what: F) -> Result<(), String>
        where F: FnMut(&(usize, usize), &PaintColor) -> R
    {
        while !self.core.halted()
        {
            self.step(&mut do_what)?;
        }
        Ok(())
    }
}

#[allow(unused_must_use)]
fn drawSVG(filename: &str, canvas: &Vec<PaintColor>)
{
    let svg_cell_size = 4;

    let mut svgfile = File::create(filename).expect("Failed to open SVG file");
    writeln!(svgfile, r#"<svg viewBox="{} {} {} {}" xmlns="http://www.w3.org/2000/svg">"#,
             0, 0, PaintBot::WIDTH * svg_cell_size,
             PaintBot::HEIGHT * svg_cell_size);

    for i in 0..canvas.len()
    {
        let coord = PaintBot::index2Coord(i);
        writeln!(svgfile, r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" stroke="none" />"#,
                 coord.0 * svg_cell_size, coord.1 * svg_cell_size,
                 svg_cell_size, svg_cell_size,
                 match canvas[i]
                 {
                     PaintColor::Black => "black",
                     PaintColor::White => "white",
                 });
    }
    writeln!(svgfile, "</svg>");
}

pub fn part1(input: &str) -> usize
{
    let code = intcode::parse(input);
    let mut bot = PaintBot::withCode(&code);
    bot.run(|_, _| {}).expect("Failed");
    drawSVG("11-1.svg", &bot.canvas);
    bot.painted.len()
}

pub fn part2(input: &str) -> i32
{
    let code = intcode::parse(input);
    let mut bot = PaintBot::withCode(&code);
    bot.canvas[PaintBot::coord2Index(&bot.location)] = PaintColor::White;
    bot.run(|_, _| {}).expect("Failed");
    drawSVG("11-2.svg", &bot.canvas);
    0
}
