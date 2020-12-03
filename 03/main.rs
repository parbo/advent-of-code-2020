use aoc;
use std::iter::*;

fn slope(forest: &Vec<Vec<char>>, xx: usize, yy: usize) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;
    let w = forest[0].len();
    if forest[y][x] == '#' {
        trees = trees + 1;
    }
    loop {
        x = x + xx;
        y = y + yy;
        if y >= forest.len() {
            return trees;
        }
        if forest[y][x % w] == '#' {
            trees = trees + 1;
        }
    }
}

fn part1(forest: &Vec<Vec<char>>) -> i64 {
    slope(forest, 3, 1)
}

fn part2(f: &Vec<Vec<char>>) -> i64 {
    slope(f, 1, 1) * slope(f, 3, 1) * slope(f, 5, 1) * slope(f, 7, 1) * slope(f, 1, 2)
}

fn parse(lines: &[String]) -> Vec<Vec<char>> {
    lines.iter().map(|x| x.chars().collect()).collect()
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
