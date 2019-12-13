use std::vec::Vec;
use std::collections::HashSet;
use std::cmp::Ordering;
use std::fs::File;
use std::io::Write;

use crate::ratio::Ratio;

// enum SpacePoint
// {
//     Asteroid,
//     Void
// }

// impl SpacePoint
// {
//     fn from(c: u8) -> Result(Self, String)
//     {
//         match c
//         {
//             b'.' => Ok(Self::Void),
//             b'#' => Ok(Self::Asteroid),
//             _ => Err(format!("WTF is a '{}'?", c)),
//         }
//     }
// }

struct SpaceInfo
{
    width: i32,
    height: i32,
    asteroids: HashSet<(i32, i32)>,
}

fn parse(input: &str) -> SpaceInfo
{
    let mut result = HashSet::new();
    let mut row: i32 = 0;
    let mut col: i32 = 0;

    let mut width = 0;

    for line in input.lines()
    {
        for byte in line.bytes()
        {
            if byte == b'#'
            {
                result.insert((col, row));
            }
            col += 1;
        }
        row += 1;
        width = col;
        col = 0;
    }

    SpaceInfo
    {
        width: width,
        height: row,
        asteroids: result,
    }
}

fn dist(a: &(i32, i32), b: &(i32, i32)) -> i32
{
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn calcSlope(from: &(i32, i32), to: &(i32, i32)) -> Result<Ratio<u32>, String>
{
    Ratio::from((to.1 - from.1).abs() as u32, (to.0 - from.0).abs() as u32)
}

fn findLocation(asteroids: &HashSet<(i32, i32)>) -> ((i32, i32), i32)
{
    let mut max_count = 0;
    let mut loc = (0, 0);

    for from in asteroids
    {
        let mut blocked_up = false;
        let mut blocked_right = false;
        let mut blocked_down = false;
        let mut blocked_left = false;

        let mut count = 0;

        let mut slopes1 = HashSet::new();
        let mut slopes2 = HashSet::new();
        let mut slopes3 = HashSet::new();
        let mut slopes4 = HashSet::new();

        for to in asteroids
        {
            if from == to
            {
                continue;
            }

            if from.0 == to.0
            {
                if to.1 < from.1 && !blocked_up
                {
                    blocked_up = true;
                    count += 1;
                }
                else if to.1 > from.1 && !blocked_down
                {
                    blocked_down = true;
                    count += 1;
                }
            }
            else if from.1 == to.1
            {
                if to.0 > from.0 && !blocked_right
                {
                    blocked_right = true;
                    count += 1;
                }
                else if to.0 < from.0 && !blocked_left
                {
                    blocked_left = true;
                    count += 1;
                }
            }
            else
            {
                let slope = calcSlope(from, to).unwrap();
                if to.0 > from.0
                {
                    if to.1 > from.1
                    {
                        if !slopes1.contains(&slope)
                        {
                            slopes1.insert(slope);
                            count += 1;
                        }
                    }
                    else
                    {
                        if !slopes4.contains(&slope)
                        {
                            slopes4.insert(slope);
                            count += 1;
                        }
                    }
                }
                else
                {
                    if to.1 > from.1
                    {
                        if !slopes2.contains(&slope)
                        {
                            slopes2.insert(slope);
                            count += 1;
                        }
                    }
                    else
                    {
                        if !slopes3.contains(&slope)
                        {
                            slopes3.insert(slope);
                            count += 1;
                        }
                    }
                }
            }
        }
        if count > max_count
        {
            max_count = count;
            loc = from.clone();
        }
    }
    (loc, max_count)
}

fn shootBlock<F, R>(from: &(i32, i32), mut block: Vec<Option<(i32, i32)>>, do_what: &mut F) -> Vec<Option<(i32, i32)>>
where F: FnMut(&(i32, i32)) -> R
{
    let mut size = block.len();
    let mut shot_slope: HashSet<Ratio<u32>> = HashSet::new();
    for target_maybe in &mut block
    {
        if target_maybe.is_none()
        {
            continue;
        }

        let target = target_maybe.unwrap();
        let slope = calcSlope(from, &target).unwrap();
        if !shot_slope.contains(&slope)
        {
            do_what(&target);
            shot_slope.insert(slope);
            *target_maybe = None;
        }
    }

    let mut i: usize = 0;
    while i < size
    {
        if block[i].is_none()
        {
            block.remove(i);
            size -= 1;
        }
        else
        {
            i += 1;
        }
    }
    block
}

fn shoot<F, R>(from: &(i32, i32), asteroids: &HashSet<(i32, i32)>, mut do_what: F)
where F: FnMut(&(i32, i32)) -> R
{
    let mut up: Vec<(i32, i32)> = Vec::new();
    let mut right: Vec<(i32, i32)> = Vec::new();
    let mut down: Vec<(i32, i32)> = Vec::new();
    let mut left: Vec<(i32, i32)> = Vec::new();

    let mut block1: Vec<Option<(i32, i32)>> = Vec::new();
    let mut block2: Vec<Option<(i32, i32)>> = Vec::new();
    let mut block3: Vec<Option<(i32, i32)>> = Vec::new();
    let mut block4: Vec<Option<(i32, i32)>> = Vec::new();

    for asteroid in asteroids
    {
        if asteroid == from
        {
            continue;
        }

        if asteroid.0 == from.0
        {
            if asteroid.1 > from.1
            {
                down.push(asteroid.clone());
            }
            else
            {
                up.push(asteroid.clone());
            }
        }
        else if asteroid.1 == from.1
        {
            if asteroid.0 > from.0
            {
                right.push(asteroid.clone());
            }
            else
            {
                left.push(asteroid.clone());
            }
        }
        else
        {
            if asteroid.0 > from.0
            {
                if asteroid.1 > from.1
                {
                    block1.push(Some(asteroid.clone()));
                }
                else
                {
                    block4.push(Some(asteroid.clone()));
                }
            }
            else
            {
                if asteroid.1 > from.1
                {
                    block2.push(Some(asteroid.clone()));
                }
                else
                {
                    block3.push(Some(asteroid.clone()));
                }
            }
        }
    }

    fn key1(from: &(i32, i32), a: &(i32, i32), b: &(i32, i32)) -> Ordering
    {
        let slope1 = calcSlope(from, a).unwrap();
        let slope2 = calcSlope(from, b).unwrap();
        (slope1, dist(a, from)).partial_cmp(&(slope2, dist(b, from))).unwrap()
    }

    fn key2(from: &(i32, i32), a: &(i32, i32), b: &(i32, i32)) -> Ordering
    {
        let slope1 = calcSlope(from, a).unwrap();
        let slope2 = calcSlope(from, b).unwrap();
        (slope2, dist(a, from)).partial_cmp(&(slope1, dist(b, from))).unwrap()
    }

    up.sort_unstable_by(|a, b| { dist(a, from).cmp(&dist(b, from)).reverse() });
    right.sort_unstable_by(|a, b| { dist(a, from).cmp(&dist(b, from)).reverse() });
    down.sort_unstable_by(|a, b| { dist(a, from).cmp(&dist(b, from)).reverse() });
    left.sort_unstable_by(|a, b| { dist(a, from).cmp(&dist(b, from)).reverse() });

    block1.sort_unstable_by(|a, b| key1(from, &a.unwrap(), &b.unwrap()));
    block2.sort_unstable_by(|a, b| key2(from, &a.unwrap(), &b.unwrap()));
    block3.sort_unstable_by(|a, b| key1(from, &a.unwrap(), &b.unwrap()));
    block4.sort_unstable_by(|a, b| key2(from, &a.unwrap(), &b.unwrap()));

    loop
    {
        let mut did_stuff = false;

        if !up.is_empty()
        {
            do_what(&up.pop().unwrap());
            did_stuff = true;
        }

        if !block4.is_empty()
        {
            block4 = shootBlock(from, block4, &mut do_what);
            did_stuff = true;
        }

        if !right.is_empty()
        {
            do_what(&right.pop().unwrap());
            did_stuff = true;
        }

        if !block1.is_empty()
        {
            block1 = shootBlock(from, block1, &mut do_what);
            did_stuff = true;
        }

        if !down.is_empty()
        {
            do_what(&down.pop().unwrap());
            did_stuff = true;
        }

        if !block2.is_empty()
        {
            block2 = shootBlock(from, block2, &mut do_what);
            did_stuff = true;
        }

        if !left.is_empty()
        {
            do_what(&left.pop().unwrap());
            did_stuff = true;
        }

        if !block3.is_empty()
        {
            block3 = shootBlock(from, block3, &mut do_what);
            did_stuff = true;
        }

        if !did_stuff
        {
            break;
        }
    }
}

pub fn part1(input: &str) -> i32
{
    let asteroids = parse(input).asteroids;
    let (_, max_count) = findLocation(&asteroids);
    max_count
}

#[allow(unused_must_use)]
pub fn part2(input: &str) -> i32
{
    let svg_cell_size: i32 = 32;
    let svg_padding: i32 = 4;
    let space = parse(input);
    let (from, _) = findLocation(&space.asteroids);

    let svg_fg = "#ffffff";
    let svg_bg = "#2f3542";

    let mut svgfile = File::create("10-2.svg").expect("Failed to open SVG file");
    writeln!(svgfile, r#"<svg viewBox="{} {} {} {}" xmlns="http://www.w3.org/2000/svg">"#,
             -svg_padding, -svg_padding,
             space.width * svg_cell_size + 2 * svg_padding,
             space.height * svg_cell_size + 2 * svg_padding);
    writeln!(svgfile, r##"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" />"##,
             -svg_padding, -svg_padding,
             space.width * svg_cell_size + 2 * svg_padding,
             space.height * svg_cell_size + 2 * svg_padding,
             svg_bg);
    writeln!(svgfile, r##"<rect x="{x}" y="{y}" width="{size}" height="{size}" fill="#2ed573" stroke="none" />"##,
             x=from.0 * svg_cell_size, y=from.1 * svg_cell_size, size=svg_cell_size);

    for asteroid in &space.asteroids
    {
        if asteroid != &from
        {
            writeln!(svgfile, r##"<rect x="{x}" y="{y}" width="{size}" height="{size}" fill="#ff4757" stroke="none" />"##,
                     x=asteroid.0 * svg_cell_size, y=asteroid.1 * svg_cell_size,
                     size=svg_cell_size);
        }
    }

    let mut the200: (i32, i32) = (0, 0);
    let mut count = 0;
    shoot(&from, &space.asteroids,
          |target|
          {
              count += 1;
              if count == 200
              {
                  the200 = target.clone();
              }

              writeln!(svgfile, r#"<text x="{x}" y="{y}" text-anchor="middle" dominant-baseline="middle" fill="{color}" font-family="IBM Plex Mono" font-weight="bold" font-size="13">{text}</text>"#,
                       x=target.0 * svg_cell_size + svg_cell_size / 2,
                       y=target.1 * svg_cell_size + svg_cell_size / 2,
                       color=svg_fg,
                       text=count);

          });

    writeln!(svgfile, r#"<rect x="0" y="0" width="{}" height="{}" stroke="{}" fill="none" stroke-width="2" />"#,
             space.width * svg_cell_size, space.height * svg_cell_size, svg_fg);

    for row in 1..space.height
    {
        writeln!(svgfile, r#"<line x1="0" x2="{x}" y1="{y}" y2="{y}"
stroke="{color}" stroke-width="2"/>"#,
                 x=space.width * svg_cell_size, y=row * svg_cell_size, color=svg_fg);
    }

    for col in 1..space.width
    {
        writeln!(svgfile, r#"<line x1="{x}" x2="{x}" y1="0" y2="{y}" stroke="{color}" stroke-width="2"/>"#,
                 x=col * svg_cell_size, y=space.height * svg_cell_size, color=svg_fg);
    }

    writeln!(svgfile, "</svg>");
    the200.0 * 100 + the200.1
}

#[test]
fn testPart1()
{
    assert_eq!(part1(".#..#
.....
#####
....#
...##"), 8);

    assert_eq!(part1("......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####"), 33);

    assert_eq!(part1("#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.
"), 35);

    assert_eq!(part1(".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#.."), 41);


    assert_eq!(part1(".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
"), 210);

}

#[test]
fn testPart2()
{
    let space = parse(".#..#
.....
#####
....#
...##");
    let total = space.asteroids.len();
    let (from, _) = findLocation(&space.asteroids);

    let mut count: usize = 0;
    shoot(&from, &space.asteroids, |_| { count += 1; });
    assert_eq!(count, total - 1);
}
