use aoc::{self, ParseError};
use std::iter::*;
use std::str::FromStr;

#[derive(Debug)]
struct Policy {
    min: i64,
    max: i64,
    c: char,
    password: String,
}

impl FromStr for Policy {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s
            .split(|c| c == '-' || c == ':' || c == ' ')
            .map(|w| w.trim())
            .collect();
        let min = parts[0].parse::<i64>()?;
        let max = parts[1].parse::<i64>()?;
        let c = parts[2].chars().nth(0).ok_or(ParseError::Generic)?;
        let password = parts[4].to_string();

        Ok(Policy {
            min,
            max,
            c,
            password,
        })
    }
}

impl Policy {
    fn is_valid(&self) -> bool {
        let count = self.password.chars().filter(|c| *c == self.c).count() as i64;
        count >= self.min && count <= self.max
    }

    fn is_valid_updated(&self) -> bool {
        (self.password.chars().nth((self.min - 1) as usize).unwrap() == self.c)
            ^ (self.password.chars().nth((self.max - 1) as usize).unwrap() == self.c)
    }
}

fn part1(passwords: &Vec<Policy>) -> i64 {
    passwords.iter().filter(|p| p.is_valid()).count() as i64
}

fn part2(passwords: &Vec<Policy>) -> i64 {
    passwords.iter().filter(|p| p.is_valid_updated()).count() as i64
}

fn parse(lines: &[String]) -> Vec<Policy> {
    lines.iter().map(|x| x.parse::<Policy>().unwrap()).collect()
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
