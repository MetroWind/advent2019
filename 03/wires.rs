#![allow(non_snake_case)]

use std::vec::Vec;

#[derive(PartialEq)]
enum SegmentDirection
{
    Horizontal,
    Vertical,
}

pub struct Segment
{
    pub begin: (i32, i32),
    end: (i32, i32),
    direction: SegmentDirection,
}

pub fn dist(p1: (i32, i32), p2: (i32, i32)) -> i32
{
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

impl Segment
{
    fn fromStr(begin: (i32, i32), s: &str) -> Segment
    {
        let dir = s.bytes().nth(0).expect("RIP") as char;
        let dist: i32 = s[1..].parse().expect(&format!("Failed to parse {}", s)[..]);

        let end: (i32, i32) = match dir
        {
            'U' => (begin.0, begin.1 + dist),
            'R' => (begin.0 + dist, begin.1),
            'D' => (begin.0, begin.1 - dist),
            'L' => (begin.0 - dist, begin.1),
            _ => panic!("Failed to parse {}", s),
        };

        Segment
        {
            begin: begin,
            end: end,
            direction: if dir == 'U' || dir == 'D'
            { SegmentDirection::Vertical }
            else
            { SegmentDirection::Horizontal },
        }
    }

    pub fn len(&self) -> i32
    {
        match self.direction
        {
            SegmentDirection::Horizontal => (self.begin.0 - self.end.0).abs(),
            SegmentDirection::Vertical => (self.begin.1 - self.end.1).abs(),
        }
    }

    pub fn intersect(&self, other: &Segment) -> Option<(i32, i32)>
    {
        if self.direction == other.direction
        {
            return None;
        }

        match self.direction
        {
            SegmentDirection::Horizontal =>
            {
                if (self.begin.0.min(self.end.0) <= other.begin.0 &&
                    other.begin.0 <= self.begin.0.max(self.end.0)) &&
                    (other.begin.1.min(other.end.1) <= self.begin.1 &&
                     self.begin.1 <= other.begin.1.max(other.end.1))

                {
                    Some((other.begin.0, self.begin.1))
                }
                else
                {
                    None
                }
            },
            SegmentDirection::Vertical =>
            {
                if (self.begin.1.min(self.end.1) <= other.begin.1 &&
                    other.begin.1 <= self.begin.1.max(self.end.1)) &&
                    (other.begin.0.min(other.end.0) <= self.begin.0 &&
                     self.begin.0 <= other.begin.0.max(other.end.0))
                {
                    Some((self.begin.0, other.begin.1))
                }
                else
                {
                    None
                }
            },
        }
    }
}

pub fn parseWire(s: &str) -> Vec<Segment>
{
    let start: (i32, i32) = (0, 0);
    let mut current = start.clone();
    let mut segments: Vec<Segment> = Vec::new();

    for part in s.trim().split(',')
    {
        let seg = Segment::fromStr(current, &part[..]);
        current = seg.end.clone();
        segments.push(seg);
    }
    segments
}
