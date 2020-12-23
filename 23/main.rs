use std::collections::VecDeque;
use std::iter::*;

type Parsed = VecDeque<i64>;
type Answer = String;

fn rounds(cups: &Parsed, num: usize, total: usize) -> Parsed {
    let mut cups = cups.clone();
    cups.reserve(total);
    let min = *cups.iter().min().unwrap();
    let mut x = *cups.iter().max().unwrap() + 1;
    while cups.len() < total {
        cups.push_back(x);
        x += 1;
    }
    // Compute new max
    let max = *cups.iter().max().unwrap();
    println!("{}, {}, {}", min, max, cups.len());
    let mut rev_index = vec![];
    rev_index.resize(total + 1, 0);
    for (i, c) in cups.iter().enumerate() {
        rev_index[*c as usize] = i;
    }
    for i in 0..num {
        let mut next = cups[0] - 1;
        if i % 10000 == 0 && cups.len() > 20 {
            println!("round: {}", i);
            println!("next: {}", next);
            println!("> {:?}", cups.iter().take(20).collect::<Vec<_>>());
            println!(
                "< {:?}",
                cups.iter().skip(cups.len() - 20).collect::<Vec<_>>()
            );
        }
        cups.rotate_left(1);
        let pickup = vec![cups[0], cups[1], cups[2]];
	cups.drain(0..3);
        let (mut ix, mix) = 'outer: loop {
            if next < min {
                next = max;
            }
            'next: loop {
                for p in &pickup {
                    if *p == next {
                        next -= 1;
                        if next < min {
                            next = max;
                        }
                        continue 'next;
                    }
                }
                break;
            }
            let mut maybe_ix = rev_index[next as usize];
            if maybe_ix >= cups.len() {
                maybe_ix = 0;
            }
            //	    println!("maybe ix: {}", maybe_ix);
            let mut a = maybe_ix;
            let mut b = maybe_ix;
            loop {
                let c = cups[a];
                if c == next {
                    break 'outer (a, maybe_ix);
                }
                let c = cups[b];
                if c == next {
                    break 'outer (b, maybe_ix);
                }
                if a + 1 < cups.len() {
                    a += 1;
                } else {
                    a = 0;
                }
                if b > 0 {
                    b -= 1;
                } else {
                    b = cups.len() - 1;
                }
                if a == maybe_ix || b == maybe_ix {
                    break;
                }
            }
            println!("{:?}, {}, {}, {}, {}", pickup, next, maybe_ix, a, b);
            panic!();
        };
        let a = ix.max(mix) as i64;
        let b = ix.min(mix) as i64;
        let dist = (a - b).min(cups.len() as i64 - a + b);
        if dist > 10000 {
            // Rebuild the revi index
            println!("rebuild {} {} {}", ix, mix, dist);
            for (i, c) in cups.iter().enumerate() {
                rev_index[*c as usize] = i;
            }
        }
        //	println!("insert at {}", ix);
        for p in pickup {
            ix += 1;
            cups.insert(ix, p);
            rev_index[p as usize] = ix;
        }
    }
    while cups[0] != 1 {
        cups.rotate_left(1);
    }
    if cups.len() > 20 {
        println!("> {:?}", cups.iter().take(20).collect::<Vec<_>>());
        println!(
            "< {:?}",
            cups.iter().skip(cups.len() - 20).collect::<Vec<_>>()
        );
    }
    cups
}

fn part1(cups: &Parsed) -> Answer {
    let c = rounds(cups, 100, cups.len());
    c.iter()
        .skip(1)
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join("")
}

fn part2(cups: &Parsed) -> i64 {
    let c = rounds(cups, 10000000, 1000000);
    c.into_iter().skip(1).take(2).product()
}

fn parse(lines: &[String]) -> Parsed {
    lines[0]
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i64)
        .collect()
}

fn main() {
    let (part, lines) = aoc::read_lines();
    let parsed = parse(&lines);
    if part == 1 {
        let result = part1(&parsed);
        println!("{}", result);
    } else {
        let result = part2(&parsed);
        println!("{}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec!["389125467".to_string()];
        let parsed = parse(&input);
        assert_eq!(
            rounds(&parsed, 10, parsed.len()),
            [1, 9, 2, 6, 5, 8, 3, 7, 4,]
        );
        assert_eq!(part1(&parsed), "67384529");
    }
}
