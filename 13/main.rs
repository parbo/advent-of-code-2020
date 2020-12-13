use std::iter::*;

fn part1(tt: &(usize, Vec<(usize, usize)>)) -> usize {
    let (bus, departure) =
        tt.1.iter()
            .map(|(_, v)| (v, v * (tt.0 / v + 1)))
            .min_by(|(_, va), (_, vb)| va.cmp(vb))
            .unwrap();
    (departure - tt.0) * bus
}

fn find_ix(a: usize, b: usize, align: usize) -> (usize, usize) {
    let mut i = 0;
    loop {
        if b * ((i / b) + 1) == i + align {
            return (i, a * b);
        }
        i += a;
    }
}

fn part2(tt: &(usize, Vec<(usize, usize)>)) -> usize {
    let mut b = tt.1.clone();
    loop {
        if b.len() == 1 {
            break;
        }
        let mut x = vec![];
        for i in (0..b.len()).step_by(2) {
            if i + 1 < b.len() {
                let (offs0, bus0) = b[i];
                let (offs1, bus1) = b[i + 1];
                let (offs, period) = find_ix(bus0, bus1, offs1 - offs0);
                x.push((period - offs + offs0, period));
            } else {
                x.push(b[i]);
            }
        }
        x.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        b = x.clone();
    }
    b[0].1 - b[0].0
}

fn parse(lines: &[String]) -> (usize, Vec<(usize, usize)>) {
    (
        lines[0].parse().unwrap(),
        aoc::split_ch(&lines[1], ',')
            .iter()
            .enumerate()
            .filter(|(_, x)| **x != "x")
            .map(|(i, x)| (i, x.parse().unwrap()))
            .collect(),
    )
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
        // 7,13,x,x,59,x,31,19
        let tt = (939, vec![(0, 7), (1, 13), (4, 59), (6, 31), (7, 19)]);
        assert_eq!(part1(&tt), 295);
    }

    #[test]
    fn test_part2() {
        let tt0 = (939, vec![(0, 17), (2, 13), (3, 19)]);
        assert_eq!(part2(&tt0), 3417);

        let tt1 = (939, vec![(0, 7), (1, 13), (4, 59), (6, 31), (7, 19)]);
        assert_eq!(part2(&tt1), 1068781);
    }
}
