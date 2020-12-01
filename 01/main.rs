use aoc;
use std::iter::*;

fn part1(v: &Vec<i64>) -> i64 {
    for i in 0..v.len() {
	for j in i..v.len() {
	    if v[i] + v[j] == 2020 {
		return v[i] * v[j];
	    }
	}
    }
    0
}

fn part2(v: &Vec<i64>) -> i64 {
    for i in 0..v.len() {
	for j in i..v.len() {
	    for k in j..v.len() {
		if v[i] + v[j] + v[k] == 2020 {
		    return v[i] * v[j] * v[k];
		}
	    }
	}
    }
    0
}

fn parse(lines: &[String]) -> Vec<i64> {
    lines.iter().map(|x| x.parse::<i64>().unwrap()).collect()
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
    // use super::part1;

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(&vec![0]), 0);
    // }
}
