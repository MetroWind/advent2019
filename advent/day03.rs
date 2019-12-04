use std::io::{self, prelude::*};
use std::fs::File;
use std::io::Write;

mod wires;
mod advent;

struct Day03 {}
impl advent::Solution for Day03
{
    fn part1(&self, input: &str) -> String
    {
        let segments1 = wires::parseWire(&input[..]);

        input.clear();
        if io::stdin().lock().read_line(&mut input).is_err()
        {
            panic!("Failed to read input.");
        }
        let segments2 = wires::parseWire(&input[..]);

        // Generate SVG
        let path = "wires.svg";
        let bound = wires::Boundary::union(&wires::wireBoundary(&segments1),
                                         &wires::wireBoundary(&segments2));

        let mut file = File::create(path).expect("Failed to write file.");

        file.write_all(format!("{}\n", bound.expanded(50).svgOpening(0.05)).as_bytes());
        file.write_all(format!("{}\n", wires::wire2SVGPath(&segments1, "#3742fa", 20.0)).as_bytes());
        file.write_all(format!("{}\n", wires::wire2SVGPath(&segments2, "#ff4757", 20.0)).as_bytes());

        // Find intersections
        let mut min_dist = wires::LengthType::max_value();
        for seg_from_1 in &segments1
        {
            for seg_from_2 in &segments2
            {
                if let Some(point) = seg_from_1.intersect(&seg_from_2)
                {
                    let new_dist = wires::dist((0, 0), point);
                    if new_dist > 0 && new_dist < min_dist
                    {
                        min_dist = new_dist;
                    }
                    file.write_all(format!("{}\n", wires::svgPoint(point, 35.0, "#2f3542")).as_bytes());
                }
            }
        }

        file.write_all(wires::svgOrigin(50.0, 20.0, "#2f3542").as_bytes());
        file.write_all(format!("</svg>\n").as_bytes());

        min_dist.to_string()
    }

    fn part1(&self, input: &str) -> String
    {
        let segments1 = wires::parseWire(&input[..]);

        input.clear();
        if io::stdin().lock().read_line(&mut input).is_err()
        {
            panic!("Failed to read input.");
        }
        let segments2 = wires::parseWire(&input[..]);

        let mut min_dist = wires::LengthType::max_value();

        let mut dist1: wires::LengthType = 0;
        let mut dist2: wires::LengthType = 0;

        for seg_from_1 in &segments1
        {
            for seg_from_2 in &segments2
            {
                if let Some(point) = seg_from_1.intersect(&seg_from_2)
                {
                    let new_dist = dist1 + dist2 + wires::dist(seg_from_1.begin, point)
                        + wires::dist(seg_from_2.begin, point);
                    if new_dist > 0 && new_dist < min_dist
                    {
                        min_dist = new_dist;
                    }
                }
                dist2 += seg_from_2.len();
            }
            dist2 = 0;
            dist1 += seg_from_1.len();
        }

        min_dist.to_string()
    }
}
