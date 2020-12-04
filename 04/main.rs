use aoc;
use std::collections::HashMap;

fn is_valid(p: &HashMap<String, String>) -> bool {
    if p.len() == 8 {
        true
    } else {
        p.len() == 7 && !p.contains_key("cid")
    }
}

fn is_valid_details(p: &HashMap<String, String>) -> bool {
    if !is_valid(p) {
        false
    } else {
        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        // hgt (Height) - a number followed by either cm or in:
        //     If cm, the number must be at least 150 and at most 193.
        //     If in, the number must be at least 59 and at most 76.
        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        // cid (Country ID) - ignored, missing or not.
        for (k, v) in p {
            match k.as_str() {
                "byr" => {
                    let year = v.parse::<i64>().unwrap();
                    if year < 1920 || year > 2002 {
                        return false;
                    }
                }
                "iyr" => {
                    let year = v.parse::<i64>().unwrap();
                    if year < 2010 || year > 2020 {
                        return false;
                    }
                }
                "eyr" => {
                    let year = v.parse::<i64>().unwrap();
                    if year < 2020 || year > 2030 {
                        return false;
                    }
                }
                "hgt" => {
                    if v.ends_with("cm") {
                        let height = v[0..v.len() - 2].parse::<i64>().unwrap();
                        if height < 150 || height > 193 {
                            return false;
                        }
                    } else if v.ends_with("in") {
                        let height = v[0..v.len() - 2].parse::<i64>().unwrap();
                        if height < 59 || height > 76 {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                "hcl" => {
                    if v.len() != 7 || v.chars().nth(0).unwrap() != '#' {
                        return false;
                    }
                    for c in v.chars().skip(1) {
                        if !(c.is_numeric()
                            || c == 'a'
                            || c == 'b'
                            || c == 'c'
                            || c == 'd'
                            || c == 'e'
                            || c == 'f')
                        {
                            return false;
                        }
                    }
                }
                "ecl" => {
                    if !(v == "amb"
                        || v == "blu"
                        || v == "brn"
                        || v == "gry"
                        || v == "grn"
                        || v == "hzl"
                        || v == "oth")
                    {
                        return false;
                    }
                }
                "pid" => {
                    if v.len() != 9 {
                        return false;
                    }
                    if v.chars().filter(|c| !c.is_numeric()).count() > 0 {
                        return false;
                    }
                }
                "cid" => {}
                _ => {
                    return false;
                }
            }
        }
        true
    }
}

fn part1(passports: &Vec<HashMap<String, String>>) -> usize {
    passports.iter().filter(|x| is_valid(x)).count()
}

fn part2(passports: &Vec<HashMap<String, String>>) -> usize {
    passports.iter().filter(|x| is_valid_details(x)).count()
}

fn parse(lines: &[String]) -> Vec<HashMap<String, String>> {
    let mut joined = vec![];
    let mut tmp = vec![];
    for line in lines {
        if line.len() == 0 {
            joined.push(tmp.join(" "));
            tmp.clear();
            continue;
        }
        tmp.push(line.clone());
    }
    if tmp.len() > 0 {
        joined.push(tmp.join(" "));
    }
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
