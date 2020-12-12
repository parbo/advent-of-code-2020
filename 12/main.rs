use aoc::*;
use std::iter::*;
use std::collections::HashMap;

fn draw(orig_path: &[(Point, Point)], s: &str, scale: i64) {
    let mut minx = 0;
    let mut miny = 0;
    let mut maxx = 0;
    let mut maxy = 0;
    // scale it
    let mut scaled_path = vec![];
    for (ship, wp) in orig_path {
        scaled_path.push((
            [ship[0] / scale, ship[1] / scale],
            [wp[0] / scale, wp[1] / scale],
        ));
    }
    // find min/max
    for (ship, wp) in &scaled_path {
        minx = ship[0].min(minx);
        miny = ship[1].min(miny);
        maxx = ship[0].max(maxx);
        maxy = ship[1].max(maxy);
        let p = point_add(*ship, *wp);
        minx = p[0].min(minx);
        miny = p[1].min(miny);
        maxx = p[0].max(maxx);
        maxy = p[1].max(maxy);
    }
    // extend paths by connecting with lines
    let mut path = vec![];
    for i in 0..(scaled_path.len() - 1) {
        let (s0, w0) = scaled_path[i];
        let (s1, w1) = scaled_path[i + 1];
        let s_line = plot_line(s0, s1);
        let w_line = plot_line(w0, w1);
        let ms = s_line.len().max(w_line.len());
        for j in 0..ms {
            let s = if j < s_line.len() {
                s_line[j]
            } else {
                s_line[s_line.len() - 1]
            };
            let w = if j < w_line.len() {
                w_line[j]
            } else {
                w_line[w_line.len() - 1]
            };
            path.push((s, w));
        }
    }
    let mut g = HashMap::new();
    g.insert([minx, miny], '.');
    g.insert([maxx, maxy], '.');
    let mut gd = aoc::BitmapGridDrawer::new(
        (1, 1),
        |x| match x {
            '#' => vec![(255, 0, 0)],
            'L' => vec![(0, 0, 255)],
            '*' => vec![(40, 0, 0)],
            '+' => vec![(0, 0, 40)],
            _ => vec![(255, 255, 255)],
        },
        s,
    );
    let f = path.len();
    println!("{} frames", path.len());
    for (i, (ship, wp)) in path.iter().enumerate() {
        let p = point_add(*ship, *wp);
        *g.entry(p).or_insert('L') = 'L';
        *g.entry(*ship).or_insert('#') = '#';
        // Draw max 1000 frames
        if (f > 1000 && i % (f / 1000) == 0) || i + 1 == f {
            // Center rect around ship
            let r = (
                [ship[0] - 500, ship[1] - 500],
                [ship[0] + 500, ship[1] + 500],
            );
            gd.set_rect(r);
            gd.draw(&g);
        }
        *g.entry(p).or_insert('+') = '+';
        *g.entry(*ship).or_insert('*') = '*';
    }
}

fn part1(moves: &[(char, i64)], d: bool) -> i64 {
    let mut curr = [0, 0];
    let mut facing = EAST;
    let mut path = vec![(curr, facing)];
    for (d, steps) in moves {
        match d {
            'E' => curr = point_add(curr, point_mul(EAST, *steps)),
            'W' => curr = point_add(curr, point_mul(WEST, *steps)),
            'N' => curr = point_add(curr, point_mul(NORTH, *steps)),
            'S' => curr = point_add(curr, point_mul(SOUTH, *steps)),
            'F' => curr = point_add(curr, point_mul(facing, *steps)),
            'L' => {
                for _ in (0..*steps).step_by(90) {
                    facing = [facing[1], -facing[0]]
                }
            }
            'R' => {
                for _ in (0..*steps).step_by(90) {
                    facing = [-facing[1], facing[0]]
                }
            }
            _ => panic!(),
        }
        path.push((curr, facing));
    }
    if d {
	draw(&path, "ppm/day12/part1", 1);
    }
    curr[0].abs() + curr[1].abs()
}

fn part2(moves: &[(char, i64)], d: bool) -> i64 {
    let mut waypoint = [10, -1];
    let mut ship = [0, 0];
    let mut path = vec![(ship, waypoint)];
    for (d, steps) in moves {
        match d {
            'E' => waypoint = point_add(waypoint, point_mul(EAST, *steps)),
            'W' => waypoint = point_add(waypoint, point_mul(WEST, *steps)),
            'N' => waypoint = point_add(waypoint, point_mul(NORTH, *steps)),
            'S' => waypoint = point_add(waypoint, point_mul(SOUTH, *steps)),
            'F' => ship = point_add(ship, point_mul(waypoint, *steps)),
            'L' => {
                for _ in (0..*steps).step_by(90) {
                    waypoint = [waypoint[1], -waypoint[0]]
                }
            }
            'R' => {
                for _ in (0..*steps).step_by(90) {
                    waypoint = [-waypoint[1], waypoint[0]]
                }
            }
            _ => panic!(),
        }
        path.push((ship, waypoint));
    }
    if d {
	draw(&path, "ppm/day12/part2", 16);
    }
    ship[0].abs() + ship[1].abs()
}

fn parse(lines: &[String]) -> Vec<(char, i64)> {
    lines
        .iter()
        .map(|x| (x.chars().next().unwrap(), x[1..].parse().unwrap()))
        .collect()
}

fn main() {
    let (part, lines) = aoc::read_lines();
    let parsed = parse(&lines);
    let result = if part == 1 {
        part1(&parsed, true)
    } else {
        part2(&parsed, true)
    };
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&vec![('F', 10), ('N', 3), ('F', 7), ('R', 90), ('F', 11)], false),
            286
        );
    }
}
