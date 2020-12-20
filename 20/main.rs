use aoc::Grid;
use aoc::GridDrawer;
use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::*;

type Parsed = Vec<(i64, Vec<Vec<char>>)>;
type Answer = i64;

fn get_edge(grid: &Vec<Vec<char>>, dir: aoc::Point) -> Vec<char> {
    let ([_min_x, min_y], [_max_x, max_y]) = grid.extents();
    match dir {
        aoc::NORTH => grid[0].clone(),
        aoc::SOUTH => grid.last().unwrap().clone(),
        aoc::EAST => {
            let mut v = vec![];
            for y in min_y..=max_y {
                v.push(*grid[y as usize].last().unwrap());
            }
            v
        }
        aoc::WEST => {
            let mut v = vec![];
            for y in min_y..=max_y {
                v.push(grid[y as usize][0]);
            }
            v
        }
        _ => panic!(),
    }
}

fn get_matches(input: &Parsed) -> HashMap<i64, Vec<(i64, i64, i64, bool)>> {
    let mut matches: HashMap<i64, Vec<(i64, i64, i64, bool)>> = HashMap::new();
    for i in 0..input.len() {
        let edges = vec![
            get_edge(&input[i].1, aoc::NORTH),
            get_edge(&input[i].1, aoc::EAST),
            get_edge(&input[i].1, aoc::SOUTH),
            get_edge(&input[i].1, aoc::WEST),
        ];
        for j in 0..input.len() {
            if i == j {
                continue;
            }
            let other_edges = vec![
                get_edge(&input[j].1, aoc::NORTH),
                get_edge(&input[j].1, aoc::EAST),
                get_edge(&input[j].1, aoc::SOUTH),
                get_edge(&input[j].1, aoc::WEST),
            ];
            for (di, edge) in edges.iter().enumerate() {
                for (dj, other_edge) in other_edges.iter().enumerate() {
                    let mut other_edge_reversed = other_edge.to_owned();
                    other_edge_reversed.reverse();
                    if *edge == *other_edge {
                        matches
                            .entry(input[i].0)
                            .or_insert(vec![])
                            .push((di as i64, input[j].0, dj as i64, false));
                    } else if *edge == other_edge_reversed {
                        matches
                            .entry(input[i].0)
                            .or_insert(vec![])
                            .push((di as i64, input[j].0, dj as i64, true));
                    }
                }
            }
        }
    }
    matches
}

fn part1(input: &Parsed) -> Answer {
    let matches = get_matches(input);
    let mut ans = 1;
    for (id, b) in matches {
        println!("{}, {:?}", id, b);
        if b.len() == 2 {
            println!("{} is a corner", id);
            ans *= id;
        }
    }
    ans
}

fn find_monsters(
    big_grid: &HashMap<aoc::Point, char>,
    flip_x: i64,
    flip_y: i64,
    rotate: i64,
) -> Vec<aoc::Point> {
    println!("flip_x: {}, flip_y: {}, rotate: {}", flip_x, flip_y, rotate);
    let monster = vec![
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ];
    let mut coords = vec![];
    let ([min_x, min_y], [max_x, max_y]) = big_grid.extents();
    let mut gd = aoc::PrintGridDrawer::new(|c| c);
    for iy in min_y..=max_y {
        let y = if flip_y == -1 { max_y - iy } else { iy };
        'outer: for ix in min_x..=max_x {
            let x = if flip_x == -1 { max_x - ix } else { ix };
            let mut matches = 0;
            let mut monster_coords = vec![];
            for yy in 0..monster.len() {
                for (xx, mc) in monster[yy].chars().enumerate() {
                    if mc == '#' {
                        let xxx = x + xx as i64;
                        let yyy = y + yy as i64;
                        let gc = match rotate {
                            0 => [xxx, yyy],
                            1 => [max_y - yyy, xxx],
                            2 => [max_x - xxx, max_y - yyy],
                            3 => [yyy, max_x - xxx],
                            _ => panic!(),
                        };
                        monster_coords.push(gc);
                    }
                }
            }
            for gc in &monster_coords {
                if let Some(c) = big_grid.get_value(*gc) {
                    if c == '#' {
                        matches += 1;
                    }
                } else {
                    // Monster is outside the picture, skip this coord
                    continue 'outer;
                }
            }
            // gd.draw(&grid);
            //	    println!("matches: {}", matches);
            if matches == 15 {
                println!("found monster at: {}, {}", x, y);
                coords.append(&mut monster_coords);
            }
        }
    }
    coords
}
const IDENTITY: aoc::Mat3 = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];

