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

fn part2(input: &Parsed) -> Answer {
    let matches = get_matches(input);
    let mut grid_of_grids = HashMap::new();
    let mut queue = VecDeque::new();
    // Find one corner to use as a starting point
    for (id, b) in &matches {
        if b.len() == 2 {
            println!("{}, {:?}", id, b);
            queue.push_back(([0, 0], id, 0, false));
        }
    }
    let mut seen = HashSet::new();
    while let Some((coord, id, rot, flipped)) = queue.pop_back() {
        if seen.contains(id) {
            continue;
        }
        println!("coord: {:?}, id: {:?}, rot: {}, flipped: {}", coord, id, rot, flipped);
        grid_of_grids.insert(coord, id);
        seen.insert(id);
        if let Some(m) = matches.get(&id) {
            for (di, idj, dj, flippedj) in m {
                let rotj = match di {
                    0 => match dj {
                        0 => 2,
                        1 => 3,
                        2 => 0,
                        3 => 1,
                        _ => panic!(),
                    },
                    1 => match dj {
                        0 => 1,
                        1 => 2,
                        2 => 3,
                        3 => 0,
                        _ => panic!(),
                    },
                    2 => match dj {
                        0 => 0,
                        1 => 1,
                        2 => 2,
                        3 => 3,
                        _ => panic!(),
                    },
                    3 => match dj {
                        0 => 3,
                        1 => 0,
                        2 => 1,
                        3 => 2,
                        _ => panic!(),
                    },
                    _ => panic!(),
                };
                let rot_i = if flipped { (4 - rot) % 4 } else { rot };
                let unrotated_di = (di + (4 - rot_i) % 4) % 4;
                println!("id: {}, di: {}, flipped: {}, idj: {}, dj: {}, flippedj: {}, rot_i: {}, unrotated_di: {}, rotj: {}", id, di, flipped, idj, dj, flippedj, rot_i, unrotated_di, rotj);
                let dir = match unrotated_di {
                    0 => aoc::NORTH,
                    1 => aoc::EAST,
                    2 => aoc::SOUTH,
                    3 => aoc::WEST,
                    _ => panic!(),
                };
                let new_coord = aoc::point_add(coord, dir);
                let new_id = idj;
                let rot_j = if *flippedj { (4 - rotj) % 4 } else { rotj };
                let new_rot = (rot_i + rot_j) % 4;
                queue.push_back((new_coord, new_id, new_rot, flipped ^ *flippedj));
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
            if let Some(id) = grid_of_grids.get(&[x, y]) {
                print!("{}, ", id);
            } else {
                print!("    , ");
            }
        }
        println!();
    }

    // let mut big_grid = HashMap::new();
    // let mut xxx = 0;
    // let mut yyy = 0;
    // let mut grids = HashMap::new();
    // for (id, g) in input {
    //     grids.insert(id, g);
    // }
    // for _y in min_y..=max_y {
    //     for x in min_x..=max_x {
    //         if let Some(id) = grid_of_grids.get(&[min_x, min_y]) {
    //             let g = grids.get(id).unwrap();
    //             let ([min_xx, min_yy], [max_xx, max_yy]) = g.extents();
    //             for yy in min_yy..=max_yy {
    //                 for xx in min_xx..=max_xx {
    //                     let v = g.get_value([xx, yy]).unwrap();
    //                     big_grid.insert([xxx, yyy], v);
    //                     xxx += 1;
    //                 }
    //                 xxx -= max_xx - min_xx + 1;
    //                 yyy += 1;
    //             }
    //             xxx += max_xx - min_xx + 1;
    //             if x != max_x {
    //                 yyy -= max_yy - min_yy + 1;
    //             }
    //         } else {
    // 		xxx += 10;  // jaja
    // 	    }
    //     }
    // }
    // let mut gd = aoc::PrintGridDrawer::new(|c| c);
    // gd.draw(&big_grid);
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
