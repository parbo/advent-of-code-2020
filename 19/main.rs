use aoc::ParseError;
use std::iter::*;
use std::str::FromStr;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Rule {
    Char(char),
    One(usize),
    OneOr(usize, usize),
    Two(usize, usize),
    TwoOr((usize, usize), (usize, usize)),
    Four(usize, usize, usize, usize),
}

impl FromStr for Rule {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('|') {
            let parts = aoc::split(s, |c| c == '|' || c == ' ');
            match parts.len() {
                4 => Ok(Rule::TwoOr(
                    (parts[0].parse()?, parts[1].parse()?),
                    (parts[2].parse()?, parts[3].parse()?),
                )),
                2 => Ok(Rule::OneOr(parts[0].parse()?, parts[1].parse()?)),
                _ => Err(aoc::ParseError::Generic),
            }
        } else if s.contains(' ') {
            let parts = aoc::split_ch(s, ' ');
            match parts.len() {
                4 => Ok(Rule::Four(parts[0].parse()?, parts[1].parse()?, parts[2].parse()?, parts[3].parse()?)),

                2 => Ok(Rule::Two(parts[0].parse()?, parts[1].parse()?)),
                _ => Err(aoc::ParseError::Generic),
            }
        } else {
            if let Ok(n) = s.parse::<usize>() {
                Ok(Rule::One(n))
            } else if s.len() == 3 {
                if let Some(c) = s.chars().nth(1) {
                    Ok(Rule::Char(c))
                } else {
                    Err(aoc::ParseError::Generic)
                }
            } else {
                Err(aoc::ParseError::Generic)
            }
        }
    }
}

type Parsed = (Vec<Rule>, Vec<String>);
type Answer = usize;

fn expand_rule(rules: &[Rule], ix: usize) -> Vec<String> {
    match rules[ix] {
	Rule::Char(c) => vec![c.to_string()],
	Rule::One(i) => expand_rule(rules, i),
	Rule::OneOr(a, b) => {
	    let mut v = vec!["(".to_string()];
	    v.append(&mut expand_rule(rules, a));
	    v.push("|".to_string());
	    v.append(&mut expand_rule(rules, b));
	    v.push(")".to_string());
	    v
	}
	Rule::Two(a, b) => {
	    let mut v = expand_rule(rules, a);
	    v.append(&mut expand_rule(rules, b));
	    v
	}
	Rule::TwoOr((a, b), (c, d)) => {
	    let mut v = vec!["(".to_string()];
	    v.append(&mut expand_rule(rules, a));
	    v.append(&mut expand_rule(rules, b));
	    v.push("|".to_string());
	    v.append(&mut expand_rule(rules, c));
	    v.append(&mut expand_rule(rules, d));
	    v.push(")".to_string());
	    v
	}
	Rule::Four(a, b, c, d) => {
	    let mut v = expand_rule(rules, a);
	    v.append(&mut expand_rule(rules, b));
	    v.append(&mut expand_rule(rules, c));
	    v.append(&mut expand_rule(rules, d));
	    v
	}
    }
}

fn part1(input: &Parsed) -> Answer {
    let (rules, strings) = input;
    // Expand the rules
    let expanded = expand_rule(rules, 0).join("");
    let re = Regex::new(&expanded).unwrap();
    let mut n = 0;
    for s in strings {
	if let Some(mat) = re.find(s) {
	    if mat.start() == 0 && mat.end() == s.len() {
		n += 1;
	    }
	}
    }
    n
}

fn part2(_: &Parsed) -> Answer {
    0
}

fn parse(lines: &[String]) -> Parsed {
    let mut v = vec![];
    let mut s = vec![];
    let mut state = 0;
    for line in lines {
	if line.is_empty() {
	    state = 1
	}
	if state == 0 {
            let parts = aoc::split_ch(line, ':');
            let a = (
                parts[0].parse::<usize>().unwrap(),
                parts[1].parse::<Rule>().unwrap(),
            );
	    v.push(a);
        } else {
	    s.push(line.clone());
	}
    }
    let mut r = vec![Rule::Char('x'); v.len()];
    for (i, rule) in v {
        r[i] = rule;
    }
    (r, s)
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
    // use super::*;

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(&vec![0]), 0);
    // }
}
