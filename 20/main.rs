use aoc::Grid;
use aoc::GridDrawer;
use std::iter::*;
use std::collections::HashMap;
use std::collections::VecDeque;

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

fn part1(input: &Parsed) -> Answer {
    let mut matches : HashMap<i64, Vec<(i64, i64, i64, bool)>> = HashMap::new();
    let mut grids = HashMap::new();
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
		    	println!("{},{} matches {},{}", input[i].0, di, input[j].0, dj);
			matches.entry(input[i].0).or_insert(vec![]).push((di as i64, input[j].0, dj as i64, false));
		    } else if *edge == other_edge_reversed {
		    	println!("{},{} matches {},{} flipped", input[i].0, di, input[j].0, dj);
			matches.entry(input[i].0).or_insert(vec![]).push((di as i64, input[j].0, dj as i64, true));
		    }
		}
	    }
	}
	grids.insert(input[i].0, input[i].1.clone());
    }
    let mut ans = 1;
    for (id, b) in matches {
	println!("{}, {:?}", id, b);
	if b.len() == 2 {
	    ans *= id;
	}
    }
    ans
    // let mut grid_of_grids = HashMap::new();
    // let mut queue = VecDeque::new();
    // while let Some((coord, id)) = queue.pop_back() {
    // 	println!("{:?}, {:?}", coord, id);
    // 	grid_of_grids.insert(coord, id);
    // 	if let Some(m) = matches.get(&id) {
    // 	    for (other_id, d) in m {
    // 		println!("{}, {}", other_id, d);
    // 		let dir = match d {
    // 		    0 => aoc::NORTH,
    // 		    1 => aoc::EAST,
    // 		    2 => aoc::SOUTH,
    // 		    3 => aoc::WEST,
    // 		    _ => panic!(),
    // 		};
    // 		let new_coord = aoc::point_add(coord, dir);
    // 		let new_id = other_id;
    // 		queue.push_back((new_coord, *new_id));
    // 	    }
    // 	} else {
    // 	}
    // }
    // println!("{:?}", grid_of_grids);
    // let min_x = grid_of_grids.iter().map(|(p, _v)| p[0]).min().unwrap();
    // let min_y = grid_of_grids.iter().map(|(p, _v)| p[1]).min().unwrap();
    // let max_x = grid_of_grids.iter().map(|(p, _v)| p[0]).max().unwrap();
    // let max_y = grid_of_grids.iter().map(|(p, _v)| p[1]).max().unwrap();
    // let mut big_grid = HashMap::new();
    // let mut xxx = 0;
    // let mut yyy = 0;
    // for _y in min_y..=max_y {
    // 	for x in min_x..=max_x {
    // 	    let id = grid_of_grids.get(&[min_x, min_y]).unwrap();
    // 	    let g = grids.get(id).unwrap();
    // 	    let ([min_xx, min_yy], [max_xx, max_yy]) = g.extents();
    // 	    for yy in min_yy..=max_yy {
    // 		for xx in min_xx..=max_xx {
    // 		    let v = g.get_value([xx, yy]).unwrap();
    // 		    big_grid.insert([xxx, yyy], v);
    // 		    xxx += 1;
    // 		}
    // 		xxx -= max_xx - min_xx + 1;
    // 		yyy += 1;
    // 	    }
    // 	    xxx += max_xx - min_xx + 1;
    // 	    if x != max_x {
    // 		yyy -= max_yy - min_yy + 1;
    // 	    }
    // 	}
    // }
    // let mut gd = aoc::PrintGridDrawer::new(|c| c);
    // gd.draw(&big_grid);
    // *grid_of_grids.get(&[min_x, min_y]).unwrap() *
    // *grid_of_grids.get(&[min_x, max_y]).unwrap() *
    // *grid_of_grids.get(&[max_x, min_y]).unwrap() *
    // *grid_of_grids.get(&[max_x, max_y]).unwrap()
}

fn part2(_: &Parsed) -> Answer {
    0
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
