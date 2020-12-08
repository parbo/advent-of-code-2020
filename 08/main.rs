use std::iter::*;
use std::collections::HashSet;

fn run(program: &[(String, i64)]) -> (bool, i64) {
    let mut accumulator = 0;
    let mut pc : i64 = 0;
    let mut seen_pc = HashSet::new();
    loop {
	if !seen_pc.insert(pc) {
	    break;
	}
	if pc < 0 || pc as usize >= program.len() {
	    break;
	}
	let (op, arg) = &program[pc as usize];
	match op.as_str() {
	    "nop" => pc += 1,
	    "acc" => { accumulator += arg; pc += 1; }
	    "jmp" => pc += arg,
	    _ => panic!(),
	}
    }
    (pc as usize == program.len(), accumulator)
}

fn part1(program: &[(String, i64)]) -> i64 {
    run(program).1
}

fn part2(program: &[(String, i64)]) -> i64 {
    let l = program.len();
    for m in 0..l {
	let mut p: Vec<_> = program.to_vec();
	let (op, arg) = &p[m];
	match op.as_str() {
	    "nop" => p[m] = ("jmp".to_string(), *arg),
	    "jmp" => p[m] = ("nop".to_string(), *arg),
	    _ => {}
	}
	let (terminated, acc) = run(&p);
	if terminated {
	    return acc
	}
    }
    0
}

fn parse(lines: &[String]) -> Vec<(String, i64)> {
    lines
        .iter()
        .map(|x| {
            let parts = aoc::split(x, |c| c == ' ');
            (parts[0].to_string(), parts[1].parse().unwrap())
        })
        .collect()
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
