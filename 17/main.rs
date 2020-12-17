use aoc::{vec4_add, vec_add, Vec3, Vec4};
use std::collections::HashMap;
use std::iter::*;

// #[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
// #[display("{thing}: {al}-{ah} or {bl}-{bh}")]
// struct Rule {
//     thing: String,
//     al: i64,
//     ah: i64,
//     bl: i64,
//     bh: i64,
// }

type Parsed = HashMap<Vec3, char>;
type Answer = usize;

fn extents(g: &Parsed) -> (Vec3, Vec3) {
    let min_x = g.iter().map(|(p, _v)| p[0]).min().unwrap();
    let min_y = g.iter().map(|(p, _v)| p[1]).min().unwrap();
    let min_z = g.iter().map(|(p, _v)| p[2]).min().unwrap();
    let max_x = g.iter().map(|(p, _v)| p[0]).max().unwrap();
    let max_y = g.iter().map(|(p, _v)| p[1]).max().unwrap();
    let max_z = g.iter().map(|(p, _v)| p[2]).max().unwrap();
    ([min_x, min_y, min_z], [max_x, max_y, max_z])
}

fn extents4(g: &HashMap<Vec4, char>) -> (Vec4, Vec4) {
    let min_x = g.iter().map(|(p, _v)| p[0]).min().unwrap();
    let min_y = g.iter().map(|(p, _v)| p[1]).min().unwrap();
    let min_z = g.iter().map(|(p, _v)| p[2]).min().unwrap();
    let min_w = g.iter().map(|(p, _v)| p[3]).min().unwrap();
    let max_x = g.iter().map(|(p, _v)| p[0]).max().unwrap();
    let max_y = g.iter().map(|(p, _v)| p[1]).max().unwrap();
    let max_z = g.iter().map(|(p, _v)| p[2]).max().unwrap();
    let max_w = g.iter().map(|(p, _v)| p[3]).max().unwrap();
    ([min_x, min_y, min_z, min_w], [max_x, max_y, max_z, max_w])
}

fn dirs() -> Vec<Vec3> {
    let mut d = vec![];
    for z in -1..=1 {
        for y in -1..=1 {
            for x in -1..=1 {
                if x == 0 && y == 0 && z == 0 {
                    continue;
                }
                d.push([x, y, z]);
            }
        }
    }
    d
}

fn dirs4() -> Vec<Vec4> {
    let mut d = vec![];
    for w in -1..=1 {
        for z in -1..=1 {
            for y in -1..=1 {
                for x in -1..=1 {
                    if x == 0 && y == 0 && z == 0 && w == 0 {
                        continue;
                    }
                    d.push([x, y, z, w]);
                }
            }
        }
    }
    d
}

fn print_slices(g: &Parsed) {
    let ([min_x, min_y, min_z], [max_x, max_y, max_z]) = extents(g);
    for z in min_z..=max_z {
        println!("z={}", z);
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                print!("{}", g.get(&[x, y, z]).unwrap_or(&'.'));
            }
            println!();
        }
        println!();
    }
}

fn step(g: &Parsed, d: &[Vec3]) -> Parsed {
    let mut newg = g.clone();
    let ([min_x, min_y, min_z], [max_x, max_y, max_z]) = extents(&newg);
    for z in (min_z - 1)..=(max_z + 1) {
        for y in (min_y - 1)..=(max_y + 1) {
            for x in (min_x - 1)..=(max_x + 1) {
                let p = [x, y, z];
                let mut active = 0;
                let c = g.get(&p).unwrap_or(&'.');
                for dir in d {
                    let np = vec_add(p, *dir);
                    match g.get(&np) {
                        Some('#') => {
                            active += 1;
                        }
                        _ => {}
                    }
                }
                if *c == '#' && !(active == 2 || active == 3) {
                    newg.remove(&p);
                } else if *c == '.' && active == 3 {
                    newg.insert(p, '#');
                }
            }
        }
    }
    newg
}

fn part1(input: &Parsed) -> Answer {
    let d = dirs();
    let mut i = 0;
    let mut g = input.clone();
    print_slices(&g);
    loop {
        let newg = step(&g, &d);
        println!("After {} cycle:", i);
        print_slices(&newg);
        if i == 6 {
            break;
        }
        i += 1;
        g = newg.clone();
    }
    g.iter().filter(|(_p, v)| **v == '#').count()
}

fn part2(input: &Parsed) -> Answer {
    let d = dirs4();
    let mut i = 0;
    let mut g = HashMap::new();
    for ([x, y, z], v) in input {
        g.insert([*x, *y, *z, 0], *v);
    }
    loop {
        let mut newg = g.clone();
        let ([min_x, min_y, min_z, min_w], [max_x, max_y, max_z, max_w]) = extents4(&newg);
        for w in (min_w - 1)..=(max_w + 1) {
            for z in (min_z - 1)..=(max_z + 1) {
                for y in (min_y - 1)..=(max_y + 1) {
                    for x in (min_x - 1)..=(max_x + 1) {
                        let p = [x, y, z, w];
                        let mut active = 0;
                        let c = g.get(&p).unwrap_or(&'.');
                        for dir in &d {
                            let np = vec4_add(p, *dir);
                            match g.get(&np) {
                                Some('#') => {
                                    active += 1;
                                }
                                _ => {}
                            }
                        }
                        if *c == '#' && !(active == 2 || active == 3) {
                            newg.remove(&p);
                        } else if *c == '.' && active == 3 {
                            newg.insert(p, '#');
                        }
                    }
                }
            }
        }
        if i == 6 {
            break;
        }
        i += 1;
        g = newg.clone();
    }
    g.iter().filter(|(_p, v)| **v == '#').count()
}

fn parse(lines: &[String]) -> Parsed {
    let mut g = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            g.insert([x as i64, y as i64, 0i64], c);
        }
    }
    g
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
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![".#.".to_string(), "..#".to_string(), "###".to_string()];
        let parsed = parse(&input);
        assert_eq!(part1(&parsed), 112);
    }
}
