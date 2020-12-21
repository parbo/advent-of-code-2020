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

fn part2(input: &Parsed) -> Answer {
    let matches = get_matches(input);
    let mut grid_of_grids = HashMap::new();
    let mut grids = HashMap::new();
    let mut placed = HashSet::new();
    for (id, g) in input {
        grids.insert(id, g.clone());
    }
    // Find one corner to use as a starting point
    let mut coord = [0, 0];
    for (id, b) in &matches {
        if b.len() == 2 {
	    println!("placing {} at {:?}", id, coord);
	    grid_of_grids.insert(coord, (id, grids.get(&id).unwrap().clone()));
	    placed.insert(*id);
	    break;
        }
    }
    // Look at the east neighbor
    coord = [coord[0] + 1, coord[1]];
    loop {
	println!("coord: {:?}", coord);
	'top: for (id, g) in &grids {
	    if placed.contains(&id) {
		continue;
	    }
	    // Try to make it fit with the neighbors
	    for rot in 0..4 {
		let mut gg = g.clone();
		match rot {
		    0 => {},
		    1 => gg.rotate_90_cw(),
		    2 => gg.rotate_180_cw(),
		    3 => gg.rotate_270_cw(),
		    _ => panic!(),
		}
		for flip_x in 0..2 {
		    let mut ggg = gg.clone();
		    match flip_x {
			0 => {},
			1 => ggg.flip_horizontal(),
			_ => panic!(),
		    }
		    'outer: for flip_y in 0..2 {
			let mut gggg = ggg.clone();
			match flip_y {
			    0 => {},
			    1 => gggg.flip_vertical(),
			    _ => panic!(),
			}
			for d in aoc::DIRECTIONS.clone() {
			    let c = aoc::point_add(coord, d);
			    if let Some((id, g)) = grid_of_grids.get(&c) {
//				println!("compare to grid: {} at {:?}", id, c);
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
				println!("d: {:?}:  {:?} - {:?}", d, e, other_e);
			    }
			}
			// All existing dirs matched, we can place this here
			println!("placing {} at {:?}", id, coord);
			grid_of_grids.insert(coord, (id, gggg.clone()));
			placed.insert(**id);
			// go to the next column
			coord = [coord[0] + 1, coord[1]];
			continue 'top;
		    }
		}
	    }
	}
	// Could not place any grid!
	println!("{}, {}", grid_of_grids.len(), grids.len());
	if grid_of_grids.len() == grids.len() {
	    break;
	}
	// go to the next row
	coord = [0, coord[1] + 1];
	if coord[1] > 5 {
	    panic!();
	}
    }
    println!("{:?}", grid_of_grids);
    let min_x = grid_of_grids.iter().map(|(p, _v)| p[0]).min().unwrap();
    let min_y = grid_of_grids.iter().map(|(p, _v)| p[1]).min().unwrap();
    let max_x = grid_of_grids.iter().map(|(p, _v)| p[0]).max().unwrap();
    let max_y = grid_of_grids.iter().map(|(p, _v)| p[1]).max().unwrap();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if let Some((id, _transform)) = grid_of_grids.get(&[x, y]) {
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
                for yy in min_yy..=max_yy {
                    for xx in min_xx..=max_xx {
                        if let Some(v) = g.get_value([xx, yy]) {
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
