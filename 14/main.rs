use aoc::ParseError;
use std::collections::{HashMap, HashSet};
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

fn combos(f: &[i64], c: &mut HashSet<Vec<i64>>) {
    let v = f.to_owned();
    if !c.insert(v.clone()) {
	return;
    }
    for i in 0..v.len() {
	let mut vv = v.clone();
	vv[i] = 0;
	combos(&vv, c);
    }
}

fn part2(ops: &[Op]) -> i64 {
    let mut mem = HashMap::new();
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
                        '0' => {},
                        '1' => base_addr = (base_addr & !bit) | bit,
                        _ => panic!(),
                    }
                }
		let mut c = HashSet::new();
		combos(&floating, &mut c);
		for combo in c {
		    let s : i64 = combo.iter().sum();
		    let a = base_addr | s;
                    *mem.entry(a).or_insert(0) = *value;
		}
            }
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
        part2(&parsed)
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
        assert_eq!(part2(&parsed), 208);
    }
}
