use std::io::{self, prelude::*};
use std::fs::File;
use std::io::Write;

mod wires;

fn main()
{
    let mut input = String::new();
    if io::stdin().lock().read_line(&mut input).is_err()
    {
        panic!("Failed to read input.");
    }

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
    println!("{}", min_dist);

}
