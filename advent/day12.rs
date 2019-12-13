use std::fmt;
use std::io::{self, Write};

use regex::Regex;

use crate::vector::Vec3;
use crate::ratio;

#[derive(Copy, Clone, fmt::Debug)]
struct Moon
{
    pub x: Vec3<i32>,
    pub v: Vec3<i32>,
}

impl Moon
{
    pub fn new() -> Self
    {
        Self
        {
            x: Vec3::new(0, 0, 0),
            v: Vec3::new(0, 0, 0),
        }
    }

    pub fn fromInputLine(line: &str) -> Result<Self, String>
    {
        let mut moon = Self::new();

        let pattern = Regex::new(r"^<x=(.+), y=(.+), z=(.+)>$").unwrap();
        let groups = if let Some(capture) = pattern.captures(line)
        {
            capture
        }
        else
        {
            return Err(format!("Invalid line: {}", line));
        };

        for i in 0..moon.x.len()
        {
            let catched_str = groups.get(i + 1).unwrap().as_str();
            moon.x[i] = if let Ok(x) = catched_str.parse()
            {
                x
            }
            else
            {
                return Err(format!("Invalid number: {}", catched_str));
            }
        }

        Ok(moon)
    }

    pub fn pullByDirection(&mut self, other: &Moon, i: usize)
    {
        if other.x[i] > self.x[i]
        {
            self.v[i] += 1;
        }
        else if other.x[i] < self.x[i]
        {
            self.v[i] -= 1;
        }
    }

    pub fn pullBy(&mut self, other: &Moon)
    {
        for i in 0..3
        {
            self.pullByDirection(other, i);
        }
    }

    pub fn updatePosition(&mut self)
    {
        self.x = self.x + self.v;
    }

    pub fn energy(&self) -> i32
    {
        self.x.iter().map(|x| x.abs()).sum::<i32>() *
            self.v.iter().map(|x| x.abs()).sum::<i32>()
    }

    pub fn eqDirection(&self, rhs: &Moon, dir_idx: usize) -> bool
    {
        self.x[dir_idx] == rhs.x[dir_idx] && self.v[dir_idx] == rhs.v[dir_idx]
    }
}

impl PartialEq for Moon
{
    fn eq(&self, rhs: &Self) -> bool
    {
        self.x == rhs.x && self.v == rhs.v
    }
}

#[derive(Clone, fmt::Debug)]
struct Universe
{
    bodies: [Moon; 4],
}

impl Universe
{
    pub fn fromInput(input: &str) -> Result<Self, String>
    {
        let mut result = Self
        {
            bodies: [Moon::new(), Moon::new(), Moon::new(), Moon::new()],
        };

        let mut lines = input.lines();
        for body in &mut result.bodies
        {
            if let Some(line) = lines.next()
            {
                *body = Moon::fromInputLine(line)?;
            }
            else
            {
                return Err(String::from("Failed to get line"));
            }
        }

        Ok(result)
    }

    pub fn step(&mut self)
    {
        let parallel_universe = self.clone();
        for i in 0..self.bodies.len()
        {
            for j in 0..self.bodies.len()
            {
                if i != j
                {
                    self.bodies[i].pullBy(&parallel_universe.bodies[j]);
                }
            }
        }
        for i in 0..self.bodies.len()
        {
            self.bodies[i].updatePosition();
        }
    }

    pub fn stepDirection(&mut self, dir_idx: usize)
    {
        let parallel_universe = self.clone();
        for i in 0..self.bodies.len()
        {
            for j in 0..self.bodies.len()
            {
                if i != j
                {
                    self.bodies[i].pullByDirection(
                        &parallel_universe.bodies[j], dir_idx);
                }
            }
        }
        for i in 0..self.bodies.len()
        {
            self.bodies[i].updatePosition();
        }
    }

    pub fn energy(&self) -> i32
    {
        self.bodies.iter().map(|body| body.energy()).sum()
    }

    pub fn eqDirection(&self, rhs: &Universe, dir_idx: usize) -> bool
    {
        let mut result = true;
        for i in 0..self.bodies.len()
        {
            result = result && self.bodies[i].eqDirection(&rhs.bodies[i], dir_idx);
        }
        result
    }
}

impl PartialEq for Universe
{
    fn eq(&self, rhs: &Self) -> bool
    {
        self.bodies == rhs.bodies
    }
}

pub fn part1(input: &str) -> i32
{
    let mut universe = Universe::fromInput(input).unwrap();
    for _ in 0..1000
    {
        universe.step();
    }
    universe.energy()
}

fn findPeriodDirection(initial: &Universe, dir_idx: usize) -> u64
{
    let mut universe = initial.clone();
    let mut steps: u64 = 0;

    loop
    {
        io::stdout().flush().unwrap();

        universe.stepDirection(dir_idx);
        steps += 1;
        if universe.eqDirection(initial, dir_idx)
        {
            println!("Did {} steps.", steps);
            return steps;
        }
    }
}

fn findPeriod(initial: &Universe) -> u64
{
    let steps = ratio::lcm(findPeriodDirection(initial, 0),
                               findPeriodDirection(initial, 1));
    ratio::lcm(steps, findPeriodDirection(initial, 2))
}

#[allow(dead_code)]
fn findPeriodBrutal(initial: &Universe) -> u64
{
    let mut universe = initial.clone();
    let mut steps: u64 = 0;

    loop
    {
        if steps % 10000000 == 0
        {
            print!(".");
        }
        io::stdout().flush().unwrap();

        universe.step();
        steps += 1;
        if &universe == initial
        {
            return steps;
        }
    }
}

pub fn part2(input: &str) -> u64
{
    let universe = Universe::fromInput(input).unwrap();
    findPeriod(&universe)
}

#[test]
fn testPart1()
{
    let mut universe = Universe::fromInput("<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>").unwrap();
    for _ in 0..10
    {
        universe.step();
    }
    assert_eq!(universe.energy(), 179);
}

#[test]
fn testPart2()
{
    {
        let universe = Universe::fromInput("<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>").unwrap();
        assert_eq!(findPeriodBrutal(&universe), 2772);
        assert_eq!(findPeriod(&universe), 2772);
    }

    {
        let universe = Universe::fromInput("<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>").unwrap();
        assert_eq!(findPeriod(&universe), 4686774924);
        // assert_eq!(findPeriodBrutal(&universe), 4686774924);
    }
}
