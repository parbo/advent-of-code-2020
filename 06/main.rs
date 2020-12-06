use aoc::Itertools;
use std::collections::HashMap;
use std::iter::*;

fn part1(groups: &[(usize, HashMap<char, usize>)]) -> usize {
    groups.iter().map(|(_c, m)| m.len()).sum()
}

fn part2(groups: &[(usize, HashMap<char, usize>)]) -> usize {
    groups
        .iter()
        .map(|(c, m)| m.iter().filter(|(_k, v)| *v == c).count())
        .sum()
}

fn parse(lines: &[String]) -> Vec<(usize, HashMap<char, usize>)> {
    let groups: Vec<_> = lines
        .iter()
        .group_by(|line| !line.is_empty())
        .into_iter()
        .map(|(_key, group)| -> Vec<_> { group.collect() })
        .collect();
    let mut ret = vec![];
    for group in groups {
        let mut q = HashMap::new();
        let num = group.len();
        for person in group {
            for answer in person.chars() {
                *q.entry(answer).or_insert(0) += 1;
            }
        }
        ret.push((num, q));
    }
    ret
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
        let example: Vec<String> = vec![
            "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b",
        ]
        .into_iter()
        .map(|x| x.to_string())
        .collect();

        let parsed = parse(&example);
        println!("{:?}", parsed);

        assert_eq!(part1(&parsed), 11);
        assert_eq!(part2(&parsed), 6);
    }
}
