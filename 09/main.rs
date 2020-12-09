use std::iter::*;

fn is_valid(input: &[i64], num: usize, ix: usize) -> bool {
    for j in 0..num {
        for k in 0..num {
	    if j == k {
                continue;
	    }
	    if input[ix + j - num] + input[ix + k - num] == input[ix] {
                return true;
	    }
        }
    }
    false
}

fn get_first_non_valid(input: &[i64], pl: usize) -> i64 {
    for i in pl..input.len() {
        if !is_valid(input, pl, i) {
            return input[i];
        }
    }
    -1
}

fn part1(input: &[i64]) -> i64 {
    get_first_non_valid(input, 25)
}

fn part2(input: &[i64]) -> i64 {
    let num = part1(input);
    for i in 0..input.len() {
	for j in i..input.len() {
	    let s : i64 = input[i..j].iter().sum();
	    if s == num {
		let min = input[i..j].iter().min().unwrap();
		let max = input[i..j].iter().max().unwrap();
		return min + max;
	    }
	}
    }
    0
}

fn parse(lines: &[String]) -> Vec<i64> {
    lines.iter().map(|x| x.parse().unwrap()).collect()
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
        let input: Vec<i64> = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(get_first_non_valid(&input, 5), 127);
    }
}
