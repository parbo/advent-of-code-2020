use std::iter::*;

fn part1(v: &[i64]) -> i64 {
    for i in 0..v.len() {
        for j in i + 1..v.len() {
            if v[i] + v[j] == 2020 {
                return v[i] * v[j];
            }
        }
    }
    0
}

fn part2(v: &[i64]) -> i64 {
    for i in 0..v.len() {
        for j in i + 1..v.len() {
            for k in j + 1..v.len() {
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
