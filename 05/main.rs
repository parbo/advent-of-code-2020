use std::iter::*;

fn find_seat(bp: &[char]) -> (i64, i64) {
    let mut start = 0;
    let mut end = 128;
    for item in bp.iter().take(7) {
        match item {
            'F' => end -= (end - start) / 2,
            'B' => start += (end - start) / 2,
            _ => panic!(),
        }
    }
    let mut cstart = 0;
    let mut cend = 8;
    for item in bp.iter().skip(7).take(3) {
        match item {
            'L' => cend -= (cend - cstart) / 2,
            'R' => cstart += (cend - cstart) / 2,
            _ => panic!(),
        }
    }
    (start, cstart)
}

fn part1(bps: &[Vec<char>]) -> i64 {
    bps.iter()
        .map(|bp| {
            let seat = find_seat(&bp);
            seat.0 * 8 + seat.1
        })
        .max()
        .unwrap()
}

fn part2(bps: &[Vec<char>]) -> i64 {
    let mut ids: Vec<_> = bps
        .iter()
        .map(|bp| {
            let seat = find_seat(&bp);
            seat.0 * 8 + seat.1
        })
        .collect();
    ids.sort_unstable();
    let mut last = None;
    for id in &ids {
        if let Some(l) = last {
            if *id != l + 1 {
                return *id - 1;
            }
        }
        last = Some(id);
    }
    0
}

fn parse(lines: &[String]) -> Vec<Vec<char>> {
    lines.iter().map(|x| x.chars().collect()).collect()
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
    fn test_seat() {
        let a: Vec<_> = "BFFFBBFRRR".chars().collect();
        let b: Vec<_> = "FFFBBBFRRR".chars().collect();
        let c: Vec<_> = "BBFFBBFRLL".chars().collect();
        let d: Vec<_> = "FBFBBFFRLR".chars().collect();
        assert_eq!(find_seat(&d), (44, 5)); // 357
        assert_eq!(find_seat(&a), (70, 7)); // 567
        assert_eq!(find_seat(&b), (14, 7)); // 119
        assert_eq!(find_seat(&c), (102, 4)); // 820
    }
}
