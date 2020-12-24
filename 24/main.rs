use aoc::Vec3;
use std::collections::HashMap;
use std::iter::*;

const HEX_E: Vec3 = [1, -1, 0];
const HEX_W: Vec3 = [-1, 1, 0];
const HEX_SE: Vec3 = [0, -1, 1];
const HEX_SW: Vec3 = [-1, 0, 1];
const HEX_NW: Vec3 = [0, 1, -1];
const HEX_NE: Vec3 = [1, 0, -1];

type Parsed = Vec<Vec<Vec3>>;
type Answer = usize;

fn init(paths: &Parsed) -> HashMap<Vec3, char> {
    let mut g = HashMap::new();
    for path in paths {
        let mut coord = [0, 0, 0];
        for d in path {
            coord = aoc::vec_add(coord, *d);
        }
	let col = g.entry(coord).or_insert('.');
	if *col == '.' {
	    *col = 'B';
	} else {
	    *col = '.';
	}
    }
    g
}

fn part1(paths: &Parsed) -> Answer {
    let tiles = init(paths);
    tiles.iter().filter(|(_coord, col)| **col == 'B').count()
}

fn extents(g: &HashMap<Vec3, char>) -> (Vec3, Vec3) {
    let min_x = g.iter().map(|(p, _v)| p[0]).min().unwrap();
    let min_y = g.iter().map(|(p, _v)| p[1]).min().unwrap();
    let min_z = g.iter().map(|(p, _v)| p[2]).min().unwrap();
    let max_x = g.iter().map(|(p, _v)| p[0]).max().unwrap();
    let max_y = g.iter().map(|(p, _v)| p[1]).max().unwrap();
    let max_z = g.iter().map(|(p, _v)| p[2]).max().unwrap();
    ([min_x, min_y, min_z], [max_x, max_y, max_z])
}

fn part2(paths: &Parsed) -> Answer {
    let mut g = HashMap::new();
    // init tiles to white
    for path in paths {
        let mut coord = [0, 0, 0];
        for d in path {
            coord = aoc::vec_add(coord, *d);
        }
	let col = g.entry(coord).or_insert('.');
	if *col == '.' {
	    *col = 'B';
	} else {
	    *col = '.';
	}
    }
    let d =  [HEX_E, HEX_W, HEX_SW, HEX_SE, HEX_NW, HEX_NE];
    for _ in 0..100 {
        let mut newg = g.clone();
        let ([min_x, min_y, min_z], [max_x, max_y, max_z]) = extents(&newg);
        for z in (min_z - 1)..=(max_z + 1) {
            for y in (min_y - 1)..=(max_y + 1) {
                for x in (min_x - 1)..=(max_x + 1) {
                    let p = [x, y, z];
                    let mut black = 0;
                    let c = g.get(&p).unwrap_or(&'.');
                    for dir in &d {
                        let np = aoc::vec_add(p, *dir);
                        match g.get(&np) {
                            Some('B') => {
                                black += 1;
                            }
                            _ => {}
                        }
                    }
                    if *c == 'B' && (black == 0 || black > 2) {
                        newg.remove(&p);
                    } else if *c == '.' && black == 2 {
                        newg.insert(p, 'B');
                    }
                }
            }
        }
	g = newg.clone();
    }
    g.iter().filter(|(_coord, c)| **c == 'B').count()
}

fn parse(lines: &[String]) -> Parsed {
    let mut paths = vec![];
    for line in lines {
        let mut path = vec![];
        let mut cit = line.chars();
        while let Some(c) = cit.next() {
            match c {
                's' => {
                    if let Some(cc) = cit.next() {
                        match cc {
                            'e' => path.push(HEX_SE),
                            'w' => path.push(HEX_SW),
                            _ => panic!(),
                        }
                    }
                }
                'n' => {
                    if let Some(cc) = cit.next() {
                        match cc {
                            'e' => path.push(HEX_NE),
                            'w' => path.push(HEX_NW),
                            _ => panic!(),
                        }
                    }
                }
                'e' => path.push(HEX_E),
                'w' => path.push(HEX_W),
                _ => panic!(),
            }
        }
        paths.push(path);
    }
    paths
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
        let input = vec![
            "sesenwnenenewseeswwswswwnenewsewsw".to_string(),
            "neeenesenwnwwswnenewnwwsewnenwseswesw".to_string(),
            "seswneswswsenwwnwse".to_string(),
            "nwnwneseeswswnenewneswwnewseswneseene".to_string(),
            "swweswneswnenwsewnwneneseenw".to_string(),
            "eesenwseswswnenwswnwnwsewwnwsene".to_string(),
            "sewnenenenesenwsewnenwwwse".to_string(),
            "wenwwweseeeweswwwnwwe".to_string(),
            "wsweesenenewnwwnwsenewsenwwsesesenwne".to_string(),
            "neeswseenwwswnwswswnw".to_string(),
            "nenwswwsewswnenenewsenwsenwnesesenew".to_string(),
            "enewnwewneswsewnwswenweswnenwsenwsw".to_string(),
            "sweneswneswneneenwnewenewwneswswnese".to_string(),
            "swwesenesewenwneswnwwneseswwne".to_string(),
            "enesenwswwswneneswsenwnewswseenwsese".to_string(),
            "wnwnesenesenenwwnenwsewesewsesesew".to_string(),
            "nenewswnwewswnenesenwnesewesw".to_string(),
            "eneswnwswnwsenenwnwnwwseeswneewsenese".to_string(),
            "neswnwewnwnwseenwseesewsenwsweewe".to_string(),
            "wseweeenwnesenwwwswnew".to_string(),
        ];
        let parsed = parse(&input);
        assert_eq!(part1(&parsed), 10);
    }

    #[test]
    fn test_part2() {
        let input = vec![
            "sesenwnenenewseeswwswswwnenewsewsw".to_string(),
            "neeenesenwnwwswnenewnwwsewnenwseswesw".to_string(),
            "seswneswswsenwwnwse".to_string(),
            "nwnwneseeswswnenewneswwnewseswneseene".to_string(),
            "swweswneswnenwsewnwneneseenw".to_string(),
            "eesenwseswswnenwswnwnwsewwnwsene".to_string(),
            "sewnenenenesenwsewnenwwwse".to_string(),
            "wenwwweseeeweswwwnwwe".to_string(),
            "wsweesenenewnwwnwsenewsenwwsesesenwne".to_string(),
            "neeswseenwwswnwswswnw".to_string(),
            "nenwswwsewswnenenewsenwsenwnesesenew".to_string(),
            "enewnwewneswsewnwswenweswnenwsenwsw".to_string(),
            "sweneswneswneneenwnewenewwneswswnese".to_string(),
            "swwesenesewenwneswnwwneseswwne".to_string(),
            "enesenwswwswneneswsenwnewswseenwsese".to_string(),
            "wnwnesenesenenwwnenwsewesewsesesew".to_string(),
            "nenewswnwewswnenesenwnesewesw".to_string(),
            "eneswnwswnwsenenwnwnwwseeswneewsenese".to_string(),
            "neswnwewnwnwseenwseesewsenwsweewe".to_string(),
            "wseweeenwnesenwwwswnew".to_string(),
        ];
        let parsed = parse(&input);
        assert_eq!(part2(&parsed), 2208);
    }
}
