use num;
use pancurses;
use std::collections::HashMap;
use std::env;
use std::error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::*;
use std::marker::PhantomData;
use std::num::ParseIntError;
use std::path::Path;

#[macro_use]
extern crate lazy_static;

extern crate vecmath;

pub use mod_exp::mod_exp;
pub use modinverse::modinverse;
pub use num::integer::*;
pub use petgraph::graphmap::UnGraphMap;
pub use serde_scan::from_str;
pub use serde_scan::scan;
pub use itertools::Itertools;

pub type Point = self::vecmath::Vector2<i64>;
pub type FPoint = self::vecmath::Vector2<f64>;
pub type Vec3 = self::vecmath::Vector3<i64>;
pub type FVec3 = self::vecmath::Vector3<f64>;
pub type Vec4 = self::vecmath::Vector4<i64>;
pub type FVec4 = self::vecmath::Vector4<f64>;
pub type Mat4 = self::vecmath::Matrix4<i64>;
pub type FMat4 = self::vecmath::Matrix4<f64>;

pub use self::vecmath::mat4_id as mat_identity;
pub use self::vecmath::mat4_transposed as mat_transpose;
pub use self::vecmath::row_mat4_mul as mat_mul;
pub use self::vecmath::row_mat4_transform as mat_transform;
pub use self::vecmath::vec2_add as point_add;
pub use self::vecmath::vec2_dot as point_dot;
pub use self::vecmath::vec2_neg as point_neg;
pub use self::vecmath::vec2_normalized as point_normalize;
pub use self::vecmath::vec2_square_len as point_square_length;
pub use self::vecmath::vec2_sub as point_sub;
pub use self::vecmath::vec3_add as vec_add;
pub use self::vecmath::vec3_cross as vec_cross;
pub use self::vecmath::vec3_dot as vec_dot;
pub use self::vecmath::vec3_neg as vec_neg;
pub use self::vecmath::vec3_normalized as vec_normalize;
pub use self::vecmath::vec3_scale as vec_mul;
pub use self::vecmath::vec3_scale as point_mul;
pub use self::vecmath::vec3_square_len as vec_square_length;
pub use self::vecmath::vec3_sub as vec_sub;

pub fn length(v: FVec3) -> f64 {
    vec_square_length(v).sqrt()
}

pub fn cmul(v1: Vec3, v2: Vec3) -> Vec3 {
    let [x1, y1, z1] = v1;
    let [x2, y2, z2] = v2;
    [x1 * x2, y1 * y2, z1 * z2]
}

pub static NORTH: Point = [0, -1];
pub static UP: Point = NORTH;
pub static NORTH_EAST: Point = [1, -1];
pub static UP_RIGHT: Point = NORTH_EAST;
pub static EAST: Point = [1, 0];
pub static RIGHT: Point = EAST;
pub static SOUTH_EAST: Point = [1, 1];
pub static DOWN_RIGHT: Point = SOUTH_EAST;
pub static SOUTH: Point = [0, 1];
pub static DOWN: Point = SOUTH;
pub static SOUTH_WEST: Point = [-1, 1];
pub static DOWN_LEFT: Point = SOUTH_WEST;
pub static WEST: Point = [-1, 0];
pub static LEFT: Point = WEST;
pub static NORTH_WEST: Point = [-1, -1];
pub static UP_LEFT: Point = NORTH_WEST;

lazy_static! {
    pub static ref DIRECTIONS: Vec<Point> = vec![NORTH, EAST, SOUTH, WEST];
    pub static ref DIRECTIONS_INCL_DIAGONALS: Vec<Point> = {
        vec![
            NORTH, NORTH_EAST, EAST, SOUTH_EAST, SOUTH, SOUTH_WEST, WEST, NORTH_WEST,
        ]
    };
    pub static ref DIRECTION_MAP: HashMap<&'static str, Point> = {
        let mut map = HashMap::new();
        map.insert("U", NORTH);
        map.insert("D", SOUTH);
        map.insert("R", EAST);
        map.insert("L", WEST);
        map.insert("N", NORTH);
        map.insert("S", SOUTH);
        map.insert("E", EAST);
        map.insert("W", WEST);
        map.insert("NW", NORTH_WEST);
        map.insert("SW", SOUTH_WEST);
        map.insert("NE", NORTH_WEST);
        map.insert("SE", SOUTH_EAST);
        map
    };
}

#[derive(Debug)]
pub enum ParseError {
    Generic,
    Parse(ParseIntError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::Generic => write!(f, "some error"),
            ParseError::Parse(..) => write!(f, "the provided string could not be parsed as int"),
        }
    }
}

impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            ParseError::Parse(ref e) => Some(e),
            ParseError::Generic => None,
        }
    }
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> ParseError {
        ParseError::Parse(err)
    }
}

pub fn split(s: &str, pred: fn(char) -> bool) -> Vec<&str> {
    s.split(pred)
        .map(|w| w.trim())
        .filter(|x| x.len() > 0)
        .collect()
}

