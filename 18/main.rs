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

fn calc(s: &[Ops], a: i64) -> (i64, usize) {
    let mut op1 = a;
    let mut op = None;
    let mut i = 0;
    while i < s.len() {
        let o = s[i];
        i += 1;
        match o {
            Ops::LParen => {
                let (val, eaten) = calc(&s[i..], 0);
                op1 = match op {
                    Some(Ops::Add) => op1 + val,
                    Some(Ops::Mul) => op1 * val,
                    _  => val,
                };
                i += eaten;
            }
            Ops::RParen => break,
            Ops::Add | Ops:: Mul => op = Some(o),
            Ops::Num(op2) => {
                op1 = match op {
                    Some(Ops::Add) => op1 + op2,
                    Some(Ops::Mul) => op1 * op2,
                    _ => op2,
                }
            }
        }
    }
    (op1, i)
}

fn prec(s: Ops) -> i64 {
    match s {
	Ops::Add => 2,
	Ops::Mul => 1,
	_ => -1,
    }
}

fn calc2(s: &[Ops]) -> Option<i64> {
    // Convert to postfix
    let mut stack = VecDeque::new();
    let mut out = vec![];
    for o in s {
        match o {
            Ops::LParen => stack.push_back(o.clone()),
            Ops::RParen => {
		while let Some(x) = stack.back() {
		    if *x == Ops::LParen {
			stack.pop_back();
			break;
		    }
		    out.push(*x);
		    stack.pop_back();
		}
	    },
            Ops::Add | Ops::Mul => {
		while let Some(x) = stack.back() {
		    if prec(*o) <= prec(*x) {
			out.push(*x);
			stack.pop_back();
		    } else {
			break;
		    }
		}
		stack.push_back(*o);
	    }
            x => out.push(*x),
        }
    }
    while let Some(x) = stack.pop_back() {
	out.push(x);
    }
    // evaluate postfix
    for o in out {
	if let Ops::Num(_) = o {
	    stack.push_back(o);
	} else {
	    if let Ops::Num(a) = stack.pop_back()? {
		if let Ops::Num(b) = stack.pop_back()? {
		    if o == Ops::Mul {
			stack.push_back(Ops::Num(a * b));
		    } else {
			stack.push_back(Ops::Num(a + b));
		    }
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
    input.iter().map(|x| calc(x, 0).0).sum()
}

fn part2(input: &Parsed) -> Answer {
    input.iter().map(|x| calc2(x).unwrap()).sum()
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
        assert_eq!(calc(&parsed, 0), (51, parsed.len()));
        let parsed = tokenize("2 * 3 + (4 * 5)");
        assert_eq!(calc(&parsed, 0), (26, parsed.len()));
        let parsed = tokenize("5 + (8 * 3 + 9 + 3 * 4 * 3)");
        assert_eq!(calc(&parsed, 0), (437, parsed.len()));
        let parsed = tokenize("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");
        assert_eq!(calc(&parsed, 0), (12240, parsed.len()));
        let parsed = tokenize("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
        assert_eq!(calc(&parsed, 0), (13632, parsed.len()));
    }

    #[test]
    fn test_calc2() {
        let parsed = tokenize("1 + (2 * 3) + (4 * (5 + 6))");
        assert_eq!(calc2(&parsed), Some(51));
        let parsed = tokenize("2 * 3 + (4 * 5)");
        assert_eq!(calc2(&parsed), Some(46));
        let parsed = tokenize("5 + (8 * 3 + 9 + 3 * 4 * 3)");
        assert_eq!(calc2(&parsed), Some(1445));
        let parsed = tokenize("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");
        assert_eq!(calc2(&parsed), Some(669060));
        let parsed = tokenize("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
        assert_eq!(calc2(&parsed), Some(23340));
    }
}
