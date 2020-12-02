use aoc::{self, ParseError};
use std::iter::*;
use std::str::FromStr;

#[derive(Debug)]
struct Policy {
    min: usize,
    max: usize,
    c: char,
    password: String,
}

impl FromStr for Policy {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = aoc::split(s, |c| c == '-' || c == ':' || c == ' ');
        Ok(Policy {
            min: parts[0].parse()?,
            max: parts[1].parse()?,
            c: aoc::parse_char(parts[2], 0)?,
            password: parts[4].to_string(),
        })
    }
}

impl Policy {
    fn is_valid(&self) -> bool {
        let count = self.password.chars().filter(|c| *c == self.c).count();
        count >= self.min && count <= self.max
    }

    fn is_valid_updated(&self) -> bool {
        (self.password.chars().nth(self.min - 1).unwrap() == self.c)
            ^ (self.password.chars().nth(self.max - 1).unwrap() == self.c)
    }
}

fn part1(passwords: &Vec<Policy>) -> usize {
    passwords.iter().filter(|p| p.is_valid()).count()
}

fn part2(passwords: &Vec<Policy>) -> usize {
    passwords.iter().filter(|p| p.is_valid_updated()).count()
}

fn parse(lines: &[String]) -> Vec<Policy> {
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
