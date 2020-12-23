use std::iter::*;

type Parsed = Vec<i64>;
type Answer = String;

fn get_values(ll: &[i64]) -> Vec<i64> {
    let mut s = vec![1];
    s.reserve(ll.len());
    let mut next = ll[1];
    while next != 1 {
        s.push(next);
        next = ll[next as usize];
    }
    s
}

fn rounds(cups: &Parsed, num: usize, total: usize) -> Vec<i64> {
    // Compute min/max
    let mut get_next: Vec<i64> = Vec::with_capacity(total + 1);
    get_next.resize(total + 1, 0);
    for i in 0..cups.len() {
        get_next[cups[i] as usize] = cups[(i + 1) % cups.len()];
    }
    for i in cups.len()..total {
        get_next[i] = (i + 1) as i64;
    }
    if total > cups.len() {
        get_next[total] = cups[0];
    }
    let mut node = cups[0];
    for _i in 0..num {
        // Pick up three values to the right of current node
        let mut last_picked_up = get_next[node as usize];
        let mut pickup = [0; 3];
        pickup[0] = last_picked_up;
        last_picked_up = get_next[last_picked_up as usize];
        pickup[1] = last_picked_up;
        last_picked_up = get_next[last_picked_up as usize];
        pickup[2] = last_picked_up;
        let remaining = get_next[last_picked_up as usize];

        // Find the next _value_
        let mut next = node as i64;
        if next > 1 {
            next -= 1;
        } else {
            next = total as i64;
        }
        'outer: loop {
            for c in &pickup {
                if *c == next {
                    if next > 1 {
                        next -= 1;
                    } else {
                        next = total as i64;
                    }
                    continue 'outer;
                }
            }
            break;
        }
        // insert the picked up items at next
        let old = get_next[next as usize];
        get_next[next as usize] = get_next[node as usize];
        get_next[last_picked_up as usize] = old;
        // close the gap
        get_next[node as usize] = remaining;
        // Move cw
        node = get_next[node as usize];
    }
    get_next
}

fn part1(cups: &Parsed) -> Answer {
    let c = rounds(&cups, 100, cups.len());
    get_values(&c)
        .iter()
        .skip(1)
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("")
}

fn part2(cups: &Parsed) -> i64 {
    let c = rounds(&cups, 10000000, 1000000);
    let a = c[1];
    let b = c[a as usize];
    a * b
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
        let ll = rounds(&parsed, 10, parsed.len());
        println!("ll: {:?}", ll);
        let s = get_values(&ll);
        println!("s: {:?}", s);
        assert_eq!(s, [1, 9, 2, 6, 5, 8, 3, 7, 4,]);
        assert_eq!(part1(&parsed), "67384529");
    }
}