pub fn parse_grid(lines: &[String]) -> Vec<Vec<char>> {
    lines.iter().map(|x| x.chars().collect()).collect()
}

pub fn parse_grid_to<T>(lines: &[String], f: fn(char) -> T) -> Vec<Vec<T>> {
    lines.iter().map(|x| x.chars().map(f).collect()).collect()
}

pub fn parse_str_grid(lines: &[&str]) -> Vec<Vec<char>> {
    lines.iter().map(|x| x.chars().collect()).collect()
}

pub fn parse_str_grid_to<T>(lines: &[&str], f: fn(char) -> T) -> Vec<Vec<T>> {
    lines.iter().map(|x| x.chars().map(f).collect()).collect()
}

pub fn grid_to_graph<T>(
    grid: &Vec<Vec<T>>,
    is_node: fn(&Point, &T) -> bool,
    get_edge: fn(&Point, &T, &Point, &T) -> Option<i64>,
    directions: usize,
) -> UnGraphMap<Point, i64> {
    let directions: Vec<_> = match directions {
        4 => DIRECTIONS.clone(),
        8 => DIRECTIONS_INCL_DIAGONALS.clone(),
        _ => panic!(),
    };

    let mut graph = UnGraphMap::new();
    let ysize = grid.len();
    let xsize = grid[0].len();

    for y in 0..ysize {
        for x in 0..xsize {
            let p: Point = [x as i64, y as i64];
            let c = &grid[y][x];
            if is_node(&p, &c) {
                let gp = graph.add_node(p);
                for d in &directions {
                    let np = point_add(p, *d);
                    if np[0] >= 0 && np[0] < xsize as i64 && np[1] >= 0 && np[1] < ysize as i64 {
                        let nc = &grid[np[1] as usize][np[0] as usize];
                        if is_node(&np, &nc) {
                            if let Some(e) = get_edge(&p, &c, &np, &nc) {
                                let gnp = graph.add_node(np);
                                graph.add_edge(gp, gnp, e);
                            }
                        }
                    }
                }
            }
        }
    }

    graph
}

pub fn astar(
    graph: &UnGraphMap<Point, i64>,
    start: Point,
    goal: Point,
) -> Option<(i64, Vec<Point>)> {
    petgraph::algo::astar(
        &graph,
        start,
        |finish| finish == goal,                             // is finish
        |(_n1, _n2, e)| *e,                                  // true cost
        |n| (goal[0] - n[0]).abs() + (goal[1] - n[1]).abs(), // estimated cost: manhattan distance}
    )
}

pub fn get_char(s: &str, ix: usize) -> Option<char> {
    s.chars().nth(ix)
}

pub fn parse_char(s: &str, ix: usize) -> Result<char, ParseError> {
    get_char(s, ix).ok_or(ParseError::Generic)
}

pub fn cum_sum<T: num::Num + Copy>(a: &[T]) -> Vec<T> {
    a.iter()
        .scan(T::zero(), |state, x| {
            *state = *state + *x;
            Some(*state)
        })
        .collect()
}

pub fn range_sum_inclusive<T: num::Num + Copy>(cum_sum: &[T], a: usize, b: usize) -> T {
    if b < a {
        T::zero()
    } else {
        if a == 0 {
            cum_sum[b]
        } else {
            cum_sum[b] - cum_sum[a - 1]
        }
    }
}

pub fn range_sum<T: num::Num + Copy>(cum_sum: &[T], a: usize, b: usize) -> T {
    if b > 0 {
        range_sum_inclusive(cum_sum, a, b - 1)
    } else {
        T::zero()
    }
}

pub trait Grid<T> {
    fn get_value(&self, pos: (i64, i64)) -> Option<T>;
    fn extents(&self) -> ((i64, i64), (i64, i64));
}

impl Grid<i64> for HashMap<(i64, i64), i64> {
    fn get_value(&self, pos: (i64, i64)) -> Option<i64> {
        if let Some(x) = self.get(&pos) {
            Some(*x)
        } else {
            None
        }
    }
    fn extents(&self) -> ((i64, i64), (i64, i64)) {
        let min_x = self.iter().map(|p| (p.0).0).min().unwrap();
        let min_y = self.iter().map(|p| (p.0).1).min().unwrap();
        let max_x = self.iter().map(|p| (p.0).0).max().unwrap();
        let max_y = self.iter().map(|p| (p.0).1).max().unwrap();
        ((min_x, max_x), (min_y, max_y))
    }
}

impl Grid<char> for HashMap<(i64, i64), char> {
    fn get_value(&self, pos: (i64, i64)) -> Option<char> {
        if let Some(x) = self.get(&pos) {
            Some(*x)
        } else {
            None
        }
    }
    fn extents(&self) -> ((i64, i64), (i64, i64)) {
        let min_x = self.iter().map(|p| (p.0).0).min().unwrap();
        let min_y = self.iter().map(|p| (p.0).1).min().unwrap();
        let max_x = self.iter().map(|p| (p.0).0).max().unwrap();
        let max_y = self.iter().map(|p| (p.0).1).max().unwrap();
        ((min_x, max_x), (min_y, max_y))
    }
}

