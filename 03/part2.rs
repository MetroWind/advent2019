use std::io::{self, prelude::*};

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

    let mut min_dist = i32::max_value();

    let mut dist1: i32 = 0;
    let mut dist2: i32 = 0;

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

    println!("{}", min_dist);
}
