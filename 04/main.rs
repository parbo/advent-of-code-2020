use aoc;
use aoc::Itertools;
use std::collections::HashMap;

fn is_valid(p: &HashMap<String, String>) -> bool {
    if p.len() == 8 {
        true
    } else {
        p.len() == 7 && !p.contains_key("cid")
    }
}

fn is_valid_details(p: &HashMap<String, String>) -> bool {
    is_valid(p)
        && p.iter().all(|(k, v)| match k.as_str() {
            "byr" => v
                .parse::<usize>()
                .map_or(false, |year| year >= 1920 && year <= 2002),
            "iyr" => v
                .parse::<usize>()
                .map_or(false, |year| year >= 2010 && year <= 2020),
            "eyr" => v
                .parse::<usize>()
                .map_or(false, |year| year >= 2020 && year <= 2030),
            "hgt" => {
                if v.ends_with("cm") {
                    v[0..v.len() - 2]
                        .parse::<usize>()
                        .map_or(false, |height| height >= 150 && height <= 193)
                } else if v.ends_with("in") {
                    v[0..v.len() - 2]
                        .parse::<usize>()
                        .map_or(false, |height| height >= 59 && height <= 76)
                } else {
                    false
                }
            }
            "hcl" => {
                v.len() == 7
                    && v.chars().nth(0).unwrap() == '#'
                    && v.chars()
                        .skip(1)
                        .filter(|&c| {
                            c.is_numeric()
                                || c == 'a'
                                || c == 'b'
                                || c == 'c'
                                || c == 'd'
                                || c == 'e'
                                || c == 'f'
                        })
                        .count()
                        == 6
            }
            "ecl" => {
                v == "amb"
                    || v == "blu"
                    || v == "brn"
                    || v == "gry"
                    || v == "grn"
                    || v == "hzl"
                    || v == "oth"
            }
            "pid" => v.chars().filter(|c| c.is_numeric()).count() == 9,
            "cid" => true,
            _ => false,
        })
}

fn part1(passports: &Vec<HashMap<String, String>>) -> usize {
    passports.iter().filter(|x| is_valid(x)).count()
}

fn part2(passports: &Vec<HashMap<String, String>>) -> usize {
    passports.iter().filter(|x| is_valid_details(x)).count()
}

fn parse(lines: &[String]) -> Vec<HashMap<String, String>> {
    let joined: Vec<_> = lines
        .iter()
        .group_by(|line| line.len() > 0)
        .into_iter()
        .map(|(_key, mut group)| group.join(" "))
        .filter(|s| s.len() > 0)
        .collect();
    let mut passports = vec![];
    for passport in joined {
        let mut map = HashMap::new();
        let parts = aoc::split(&passport, |c| c == ' ');
        for part in parts {
            let thing = aoc::split(&part, |c| c == ':');
            map.insert(thing[0].to_string(), thing[1].to_string());
        }
        passports.push(map);
    }
    passports
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
