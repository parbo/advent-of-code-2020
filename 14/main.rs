use aoc::GridDrawer;
use aoc::ParseError;
use std::collections::{BTreeMap, HashMap};
use std::iter::*;
use std::str::FromStr;

#[derive(Debug)]
enum Op {
    Mask(String),
    Mem(i64, i64),
}

impl FromStr for Op {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("mask") {
            Ok(Op::Mask(s[7..].to_string()))
        } else {
            let parts = aoc::split(s, |c| c == '[' || c == ']' || c == '=');
            Ok(Op::Mem(parts[1].parse()?, parts[2].parse()?))
        }
    }
}

fn part1(ops: &[Op]) -> i64 {
    let mut mem = HashMap::new();
    let mut mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string();
    for op in ops {
        match op {
            Op::Mask(m) => {
                mask = m.clone();
            }
            Op::Mem(addr, value) => {
                let mut memval = 0;
                for (i, x) in mask.chars().enumerate() {
                    let bit = 1 << (35 - i);
                    let newm = match x {
                        'X' => (memval & !bit) | (value & bit),
                        '0' => memval & !bit,
                        '1' => (memval & !bit) | bit,
                        _ => panic!(),
                    };
                    memval = newm;
                }
                *mem.entry(addr).or_insert(0) = memval;
            }
        }
    }
    mem.values().sum()
}

fn part2(ops: &[Op], draw: bool) -> i64 {
    let mut all_mem = vec![];
    let mut mem = BTreeMap::new();
    let mut mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string();
    for op in ops {
        match op {
            Op::Mask(m) => {
                mask = m.clone();
            }
            Op::Mem(addr, value) => {
                let mut base_addr = *addr;
                let mut floating = vec![];
                for (i, x) in mask.chars().enumerate() {
                    let bit = 1 << (35 - i);
                    match x {
                        'X' => {
                            base_addr = base_addr & !bit;
                            floating.push(bit);
                        }
                        '0' => {}
                        '1' => base_addr = (base_addr & !bit) | bit,
                        _ => panic!(),
                    }
                }
                let bits = floating.len();
                for i in 0..(1 << bits) {
                    let s: i64 = floating
                        .iter()
                        .enumerate()
                        .filter(|(ix, _)| (i & (1 << ix)) != 0)
                        .map(|(_, x)| *x)
                        .sum();
                    let a = base_addr | s;
                    *mem.entry(a).or_insert(0) = *value;
                }
            }
        }
        if draw {
            all_mem.push(mem.clone());
        }
    }
    if draw {
        let vals = mem.len();
        let side = (vals as f64).sqrt() as usize + 1;
        let mut gd = aoc::BitmapGridDrawer::new(
            |x| {
                [
                    ((x & (0xff << 16)) >> 16) as u8,
                    ((x & (0xff << 8)) >> 8) as u8,
                    (x & 0xff) as u8,
                ]
            },
            "ppm/day14/part2",
        );
        for m in all_mem {
            let mut grid = vec![vec![0; side]; side];
            let mut x = 0;
            let mut y = 0;
            for (_, v) in m {
                if x + 1 < side {
                    x += 1;
                } else {
                    x = 0;
                    y += 1;
                }
                grid[y][x] = v;
            }
            gd.draw(&grid);
        }
    }
    mem.values().sum()
}

fn parse(lines: &[String]) -> Vec<Op> {
    lines.iter().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let (part, lines) = aoc::read_lines();
    let parsed = parse(&lines);
    let result = if part == 1 {
        part1(&parsed)
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
        let input = vec![
            "mask = 000000000000000000000000000000X1001X".to_string(),
            "mem[42] = 100".to_string(),
            "mask = 00000000000000000000000000000000X0XX".to_string(),
            "mem[26] = 1".to_string(),
        ];
        let parsed = parse(&input);
        assert_eq!(part2(&parsed, false), 208);
    }
}
