use aoc::ParseError;
use std::iter::*;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Rule {
    thing: String,
    low: (i64, i64),
    high: (i64, i64),
}

impl Rule {
    fn is_valid(&self, val: i64) -> bool {
        (val >= self.low.0 && val <= self.low.1) || (val >= self.high.0 && val <= self.high.1)
    }
}

impl FromStr for Rule {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = aoc::split_ch(s, ':');
        if parts.len() == 2 {
            let ranges: Vec<_> = aoc::split_str(parts[1], "or")
                .iter()
                .map(|x| aoc::split_ch(x, '-'))
                .collect();
            if ranges.len() == 2 && ranges[0].len() == 2 && ranges[1].len() == 2 {
                return Ok(Rule {
                    thing: parts[0].to_string(),
                    low: (ranges[0][0].parse()?, ranges[0][1].parse()?),
                    high: (ranges[1][0].parse()?, ranges[1][1].parse()?),
                });
            }
        }
        Err(ParseError::Generic)
    }
}

fn part1(input: &(Vec<Rule>, Vec<i64>, Vec<Vec<i64>>)) -> i64 {
    let (rules, _ticket, nearby) = input;
    let mut invalid = vec![];
    for ticket in nearby {
        for &val in ticket {
            let mut any_valid = false;
            for rule in rules {
                if rule.is_valid(val) {
                    any_valid = true;
                    break;
                }
            }
            if !any_valid {
                invalid.push(val);
            }
        }
    }
    invalid.iter().sum()
}

fn is_valid(ticket: &[i64], rules: &[Rule]) -> bool {
    for &val in ticket {
        let mut any_valid = false;
        for rule in rules {
            if rule.is_valid(val) {
                any_valid = true;
                break;
            }
        }
        if !any_valid {
            return false;
        }
    }
    return true;
}

fn dfs(g: &[(usize, Vec<Rule>)], ix: usize, l: usize, sofar: &[(usize, Rule)]) -> Vec<(usize, Rule)> {
    if ix == l {
        return sofar.to_vec();
    }
    let (i, rules) = &g[ix];
    'outer: for r in rules {
        for sf in sofar {
            if *r == sf.1 {
                continue 'outer;
            }
        }
        let mut s = sofar.to_owned();
        s.push((*i, r.clone()));
        let v = dfs(g, ix + 1, l, &s);
        if !v.is_empty() {
            return v;
        }
    }
    vec![]
}

fn find_rules(input: &(Vec<Rule>, Vec<i64>, Vec<Vec<i64>>)) -> Vec<Rule> {
    let (rules, ticket, nearby) = input;
    let valid: Vec<_> = nearby.iter().filter(|x| is_valid(x, rules)).collect();
    let mut valid_rules = vec![];
    for i in 0..ticket.len() {
	let mut vr = vec![];
        'rule: for rule in rules {
            for t in &valid {
                if !rule.is_valid(t[i]) {
		    continue 'rule;
                }
            }
	    vr.push(rule.clone());
        }
	valid_rules.push((i, vr));
    }
    // Sort by most constrained
    valid_rules.sort_by(|a, b| a.1.len().cmp(&b.1.len()));
    let mut r = dfs(&valid_rules, 0, ticket.len(), &vec![]);
    // Then sort it back to ordered again and remove the indices
    r.sort_by(|a, b| a.0.cmp(&b.0));
    r.into_iter().map(|(_, x)| x).collect()
}

fn part2(input: &(Vec<Rule>, Vec<i64>, Vec<Vec<i64>>)) -> i64 {
    let (_, ticket, _) = input;
    let r = find_rules(input);
    let mut ans = vec![];
    for (i, x) in r.iter().enumerate() {
        if x.thing.starts_with("departure") {
            ans.push(ticket[i]);
        }
    }
    ans.iter().product()
}

fn parse(lines: &[String]) -> (Vec<Rule>, Vec<i64>, Vec<Vec<i64>>) {
    let mut rules = vec![];
    let mut ticket = vec![];
    let mut nearby = vec![];
    let mut state = 0;
    for line in lines {
        match state {
            0 => {
                if line == "your ticket:" {
                    state = 1
                } else if line != "" {
                    let rule: Rule = line.parse().unwrap();
                    rules.push(rule);
                }
            }
            1 => {
                if line == "nearby tickets:" {
                    state = 2
                } else if line != "" {
                    ticket = aoc::split_ch(line, ',')
                        .iter()
                        .map(|x| x.parse().unwrap())
                        .collect();
                }
            }
            2 => {
                let t = aoc::split_ch(line, ',')
                    .iter()
                    .map(|x| x.parse().unwrap())
                    .collect();
                nearby.push(t);
            }
            _ => panic!(),
        }
    }
    (rules, ticket, nearby)
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
    fn test_part1() {
        let input = vec![
            "class: 1-3 or 5-7".to_string(),
            "row: 6-11 or 33-44".to_string(),
            "seat: 13-40 or 45-50".to_string(),
            "".to_string(),
            "your ticket:".to_string(),
            "7,1,14".to_string(),
            "".to_string(),
            "nearby tickets:".to_string(),
            "7,3,47".to_string(),
            "40,4,50".to_string(),
            "55,2,20".to_string(),
            "38,6,12".to_string(),
        ];
        let parsed = parse(&input);
        assert_eq!(part1(&parsed), 71);
    }

    #[test]
    fn test_part2() {
        let input = vec![
            "class: 0-1 or 4-19".to_string(),
            "row: 0-5 or 8-19".to_string(),
            "seat: 0-13 or 16-19".to_string(),
            "".to_string(),
            "your ticket:".to_string(),
            "11,12,13".to_string(),
            "".to_string(),
            "nearby tickets:".to_string(),
            "3,9,18".to_string(),
            "15,1,5".to_string(),
            "5,14,9".to_string(),
        ];
        let parsed = parse(&input);
        assert_eq!(
            find_rules(&parsed),
            vec![
                Rule {
                    thing: "row".to_string(),
                    low: (0, 5),
                    high: (8, 19)
                },
                Rule {
                    thing: "class".to_string(),
                    low: (0, 1),
                    high: (4, 19)
                },
                Rule {
                    thing: "seat".to_string(),
                    low: (0, 13),
                    high: (16, 19)
                }
            ]
        );
    }
}
