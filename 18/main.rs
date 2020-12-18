use std::iter::*;

type Parsed = Vec<Vec<String>>;
type Answer = i64;

fn calc(s: &[String], a: i64) -> (i64, usize) {
    println!("{}, {:?}", a, s);
    let mut op1 = a;
    let mut op = None;
    let mut i = 0;
    while i < s.len() {
        let o = s[i].as_str();
        println!("o: {}, op1: {}, op: {:?}", o, op1, op);
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
    println!("res = {}", op1);
    (op1, i)
}

fn part1(input: &Parsed) -> Answer {
    input.iter().map(|x| calc(x, 0).0).sum()
}

fn part2(_: &Parsed) -> Answer {
    0
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
        let parsed1 = parse_line("1 + (2 * 3) + (4 * (5 + 6))");
        assert_eq!(calc(&parsed1, 0), (51, parsed1.len()));
        let parsed2 = parse_line("2 * 3 + (4 * 5)");
        assert_eq!(calc(&parsed2, 0), (26, parsed2.len()));
        let parsed3 = parse_line("5 + (8 * 3 + 9 + 3 * 4 * 3)");
        assert_eq!(calc(&parsed3, 0), (437, parsed3.len()));
        let parsed4 = parse_line("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");
        assert_eq!(calc(&parsed4, 0), (12240, parsed4.len()));
        let parsed5 = parse_line("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
        assert_eq!(calc(&parsed5, 0), (13632, parsed5.len()));
    }
}
