use std::iter::*;
use std::collections::VecDeque;
use aoc::ParseError;
use std::str::FromStr;

type Parsed = Vec<Vec<Ops>>;
type Answer = i64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ops {
    Add,
    Mul,
    Num(i64),
    LParen,
    RParen
}

impl FromStr for Ops {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
	match s {
	    "(" => Ok(Ops::LParen),
	    ")" => Ok(Ops::RParen),
	    "*" => Ok(Ops::Mul),
	    "+" => Ok(Ops::Add),
	    x => Ok(Ops::Num(x.parse()?)),
	}
    }
}

fn tokenize(line: &str) -> Vec<Ops> {
    let mut y = vec![];
    let mut ix = 0;
    for (i, c) in line.chars().enumerate() {
        match c {
            ' ' => {
                let s = &line[ix..i];
                if !s.is_empty() {
                    y.push(s.parse().unwrap());
                }
                ix = i + 1
            }
            '(' | ')' => {
                let s = &line[ix..i];
                if !s.is_empty() {
                    y.push(s.parse().unwrap());
                }
                y.push(c.to_string().parse().unwrap());
                ix = i + 1
            }
            _ => {}
        }
    }
    let s = &line[ix..];
    if !s.is_empty() {
        y.push(s.parse().unwrap());
    }
    y
}

fn prec1(s: Ops) -> i64 {
    match s {
	Ops::Add => 1,
	Ops::Mul => 1,
	_ => -1,
    }
}

fn prec2(s: Ops) -> i64 {
    match s {
	Ops::Add => 2,
	Ops::Mul => 1,
	_ => -1,
    }
}

fn calc<F>(s: &[Ops], prec: F) -> Option<i64> where F: Fn(Ops) -> i64 {
    // Convert to postfix
    let mut stack = VecDeque::new();
    let mut postfix = vec![];
    for op in s {
        match op {
            Ops::LParen => stack.push_back(*op),
            Ops::RParen => {
		while let Some(x) = stack.back() {
		    if *x == Ops::LParen {
			stack.pop_back();
			break;
		    }
		    postfix.push(*x);
		    stack.pop_back();
		}
	    },
            Ops::Add | Ops::Mul => {
		while let Some(x) = stack.back() {
		    if prec(*op) <= prec(*x) {
			postfix.push(*x);
			stack.pop_back();
		    } else {
			break;
		    }
		}
		stack.push_back(*op);
	    }
            x => postfix.push(*x),
        }
    }
    while let Some(x) = stack.pop_back() {
	postfix.push(x);
    }
    // Evaluate postfix
    for op in postfix {
	if let Ops::Num(_) = op {
	    stack.push_back(op);
	} else {
	    if let Some((Ops::Num(a), Ops::Num(b))) = stack.pop_back().zip(stack.pop_back()) {
		if op == Ops::Mul {
		    stack.push_back(Ops::Num(a * b));
		} else {
		    stack.push_back(Ops::Num(a + b));
		}
	    }
	}
    }
    if let Some(Ops::Num(a)) = stack.back() {
	Some(*a)
    } else {
	None
    }
}

fn part1(input: &Parsed) -> Answer {
    input.iter().map(|x| calc(x, prec1).unwrap()).sum()
}

fn part2(input: &Parsed) -> Answer {
    input.iter().map(|x| calc(x, prec2).unwrap()).sum()
}

fn parse(lines: &[String]) -> Parsed {
    let mut x = vec![];
    for line in lines {
        x.push(tokenize(&line));
    }
    x
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
    fn test_calc() {
        let parsed = tokenize("1 + (2 * 3) + (4 * (5 + 6))");
        assert_eq!(calc(&parsed, prec1), Some(51));
        let parsed = tokenize("2 * 3 + (4 * 5)");
        assert_eq!(calc(&parsed, prec1), Some(26));
        let parsed = tokenize("5 + (8 * 3 + 9 + 3 * 4 * 3)");
        assert_eq!(calc(&parsed, prec1), Some(437));
        let parsed = tokenize("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");
        assert_eq!(calc(&parsed, prec1), Some(12240));
        let parsed = tokenize("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
        assert_eq!(calc(&parsed, prec1), Some(13632));
    }

    #[test]
    fn test_calc2() {
        let parsed = tokenize("1 + (2 * 3) + (4 * (5 + 6))");
        assert_eq!(calc(&parsed, prec2), Some(51));
        let parsed = tokenize("2 * 3 + (4 * 5)");
        assert_eq!(calc(&parsed, prec2), Some(46));
        let parsed = tokenize("5 + (8 * 3 + 9 + 3 * 4 * 3)");
        assert_eq!(calc(&parsed, prec2), Some(1445));
        let parsed = tokenize("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");
        assert_eq!(calc(&parsed, prec2), Some(669060));
        let parsed = tokenize("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
        assert_eq!(calc(&parsed, prec2), Some(23340));
    }
}
