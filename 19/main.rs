use aoc::ParseError;
use regex::Regex;
use std::collections::HashMap;
use std::iter::*;
use std::str::FromStr;

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
                4 => Ok(Rule::Four(
                    parts[0].parse()?,
                    parts[1].parse()?,
                    parts[2].parse()?,
                    parts[3].parse()?,
                )),

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

type Parsed = (HashMap<usize, Rule>, Vec<String>);
type Answer = usize;

fn expand_rule(rules: &HashMap<usize, Rule>, ix: usize) -> Vec<String> {
    match rules.get(&ix).unwrap() {
        Rule::Char(c) => vec![c.to_string()],
        Rule::One(i) => expand_rule(rules, *i),
        Rule::OneOr(a, b) => {
            let mut v = vec!["(".to_string()];
            v.append(&mut expand_rule(rules, *a));
            v.push("|".to_string());
            v.append(&mut expand_rule(rules, *b));
            v.push(")".to_string());
            v
        }
        Rule::Two(a, b) => {
            let mut v = expand_rule(rules, *a);
            v.append(&mut expand_rule(rules, *b));
            v
        }
        Rule::TwoOr((a, b), (c, d)) => {
            let mut v = vec!["(".to_string()];
            v.append(&mut expand_rule(rules, *a));
            v.append(&mut expand_rule(rules, *b));
            v.push("|".to_string());
            v.append(&mut expand_rule(rules, *c));
            v.append(&mut expand_rule(rules, *d));
            v.push(")".to_string());
            v
        }
        Rule::Four(a, b, c, d) => {
            let mut v = expand_rule(rules, *a);
            v.append(&mut expand_rule(rules, *b));
            v.append(&mut expand_rule(rules, *c));
            v.append(&mut expand_rule(rules, *d));
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

fn part2(input: &Parsed) -> Answer {
    // 0: 8 11
    // 8: 42 | 42 8
    // 11: 42 31 | 42 11 31
    // so (42 42 42) (42 31) or (42 42 42) (42 42 31 31)
    // Which means (42){2,}(31)+
    // But there must be more 42 than 31
    let (rules, strings) = input;
    let s42 = expand_rule(rules, 42).join("");
    let s31 = expand_rule(rules, 31).join("");
    let r42 = Regex::new(&s42).unwrap();
    let r31 = Regex::new(&s31).unwrap();
    let mut n = 0;
    for s in strings {
        let mut ix = 0;
        let mut num42 = 0;
        while let Some(mat) = r42.find_at(s, ix) {
            if mat.start() == ix {
                num42 += 1;
                ix = mat.end();
            } else {
                break;
            }
        }
        let mut num31 = 0;
        while let Some(mat) = r31.find_at(s, ix) {
            if mat.start() == ix {
                num31 += 1;
                ix = mat.end();
            } else {
                break;
            }
        }
        if num42 > num31 && num31 > 0 && ix == s.len() {
            n += 1;
        }
    }
    n
}

fn parse(lines: &[String]) -> Parsed {
    let mut r = HashMap::new();
    let mut s = vec![];
    let mut state = 0;
    for line in lines {
        if line.is_empty() {
            state = 1
        }
        if state == 0 {
            let parts = aoc::split_ch(line, ':');
            r.insert(
                parts[0].parse::<usize>().unwrap(),
                parts[1].parse::<Rule>().unwrap(),
            );
        } else {
            s.push(line.clone());
        }
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
    use super::*;

    #[test]
    fn test_part2() {
        let example_str = include_str!("p2example.txt");
        let example: Vec<_> = example_str
            .split('\n')
            .map(|x| x.trim_end_matches('\n').trim_end_matches('\r').to_string())
            .collect();
        let parsed = parse(&example);
        assert_eq!(part2(&parsed), 12);
    }
}
