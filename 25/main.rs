use std::iter::*;
use std::collections::HashMap;

type Parsed = Vec<u64>;
type Answer = u64;

/// Computes x such that g^x % modulus == h
/// Ported from C++ code from https://en.wikipedia.org/wiki/Baby-step_giant-step
fn babystep_giantstep(g: u64, h: u64, modulus: u64) -> Option<u64> {
    let m = (modulus as f64).sqrt().ceil() as u64;
    let mut table : HashMap<u64, u64> = HashMap::new();
    table.reserve(m as usize);
    let mut e = 1u128; // temporary values may be bigger than 64 bit
    for i in 0..m {
        table.insert(e as u64, i);
        e = (e * g as u128).rem_euclid(modulus as u128);
    }
    let factor = aoc::mod_exp(g, modulus-m-1, modulus);
    e = h as u128;
    for i in 0..m {
	if let Some(v) = table.get(&(e as u64)) {
            return Some(i*m + v);
        }
        e = (e * factor as u128).rem_euclid(modulus as u128);
    }
    None
}

fn loopsize(a: u64) -> Option<u64> {
    babystep_giantstep(7, a, 20201227)
}

fn transform(a: u64, ls: u64) -> u64 {
    aoc::mod_exp(a, ls, 20201227)
}

fn part1(input: &Parsed) -> Answer {
    let ls0 = loopsize(input[0]).unwrap();
    let k = transform(input[1], ls0);
    k
}

fn part2(_: &Parsed) -> Answer {
    0
}

fn parse(lines: &[String]) -> Parsed {
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
    fn test_part1() {
        assert_eq!(loopsize(5764801), Some(8));
        assert_eq!(loopsize(17807724), Some(11));
	assert_eq!(transform(5764801, 11), 14897079);
	assert_eq!(transform(17807724, 8), 14897079);
    }
}
