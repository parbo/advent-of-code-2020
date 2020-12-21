use aoc::Grid;
use aoc::GridDrawer;
use std::collections::{HashMap, HashSet};
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
        for e in &edges {
            let mut ee = e.to_owned();
            ee.reverse();
            println!("{:?}", e);
            println!("{:?}", ee);
        }
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

fn place(
    grids: &HashMap<i64, Vec<Vec<char>>>,
    coord: aoc::Point,
    placed: HashSet<i64>,
    grid_of_grids: HashMap<aoc::Point, (i64, Vec<Vec<char>>)>,
) -> HashMap<aoc::Point, (i64, Vec<Vec<char>>)> {
    if grid_of_grids.len() == grids.len() {
        return grid_of_grids;
    }
    let expected = (grids.len() as f64).sqrt() as i64;
    // println!("== gog ==");
    // for y in 0..expected {
    // 	for x in 0..expected {
    // 	    if let Some((id, _)) = grid_of_grids.get(&[x, y]) {
    // 		print!("{}, ", id);
    // 	    } else {
    // 		print!("    , ");
    // 	    }
    // 	}
    // 	println!();
    // }
    // println!("coord: {:?}", coord);
    let mut candidates = vec![];
    for (id, g) in grids {
        if placed.contains(&id) {
            continue;
        }
        // Try to make it fit with the neighbors
        for rot in 0..4 {
            let mut gg = g.clone();
            match rot {
                0 => {}
                1 => gg.rotate_90_cw(),
                2 => gg.rotate_180_cw(),
                3 => gg.rotate_270_cw(),
                _ => panic!(),
            }
            for flip_x in 0..2 {
                let mut ggg = gg.clone();
                match flip_x {
                    0 => {}
                    1 => ggg.flip_horizontal(),
                    _ => panic!(),
                }
                'outer: for flip_y in 0..2 {
                    let mut gggg = ggg.clone();
                    match flip_y {
                        0 => {}
                        1 => gggg.flip_vertical(),
                        _ => panic!(),
                    }
                    for d in aoc::DIRECTIONS.clone() {
                        let c = aoc::point_add(coord, d);
                        if let Some((id, g)) = grid_of_grids.get(&c) {
                            let e = get_edge(&gggg, d);
                            let other_e = match d {
                                aoc::NORTH => get_edge(g, aoc::SOUTH),
                                aoc::EAST => get_edge(g, aoc::WEST),
                                aoc::SOUTH => get_edge(g, aoc::NORTH),
                                aoc::WEST => get_edge(g, aoc::EAST),
                                _ => panic!(),
                            };
                            if e != other_e {
                                continue 'outer;
                            }
                        }
                    }
                    // All existing dirs matched, we can place this here
                    candidates.push((id, gggg.clone()));
                }
            }
        }
    }
    if candidates.len() > 0 {
        // println!("cand: {:?}", candidates.len());
        // println!("placing {} at {:?}", id, coord);
	for (id, g) in candidates {
	    let mut gog = grid_of_grids.clone();
            gog.insert(coord, (*id, g.clone()));
            let mut p = placed.clone();
	    p.insert(*id);
            // go to the next coord
            let new_coord = if coord[0] + 1 < expected {
		[coord[0] + 1, coord[1]]
	    } else {
		[0, coord[1] + 1]
	    };
	    let next_gog = place(grids, new_coord, p, gog);
	    if !next_gog.is_empty() {
		return next_gog;
	    }
        }
    // } else {
    // 	println!("no candidates!");
    }
    HashMap::new()
}

fn part2(input: &Parsed) -> Answer {
    let mut grids = HashMap::new();
    for (id, g) in input {
        grids.insert(*id, g.clone());
    }
    let coord = [0, 0];
    let grid_of_grids = HashMap::new();
    let placed = HashSet::new();
    let grid_of_grids = place(&grids, coord, placed, grid_of_grids);
    println!("{:?}", grid_of_grids);
    let min_x = grid_of_grids.iter().map(|(p, _v)| p[0]).min().unwrap();
    let min_y = grid_of_grids.iter().map(|(p, _v)| p[1]).min().unwrap();
    let max_x = grid_of_grids.iter().map(|(p, _v)| p[0]).max().unwrap();
    let max_y = grid_of_grids.iter().map(|(p, _v)| p[1]).max().unwrap();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if let Some((id, _)) = grid_of_grids.get(&[x, y]) {
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
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if let Some((id, g)) = grid_of_grids.get(&[x, y]) {
                println!("Tile: {}", id);
                let ([min_xx, min_yy], [max_xx, max_yy]) = g.extents();
                for yy in (min_yy+1)..max_yy {
                    for xx in (min_xx+1)..max_xx {
                        if let Some(v) = g.get_value([xx, yy]) {
                            print!("{}", v);
                            big_grid.insert([xxx, yyy], v);
                        } else {
                            panic!();
                        }
                        xxx += 1;
                    }
                    println!();
                    xxx -= max_xx - min_xx - 1;
                    yyy += 1;
                }
                println!();
                xxx += max_xx - min_xx - 1;
                if x != max_x {
                    yyy -= max_yy - min_yy - 1;
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
