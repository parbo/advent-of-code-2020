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

fn cube_to_oddr(cube: Vec3) -> aoc::Point {
    let col = 2 * cube[0] + (cube[2] - (cube[2].rem_euclid(2)));
    let row = cube[2];
    [col, row]
}

fn draw(gg: &HashMap<Vec3, char>) {
    let mut g = HashMap::new();
    for (p, v) in gg {
        g.insert(cube_to_oddr(*p), *v);
    }
    println!("=============================================");
    let min_x = g.iter().map(|(p, _v)| p[0]).min().unwrap();
    let min_y = g.iter().map(|(p, _v)| p[1]).min().unwrap();
    let max_x = g.iter().map(|(p, _v)| p[0]).max().unwrap();
    let max_y = g.iter().map(|(p, _v)| p[1]).max().unwrap();
    for y in min_y..=max_y {
        if y.rem_euclid(2) == 0 {
            for _ in min_x..=max_x {
                print!("\\/");
            }
            println!();
        }
        if y.rem_euclid(2) == 0 {
            print!(" ");
        }
        for x in min_x..=max_x {
            let p = [x, y];
            let c = g.get(&p).unwrap_or(&'.');
            print!("|{}", c);
        }
        if y.rem_euclid(2) == 0 {
            println!();
            for _ in min_x..=max_x {
                print!("/\\");
            }
        }
        println!();
    }
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
    let mut all_grid = vec![g.clone()];
    let d = [HEX_E, HEX_W, HEX_SW, HEX_SE, HEX_NW, HEX_NE];
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
        all_grid.push(g.clone());
    }
    // Draw all the grids, using the same coord system
    let mut min_xx = 0;
    let mut min_yy = 0;
    let mut min_zz = 0;
    let mut max_xx = 0;
    let mut max_yy = 0;
    let mut max_zz = 0;
    for gg in &all_grid {
        let ([min_x, min_y, min_z], [max_x, max_y, max_z]) = extents(&gg);
        min_xx = min_xx.min(min_x);
        min_yy = min_yy.min(min_y);
        min_zz = min_zz.min(min_z);
        max_xx = max_xx.max(max_x);
        max_yy = max_yy.max(max_y);
        max_zz = max_zz.max(max_z);
    }
    for gg in &mut all_grid {
        // Insert the min/max corners in all grids
        gg.insert([min_xx, min_yy, min_zz], '.');
        gg.insert([max_xx, max_yy, max_zz], '.');
    }
    let window = aoc::initscr();
    aoc::nl();
    aoc::noecho();
    aoc::curs_set(0);
    window.keypad(true);
    window.scrollok(true);
    window.nodelay(true);
    for gg in &all_grid {
        window.clear();
        let mut g = HashMap::new();
        // Convert coords
        for (p, v) in gg {
            g.insert(cube_to_oddr(*p), *v);
        }
        let min_x = g.iter().map(|(p, _v)| p[0]).min().unwrap() as i32;
        let min_y = g.iter().map(|(p, _v)| p[1]).min().unwrap() as i32;
        let max_x = g.iter().map(|(p, _v)| p[0]).max().unwrap() as i32;
        let max_y = g.iter().map(|(p, _v)| p[1]).max().unwrap() as i32;
	// two rows per row in the output
        let w = 2 * (max_x - min_x);
        let h = 2 * (max_y - min_y);
        let ww = window.get_max_x();
        let wh = window.get_max_y();
        let offs_x = (ww - w) / 2;
        let offs_y = (wh - h) / 2;
        let mut yy = offs_y;
        let mut xx = offs_x;
        for y in min_y..=max_y {
            if y.rem_euclid(2) == 0 {
                for _ in min_x..=max_x {
                    if yy >= 0 && yy < wh {
                        if xx >= 0 && xx < ww {
                            window.mvaddch(yy, xx, '\\');
                        }
                        if xx + 1 >= 0 && xx + 1 < ww {
                            window.mvaddch(yy, xx + 1, '/');
                        }
                    }
                    xx += 2;
                }
                yy += 1;
                xx = offs_x;
            }
            if y.rem_euclid(2) == 0 {
                if xx >= 0 && xx < ww && yy >= 0 && yy < wh {
                    window.mvaddch(yy, xx, ' ');
                }
                xx += 1;
            }
            for x in min_x..=max_x {
                if yy >= 0 && y < wh {
                    let p = [x as i64, y as i64];
                    let c = g.get(&p).unwrap_or(&'.');
                    if xx >= 0 && x < ww {
                        window.mvaddch(yy, xx, '|');
                    }
                    if xx + 1 >= 0 && x + 1 < ww {
                        window.mvaddch(yy, xx + 1, *c);
                    }
                }
                xx += 2;
            }
            if y.rem_euclid(2) == 0 {
                yy += 1;
                xx = offs_x;
                for _ in min_x..=max_x {
                    if yy >= 0 && yy < wh {
                        if xx >= 0 && xx < ww {
                            window.mvaddch(yy, xx + 1, '/');
                        }
                        if xx + 1 >= 0 && xx + 1 < ww {
                            window.mvaddch(yy, xx, '\\');
                        }
                    }
                    xx += 2;
                }
            }
            yy += 1;
            xx = offs_x;
            if yy > window.get_max_y() {
                break;
            }
        }
        if let Some(aoc::Input::Character(c)) = window.getch() {
            if c == 'q' {
                aoc::endwin();
                std::process::exit(0);
            }
        }
        window.refresh();
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
