#![allow(non_snake_case)]

use std::vec::Vec;

pub type LengthType = i32;
pub type CoordType = (LengthType, LengthType);

#[derive(PartialEq)]
enum SegmentDirection
{
    Horizontal,
    Vertical,
}

pub fn dist(p1: CoordType, p2: CoordType) -> LengthType
{
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

pub struct Boundary
{
    pub top: LengthType,
    pub right: LengthType,
    pub bottom: LengthType,
    pub left: LengthType,
}

impl Boundary
{
    pub fn new() -> Boundary
    {
        Boundary { top: 0, right: 0, bottom: 0, left: 0 }
    }

    pub fn union(b1: &Boundary, b2: &Boundary) -> Boundary
    {
        Boundary
        {
            top: b1.top.max(b2.top),
            right: b1.right.max(b2.right),
            bottom: b1.bottom.min(b2.bottom),
            left: b1.left.min(b2.left),
        }
    }

    pub fn unionWith(&mut self, other: &Boundary)
    {
        self.top = self.top.max(other.top);
        self.right = self.right.max(other.right);
        self.bottom = self.bottom.min(other.bottom);
        self.left = self.left.min(other.left);
    }

    pub fn width(&self) -> LengthType
    {
        self.right - self.left
    }

    pub fn height(&self) -> LengthType
    {
        self.top - self.bottom
    }

    pub fn svgOpening(&self, scale: f64) -> String
    {
        format!("<svg width=\"{}\" height=\"{}\" viewBox=\"{} {} {} {}\"
xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\">",
                self.width() as f64 * scale, self.height() as f64 * scale,
                self.left, self.bottom, self.width(), self.height())
    }
}

pub struct Segment
{
    pub begin: CoordType,
    end: CoordType,
    direction: SegmentDirection,
}

impl Segment
{
    fn fromStr(begin: CoordType, s: &str) -> Segment
    {
        let dir = s.bytes().nth(0).expect("RIP") as char;
        let dist: LengthType = s[1..].parse().expect(&format!("Failed to parse {}", s)[..]);

        let end: CoordType = match dir
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

    fn boundary(&self) -> Boundary
    {
        Boundary::union(
            &Boundary { top: self.begin.1, right: self.begin.0,
                        bottom: self.begin.1, left: self.begin.0 },
            &Boundary { top: self.end.1, right: self.end.0,
                        bottom: self.end.1, left: self.end.0 })
    }

    pub fn len(&self) -> LengthType
    {
        match self.direction
        {
            SegmentDirection::Horizontal => (self.begin.0 - self.end.0).abs(),
            SegmentDirection::Vertical => (self.begin.1 - self.end.1).abs(),
        }
    }

    pub fn svgSnippet(&self) -> String
    {
        match self.direction
        {
            SegmentDirection::Horizontal => format!("h {}", self.end.0 - self.begin.0),
            SegmentDirection::Vertical => format!("v {}", self.end.1 - self.begin.1),
        }
    }

    pub fn intersect(&self, other: &Segment) -> Option<CoordType>
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
    let start: CoordType = (0, 0);
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

pub fn wire2SVGPath(wire: &Vec<Segment>, color: &str, width: f64) -> String
{
    let attr: String = wire.iter().map(|seg| { seg.svgSnippet() })
        .collect::<Vec<String>>().join(" ");
    format!("<path fill=\"none\" stroke=\"{}\" stroke-width=\"{}\" d=\"M 0 0 {}\" />",
            color, width, attr)
}

pub fn wireBoundary(wire: &Vec<Segment>) -> Boundary
{
    let mut bound = Boundary::new();
    for seg in wire
    {
        bound.unionWith(&seg.boundary());
    }
    bound
}

pub fn svgPoint(point: CoordType, size: f64, color: &str) -> String
{
    format!("<circle cx=\"{}\" cy=\"{}\" r=\"{}\" stroke=\"none\" fill=\"{}\" />",
            point.0, point.1, size, color)
}
