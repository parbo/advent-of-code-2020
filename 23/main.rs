use std::iter::*;

type Parsed = Vec<i32>;
type Answer = String;

fn get_values(ll: &[i32]) -> Vec<i32> {
    let mut s = vec![1];
    s.reserve(ll.len());
    let mut next = ll[1];
    while next != 1 {
        s.push(next);
        next = ll[next as usize];
    }
    s
}

fn find_next(node: i32, pickup: &[i32;3], total: usize) -> i32 {
    let mut next = node;
    if next > 1 {
        next -= 1;
    } else {
        next = total as i32;
    }
    'outer: loop {
        for c in pickup {
            if *c == next {
                if next > 1 {
                    next -= 1;
                } else {
                    next = total as i32;
                }
                continue 'outer;
            }
        }
        break;
    }
    next
}

fn rounds(cups: &Parsed, num: usize, total: usize) -> Vec<i32> {
    // Compute min/max
    let mut get_next: Vec<i32> = Vec::with_capacity(total + 1);
    get_next.resize(total + 1, 0);
    for i in 0..cups.len() {
        get_next[cups[i] as usize] = cups[(i + 1) % cups.len()];
    }
    for i in cups.len()..total {
        get_next[i] = (i + 1) as i32;
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
	let next = find_next(node, &pickup, total);

        // insert the picked up items at next
        let old = get_next[next as usize];
        get_next[next as usize] = get_next[node as usize];
        get_next[last_picked_up as usize] = old;
        // close the gap
        get_next[node as usize] = remaining;
        // Move cw
        node = remaining;
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
    a as i64 * b as i64
}

fn parse(lines: &[String]) -> Parsed {
    lines[0]
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
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