impl Grid<i64> for Vec<Vec<i64>> {
    fn get_value(&self, pos: (i64, i64)) -> Option<i64> {
        let (x, y) = pos;
        if let Some(line) = self.get(y as usize) {
            if let Some(c) = line.get(x as usize) {
                return Some(*c);
            }
        }
        None
    }
    fn extents(&self) -> ((i64, i64), (i64, i64)) {
        if self.len() > 0 {
            if self[0].len() > 0 {
                return (
                    (0, (self[0].len() - 1) as i64),
                    (0, (self.len() - 1) as i64),
                );
            }
        }
        ((0, 0), (0, 0))
    }
}

impl Grid<char> for Vec<Vec<char>> {
    fn get_value(&self, pos: (i64, i64)) -> Option<char> {
        let (x, y) = pos;
        if let Some(line) = self.get(y as usize) {
            if let Some(c) = line.get(x as usize) {
                return Some(*c);
            }
        }
        None
    }
    fn extents(&self) -> ((i64, i64), (i64, i64)) {
        if self.len() > 0 {
            if self[0].len() > 0 {
                return (
                    (0, (self[0].len() - 1) as i64),
                    (0, (self.len() - 1) as i64),
                );
            }
        }
        ((0, 0), (0, 0))
    }
}

pub trait GridDrawer<G, T>
where
    G: Grid<T>,
{
    fn draw(&mut self, area: &G);
}

pub struct NopGridDrawer {}

impl<G, T> GridDrawer<G, T> for NopGridDrawer
where
    G: Grid<T>,
{
    fn draw(&mut self, _: &G) {}
}

pub struct PrintGridDrawer<F, T>
where
    F: Fn(T) -> char,
{
    to_ch: F,
    phantom: PhantomData<T>,
}

impl<F, T> PrintGridDrawer<F, T>
where
    F: Fn(T) -> char,
{
    pub fn new(to_ch: F) -> PrintGridDrawer<F, T> {
        PrintGridDrawer {
            to_ch,
            phantom: PhantomData,
        }
    }

    fn to_char(&self, col: T) -> char {
        (self.to_ch)(col)
    }
}

impl<F, G, T> GridDrawer<G, T> for PrintGridDrawer<F, T>
where
    F: Fn(T) -> char,
    G: Grid<T>,
{
    fn draw(&mut self, area: &G) {
        let ((min_x, max_x), (min_y, max_y)) = area.extents();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let ch = if let Some(x) = area.get_value((x, y)) {
                    self.to_char(x)
                } else {
                    ' '
                };
                print!("{}", ch);
            }
            println!();
        }
    }
}

pub struct CursesGridDrawer<F, T>
where
    F: Fn(T) -> char,
{
    window: pancurses::Window,
    to_ch: F,
    phantom: PhantomData<T>,
}

impl<F, T> CursesGridDrawer<F, T>
where
    F: Fn(T) -> char,
{
    pub fn new(to_ch: F) -> CursesGridDrawer<F, T> {
        let window = pancurses::initscr();
        pancurses::nl();
        pancurses::noecho();
        pancurses::curs_set(0);
        window.keypad(true);
        window.scrollok(true);
        window.nodelay(true);
        CursesGridDrawer {
            window,
            to_ch,
            phantom: PhantomData,
        }
    }

    fn to_char(&self, col: T) -> char {
        (self.to_ch)(col)
    }
}

impl<F, T> Drop for CursesGridDrawer<F, T>
where
    F: Fn(T) -> char,
{
    fn drop(&mut self) {
        pancurses::endwin();
    }
}

impl<F, G, T> GridDrawer<G, T> for CursesGridDrawer<F, T>
where
    F: Fn(T) -> char,
    G: Grid<T>,
{
    fn draw(&mut self, area: &G) {
        self.window.clear();
        let ((min_x, max_x), (min_y, max_y)) = area.extents();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let ch = if let Some(x) = area.get_value((x, y)) {
                    self.to_char(x)
                } else {
                    ' '
                };
                self.window
                    .mvaddch((y - min_y) as i32, (x - min_x) as i32, ch);
            }
        }
        if let Some(pancurses::Input::Character(c)) = self.window.getch() {
            if c == 'q' {
                pancurses::endwin();
                std::process::exit(0);
            }
        }
        self.window.refresh();
    }
}

pub fn read_lines() -> (i32, Vec<String>) {
    let args: Vec<String> = env::args().collect();
    let part = args[1].parse::<i32>().unwrap();
    let filename = &args[2];

    let input = File::open(Path::new(filename)).unwrap();
    let buffered = BufReader::new(input);
    (
        part,
        buffered
            .lines()
            .filter_map(Result::ok)
            .map(|x| x.trim_end_matches('\n').to_string())
            .collect(),
    )
}
