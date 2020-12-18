use std::iter::*;
use std::collections::VecDeque;

type Parsed = Vec<Vec<String>>;
type Answer = i64;

fn calc(s: &[String], a: i64) -> (i64, usize) {
    let mut op1 = a;
    let mut op = None;
    let mut i = 0;
    while i < s.len() {
        let o = s[i].as_str();
        i += 1;
        match o {
            "(" => {
                let (val, eaten) = calc(&s[i..], 0);
                op1 = match op {
                    Some("+") => op1 + val,
                    Some("*") => op1 * val,
                    _ => val,
                };
                i += eaten;
            }
            ")" => break,
            "+" => op = Some("+"),
            "*" => op = Some("*"),
            x => {
                let op2 = x.parse::<i64>().unwrap();
                op1 = match op {
                    Some("+") => op1 + op2,
                    Some("*") => op1 * op2,
                    _ => op2,
                }
            }
        }
    }
    (op1, i)
}

fn prec(s: &str) -> i64 {
    match s {
	"+" => 2,
	"*" => 1,
	_ => -1,
    }
}

fn calc2(s: &[String]) -> i64{
    // Convert to postfix
    let mut stack = VecDeque::new();
    let mut out : Vec<String> = vec![];
    for o in s {
        match o.as_str() {
            "(" => stack.push_back(o.clone()),
            ")" => {
		while let Some(x) = stack.back() {
		    if x == "(" {
			stack.pop_back();
			break;
		    }
		    out.push(x.to_string());
		    stack.pop_back();
		}
	    },
            "+" | "*" => {
		while let Some(x) = stack.back() {
		    if prec(o) <= prec(&x) {
			out.push(x.to_string());
			stack.pop_back();
		    } else {
			break;
		    }
		}
		stack.push_back(o.clone());
	    }
            x => out.push(x.to_string()),
        }
    }
    while let Some(x) = stack.pop_back() {
	out.push(x);
    }
    // evaluate postfix
    for o in out {
	match o.as_str() {
	    "*" => {
		let a = stack.pop_back().unwrap();
		let b = stack.pop_back().unwrap();
		stack.push_back((a.parse::<i64>().unwrap() * b.parse::<i64>().unwrap()).to_string());
	    }
	    "+" => {
		let a = stack.pop_back().unwrap();
		let b = stack.pop_back().unwrap();
		stack.push_back((a.parse::<i64>().unwrap() + b.parse::<i64>().unwrap()).to_string());
	    }
	    x => stack.push_back(x.to_string()),
	}
    }
    stack[0].parse::<i64>().unwrap()
}

fn part1(input: &Parsed) -> Answer {
    input.iter().map(|x| calc(x, 0).0).sum()
}

fn part2(input: &Parsed) -> Answer {
    input.iter().map(|x| calc2(x)).sum()
}

fn parse_line(line: &str) -> Vec<String> {
    let mut y = vec![];
    let mut ix = 0;
    for (i, c) in line.chars().enumerate() {
        match c {
            ' ' => {
                let s = line[ix..i].to_string();
                if !s.is_empty() {
                    y.push(s);
                }
                ix = i + 1
            }
            '(' | ')' => {
                let s = line[ix..i].to_string();
                if !s.is_empty() {
                    y.push(s);
                }
                y.push(c.to_string());
                ix = i + 1
            }
            _ => {}
        }
    }
    let s = line[ix..].to_string();
    if !s.is_empty() {
        y.push(s);
    }
    y
}

fn parse(lines: &[String]) -> Parsed {
    let mut x = vec![];
    for line in lines {
        x.push(parse_line(&line));
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
        let parsed = parse_line("1 + (2 * 3) + (4 * (5 + 6))");
        assert_eq!(calc(&parsed, 0), (51, parsed.len()));
        let parsed = parse_line("2 * 3 + (4 * 5)");
        assert_eq!(calc(&parsed, 0), (26, parsed.len()));
        let parsed = parse_line("5 + (8 * 3 + 9 + 3 * 4 * 3)");
        assert_eq!(calc(&parsed, 0), (437, parsed.len()));
        let parsed = parse_line("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");
        assert_eq!(calc(&parsed, 0), (12240, parsed.len()));
        let parsed = parse_line("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
        assert_eq!(calc(&parsed, 0), (13632, parsed.len()));
    }

    #[test]
    fn test_calc2() {
        let parsed = parse_line("1 + (2 * 3) + (4 * (5 + 6))");
        assert_eq!(calc2(&parsed), 51);
        let parsed = parse_line("2 * 3 + (4 * 5)");
        assert_eq!(calc2(&parsed), 46);
        let parsed = parse_line("5 + (8 * 3 + 9 + 3 * 4 * 3)");
        assert_eq!(calc2(&parsed), 1445);
        let parsed = parse_line("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");
        assert_eq!(calc2(&parsed), 669060);
        let parsed = parse_line("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
        assert_eq!(calc2(&parsed), 23340);
    }
}
