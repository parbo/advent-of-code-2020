use aoc::*;
use std::iter::*;

fn part1(moves: &[(char, i64)]) -> i64 {
    let mut curr = [0, 0];
    let mut facing = aoc::EAST;
    for (d, steps) in moves {
        match d {
            'E' => curr = point_add(curr, point_mul(aoc::EAST, *steps)),
            'W' => curr = point_add(curr, point_mul(aoc::WEST, *steps)),
            'N' => curr = point_add(curr, point_mul(aoc::NORTH, *steps)),
            'S' => curr = point_add(curr, point_mul(aoc::SOUTH, *steps)),
            'F' => curr = point_add(curr, point_mul(facing, *steps)),
            'L' => {
                for _ in (0..*steps).step_by(90) {
                    facing = [facing[1], -facing[0]]
                }
            }
            'R' => {
                for _ in (0..*steps).step_by(90) {
                    facing = [-facing[1], facing[0]]
                }
            }
            _ => panic!(),
        }
    }
    curr[0].abs() + curr[1].abs()
}

fn part2(moves: &[(char, i64)]) -> i64 {
    let mut waypoint = [10, -1];
    let mut ship = [0, 0];
    for (d, steps) in moves {
        match d {
            'E' => waypoint = point_add(waypoint, point_mul(aoc::EAST, *steps)),
            'W' => waypoint = point_add(waypoint, point_mul(aoc::WEST, *steps)),
            'N' => waypoint = point_add(waypoint, point_mul(aoc::NORTH, *steps)),
            'S' => waypoint = point_add(waypoint, point_mul(aoc::SOUTH, *steps)),
            'F' => ship = point_add(ship, point_mul(waypoint, *steps)),
            'L' => {
                for _ in (0..*steps).step_by(90) {
                    waypoint = [waypoint[1], -waypoint[0]]
                }
            }
            'R' => {
                for _ in (0..*steps).step_by(90) {
                    waypoint = [-waypoint[1], waypoint[0]]
                }
            }
            _ => panic!(),
        }
    }
    ship[0].abs() + ship[1].abs()
}

fn parse(lines: &[String]) -> Vec<(char, i64)> {
    lines
        .iter()
        .map(|x| (x.chars().next().unwrap(), x[1..].parse().unwrap()))
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
    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&vec![('F', 10), ('N', 3), ('F', 7), ('R', 90), ('F', 11)]),
            286
        );
    }
}
