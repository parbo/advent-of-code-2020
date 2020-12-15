use std::collections::HashMap;
use std::iter::*;

fn find(numbers: &[i64], ix: usize) -> i64 {
    let mut seen : HashMap<i64, Vec<usize>> = HashMap::new();
    let mut spoken = numbers.to_owned();
    for i in 0..ix {
        if i < numbers.len() {
            seen.insert(spoken[i], vec![i]);
        } else {
            let n = spoken.last().unwrap();
            let x = seen.entry(*n).or_insert(vec![]);
	    let new = if x.len() == 1 {
                0
	    } else {
                (x[x.len() - 1] - x[x.len() - 2]) as i64
	    };
            seen.entry(new).or_insert(vec![]).push(i);
            spoken.push(new);
        };
    }
    *spoken.last().unwrap()
}

fn part1(numbers: &[i64]) -> i64 {
    find(numbers, 2020)
}

fn part2(numbers: &[i64]) -> i64 {
    find(numbers, 30000000)
}

fn parse(lines: &[String]) -> Vec<i64> {
    aoc::split_ch(&lines[0], ',').iter().map(|x| x.parse().unwrap()).collect()
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
        assert_eq!(part1(&vec![0, 3, 6]), 436);
        assert_eq!(part1(&vec![1, 3, 2]), 1);
        assert_eq!(part1(&vec![2, 1, 3]), 10);
        assert_eq!(part1(&vec![1, 2, 3]), 27);
        assert_eq!(part1(&vec![2, 3, 1]), 78);
        assert_eq!(part1(&vec![3, 2, 1]), 438);
        assert_eq!(part1(&vec![3, 1, 2]), 1836);
    }
}