const ROT90: aoc::Mat3 = [[0, -1, 0], [1, 0, 0], [0, 0, 1]];

const ROT180: aoc::Mat3 = [[-1, 0, 0], [0, -1, 0], [0, 0, 1]];

const ROT270: aoc::Mat3 = [[0, 1, 0], [-1, 0, 0], [0, 0, 1]];

const FLIPX: aoc::Mat3 = [[1, 0, 0], [0, -1, 0], [0, 0, 1]];

const FLIPY: aoc::Mat3 = [[-1, 0, 0], [0, 1, 0], [0, 0, 1]];

fn part2(input: &Parsed) -> Answer {
    let matches = get_matches(input);
    let mut grid_of_grids = HashMap::new();
    let mut queue = VecDeque::new();
    // Find one corner to use as a starting point
    for (id, b) in &matches {
        if b.len() == 2  {
            println!("{}, {:?}", id, b);
            queue.push_back(([0, 0], id, IDENTITY, false, 0));
        }
    }
    let mut seen = HashSet::new();
    while let Some((coord, id, transform, flipped, dj)) = queue.pop_back() {
        if seen.contains(id) {
            continue;
        }
        println!(
            "coord: {:?}, id: {:?}, transform: {:?}",
            coord, id, transform
        );
        grid_of_grids.insert(coord, (id, transform, flipped));
        seen.insert(id);
        if let Some(m) = matches.get(&id) {
            for (di, idj, dj, flippedj) in m {
                let rotj = match di {
                    0 => match dj {
                        0 => ROT180,
                        1 => ROT90,
                        2 => IDENTITY,
                        3 => ROT270,
                        _ => panic!(),
                    },
                    1 => match dj {
                        0 => ROT270,
                        1 => ROT180,
                        2 => ROT90,
                        3 => IDENTITY,
                        _ => panic!(),
                    },
                    2 => match dj {
                        0 => IDENTITY,
                        1 => ROT270,
                        2 => ROT180,
                        3 => ROT90,
                        _ => panic!(),
                    },
                    3 => match dj {
                        0 => ROT90,
                        1 => IDENTITY,
                        2 => ROT270,
                        3 => ROT180,
                        _ => panic!(),
                    },
                    _ => panic!(),
                };
                let dir = match di {
                    0 => aoc::NORTH,
                    1 => aoc::EAST,
                    2 => aoc::SOUTH,
                    3 => aoc::WEST,
                    _ => panic!(),
                };
                let new_coord = aoc::point_add(coord, aoc::row_mat3_transform_vec2(transform, dir));
                let new_id = idj;
                // let flipi = if flipped {
                //     if *di == 0 || *di == 2 {
                //         FLIPY
                //     } else {
                //         FLIPX
                //     }
                // } else {
                //     IDENTITY
                // };
                let flipj = if *flippedj {
                    if *dj == 0 || *dj == 2 {
                        FLIPY
                    } else {
                        FLIPX
                    }
                } else {
                    IDENTITY
                };
                let new_transform = aoc::row_mat3_mul(
                    aoc::row_mat3_mul(flipj, rotj),
                   transform,
                );
                queue.push_back((new_coord, new_id, new_transform, flipped ^ *flippedj, *dj));
            }
        } else {
        }
    }
    println!("{:?}", grid_of_grids);
    let min_x = grid_of_grids.iter().map(|(p, _v)| p[0]).min().unwrap();
    let min_y = grid_of_grids.iter().map(|(p, _v)| p[1]).min().unwrap();
    let max_x = grid_of_grids.iter().map(|(p, _v)| p[0]).max().unwrap();
    let max_y = grid_of_grids.iter().map(|(p, _v)| p[1]).max().unwrap();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if let Some((id, _transform, _flipped)) = grid_of_grids.get(&[x, y]) {
                print!("{}, ", id);
            } else {
                print!("    , ");
            }
        }
        println!();
    }

    let mut big_grid = HashMap::new();
    let mut xxx = 0;
    let mut yyy = 0;
    let mut grids = HashMap::new();
    for (id, g) in input {
        let mut gg = vec![];
        for y in 1..(g.len() - 1) {
            let ggg = g[y][1..(g[y].len() - 1)].to_owned();
            gg.push(ggg);
        }
        grids.insert(id, gg);
    }
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if let Some((id, transform, d)) = grid_of_grids.get(&[x, y]) {
                let g = grids.get(id).unwrap();
                let ([min_xx, min_yy], [max_xx, max_yy]) = g.extents();
		println!("transform: {:?}", transform);
		let mut tr = HashMap::new();
                for yy in min_yy..=max_yy {
                    for xx in min_xx..=max_xx {
                        let xtf = aoc::row_mat3_transform_pos2(*transform, [xx, yy]);
			tr.insert([xx, yy], xtf);
		    }
		}
		let min_xxx = tr.iter().map(|(_, b)| b[0]).min().unwrap();
		let min_yyy = tr.iter().map(|(_, b)| b[1]).min().unwrap();
		let max_xxx = tr.iter().map(|(_, b)| b[0]).max().unwrap();
		let max_yyy = tr.iter().map(|(_, b)| b[1]).max().unwrap();
		println!("{}, {}, {}, {}", min_xxx, min_yyy, max_xxx, max_yyy);
                for yy in min_yy..=max_yy {
                    for xx in min_xx..=max_xx {
			let diff_x = max_xx - max_xxx;
			let diff_y = max_yy - max_yyy;
			println!("diff: {}, {}", diff_x, diff_y);
                        let xtf = aoc::row_mat3_transform_pos2(*transform, [xx, yy]);
			let gc = aoc::point_add(xtf, [diff_x, diff_y]);
			println!("{:?} -> {:?}", [xx, yy], gc);
                        if let Some(v) = g.get_value(gc) {
                            print!("{}", v);
                            big_grid.insert([xxx, yyy], v);
			} else {
			    panic!();
			}
                        xxx += 1;
                    }
                    println!();
                    xxx -= max_xx - min_xx + 1;
                    yyy += 1;
                }
                println!();
                xxx += max_xx - min_xx + 1;
                if x != max_x {
                    yyy -= max_yy - min_yy + 1;
                }
            } else {
                panic!();
            }
        }
        xxx = 0;
    }
    // Find the sea monsters
    let hashes = big_grid.iter().filter(|(_p, v)| **v == '#').count();
    let mut monsters = 0;
    for rot in 0..4 {
        for flip_y in &[-1, 1] {
            for flip_x in &[-1, 1] {
                let m = find_monsters(&big_grid, *flip_x, *flip_y, rot);
                if m.len() > 0 {
                    monsters = m.len();
                }
                println!("m: {:?}", m);
            }
        }
    }
    let mut gd = aoc::PrintGridDrawer::new(|c| c);
    gd.draw(&big_grid);
    (hashes - monsters) as i64
}

fn parse(lines: &[String]) -> Parsed {
    let grids = aoc::split_by_empty_line(lines);
    let mut r = vec![];
    for g in grids {
        let id = g[0][5..(g[0].len() - 1)].parse().unwrap();
        let grid = aoc::parse_grid(&g[1..]);
        r.push((id, grid));
    }
    r
}

fn main() {
    let (part, lines) = aoc::read_lines();
    let parsed = parse(&lines);
    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(&vec![0]), 0);
    // }
}
