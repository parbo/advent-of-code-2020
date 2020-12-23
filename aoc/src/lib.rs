use image::{Rgb, RgbImage};
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
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[macro_use]
extern crate lazy_static;

extern crate vecmath;

pub use itertools::Itertools;
pub use mod_exp::mod_exp;
pub use num::integer::*;
pub use petgraph::algo;
pub use petgraph::graph::Graph;
pub use petgraph::graph::UnGraph;
pub use petgraph::graphmap::GraphMap;
pub use petgraph::graphmap::UnGraphMap;
pub use petgraph::visit;
pub use petgraph::*;
pub use regex::Regex;
pub use serde_scan::from_str;
pub use serde_scan::scan;

pub type Point = self::vecmath::Vector2<i64>;
pub type FPoint = self::vecmath::Vector2<f64>;
pub type Vec3 = self::vecmath::Vector3<i64>;
pub type FVec3 = self::vecmath::Vector3<f64>;
pub type Vec4 = self::vecmath::Vector4<i64>;
pub type FVec4 = self::vecmath::Vector4<f64>;
pub type Mat4 = self::vecmath::Matrix4<i64>;
pub type FMat4 = self::vecmath::Matrix4<f64>;
pub type Mat3 = self::vecmath::Matrix3<i64>;

pub use self::vecmath::mat3_id;
pub use self::vecmath::mat3_inv;
pub use self::vecmath::mat4_id as mat_identity;
pub use self::vecmath::mat4_transposed as mat_transpose;
pub use self::vecmath::row_mat3_mul;
pub use self::vecmath::row_mat3_transform_pos2;
pub use self::vecmath::row_mat3_transform_vec2;
pub use self::vecmath::row_mat4_mul as mat_mul;
pub use self::vecmath::row_mat4_transform as mat_transform;
pub use self::vecmath::vec2_add as point_add;
pub use self::vecmath::vec2_dot as point_dot;
pub use self::vecmath::vec2_neg as point_neg;
pub use self::vecmath::vec2_normalized as point_normalize;
pub use self::vecmath::vec2_scale as point_mul;
pub use self::vecmath::vec2_square_len as point_square_length;
pub use self::vecmath::vec2_sub as point_sub;
pub use self::vecmath::vec3_add as vec_add;
pub use self::vecmath::vec3_cross as vec_cross;
pub use self::vecmath::vec3_dot as vec_dot;
pub use self::vecmath::vec3_neg as vec_neg;
pub use self::vecmath::vec3_normalized as vec_normalize;
pub use self::vecmath::vec3_scale as vec_mul;
pub use self::vecmath::vec3_square_len as vec_square_length;
pub use self::vecmath::vec3_sub as vec_sub;
pub use self::vecmath::vec4_add;

pub fn length(v: FVec3) -> f64 {
    vec_square_length(v).sqrt()
}

pub fn cmul(v1: Vec3, v2: Vec3) -> Vec3 {
    let [x1, y1, z1] = v1;
    let [x2, y2, z2] = v2;
    [x1 * x2, y1 * y2, z1 * z2]
}

pub fn cmul2(v1: Point, v2: Point) -> Point {
    let [x1, y1] = v1;
    let [x2, y2] = v2;
    [x1 * x2, y1 * y2]
}

pub const NORTH: Point = [0, -1];
pub const UP: Point = NORTH;
pub const NORTH_EAST: Point = [1, -1];
pub const UP_RIGHT: Point = NORTH_EAST;
pub const EAST: Point = [1, 0];
pub const RIGHT: Point = EAST;
pub const SOUTH_EAST: Point = [1, 1];
pub const DOWN_RIGHT: Point = SOUTH_EAST;
pub const SOUTH: Point = [0, 1];
pub const DOWN: Point = SOUTH;
pub const SOUTH_WEST: Point = [-1, 1];
pub const DOWN_LEFT: Point = SOUTH_WEST;
pub const WEST: Point = [-1, 0];
pub const LEFT: Point = WEST;
pub const NORTH_WEST: Point = [-1, -1];
pub const UP_LEFT: Point = NORTH_WEST;

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
        .filter(|x| !x.is_empty())
        .collect()
}

pub fn split_ch(s: &str, c: char) -> Vec<&str> {
    s.split(|x| x == c)
        .map(|w| w.trim())
        .filter(|x| !x.is_empty())
        .collect()
}

pub fn split_str<'a>(s: &'a str, pat: &str) -> Vec<&'a str> {
    s.split(pat)
        .map(|w| w.trim())
        .filter(|x| !x.is_empty())
        .collect()
}

pub fn split_by_empty_line<'a, I, J>(lines: I) -> Vec<Vec<&'a str>>
where
    I: IntoIterator<Item = &'a J>,
    J: AsRef<str> + 'a,
{
    lines
        .into_iter()
        .group_by(|line| !AsRef::as_ref(line).is_empty())
        .into_iter()
        .map(|(_, group)| {
            group
                .map(|s| AsRef::as_ref(s))
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
        })
        .filter(|g| !g.is_empty())
        .collect()
}

pub fn parse_to_vec_of<'a, I, J, T, E>(items: I) -> Result<Vec<T>, ParseError>
where
    I: IntoIterator<Item = &'a J>,
    J: AsRef<str> + 'a,
    T: FromStr<Err = E>,
    E: Into<ParseError>,
{
    items
        .into_iter()
        .map(|x| T::from_str(AsRef::as_ref(x)).map_err(|x| -> ParseError { x.into() }))
        .collect()
}

pub fn parse_grid<'a, I, J>(lines: I) -> Vec<Vec<char>>
where
    I: IntoIterator<Item = &'a J>,
    J: AsRef<str> + 'a,
{
    lines
        .into_iter()
        .map(|x| AsRef::as_ref(x).chars().collect())
        .collect()
}

pub fn parse_grid_to<'a, I, J, T>(lines: I, f: fn(char) -> T) -> Vec<Vec<T>>
where
    I: IntoIterator<Item = &'a J>,
    J: AsRef<str> + 'a,
{
    lines
        .into_iter()
        .map(|x| AsRef::as_ref(x).chars().map(f).collect())
        .collect()
}

pub fn parse_grid_to_sparse<'a, I, J, T>(lines: I, f: fn(char) -> Option<T>) -> HashMap<Point, T>
where
    I: IntoIterator<Item = &'a J>,
    J: AsRef<str> + 'a,
{
    let mut grid = HashMap::new();
    for (y, line) in lines.into_iter().enumerate() {
        for (x, c) in AsRef::as_ref(line).chars().enumerate() {
            if let Some(t) = f(c) {
                grid.insert([x as i64, y as i64], t);
            }
        }
    }
    grid
}

pub fn grid_to_graph<T>(
    grid: &dyn Grid<T>,
    is_node: fn(&Point, &T) -> bool,
    get_edge: fn(&Point, &T, &Point, &T) -> Option<i64>,
    directions: usize,
) -> UnGraphMap<Point, i64>
where
    T: PartialEq + Copy,
{
    let directions: Vec<_> = match directions {
        4 => DIRECTIONS.clone(),
        8 => DIRECTIONS_INCL_DIAGONALS.clone(),
        _ => panic!(),
    };

    let mut graph = UnGraphMap::new();
    let (min, max) = grid.extents();

    for y in min[1]..=max[1] {
        for x in min[0]..=max[0] {
            let p: Point = [x as i64, y as i64];
            if let Some(c) = grid.get_value(p) {
                if is_node(&p, &c) {
                    let gp = graph.add_node(p);
                    for d in &directions {
                        let np = point_add(p, *d);
                        if np[0] >= min[0] && np[0] <= max[0] && np[1] >= min[1] && np[1] <= max[1]
                        {
                            if let Some(nc) = grid.get_value(np) {
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
    } else if a == 0 {
        cum_sum[b]
    } else {
        cum_sum[b] - cum_sum[a - 1]
    }
}

pub fn range_sum<T: num::Num + Copy>(cum_sum: &[T], a: usize, b: usize) -> T {
    if b > 0 {
        range_sum_inclusive(cum_sum, a, b - 1)
    } else {
        T::zero()
    }
}

pub fn egcd<T>(a: T, b: T) -> (T, T, T)
where
    T: std::cmp::PartialEq + num::Signed + Copy,
{
    if a == T::zero() {
        (b, T::zero(), T::one())
    } else {
        let (gcd, x, y) = egcd(b % a, a);
        (gcd, y - (b / a) * x, x)
    }
}

pub fn mod_inv<T>(x: T, n: T) -> Option<T>
where
    T: std::cmp::PartialEq + num::Signed + Copy,
{
    let (g, x, _) = egcd(x, n);
    if g == T::one() {
        Some((x % n + n) % n)
    } else {
        None
    }
}

pub fn chinese_remainder<'a, T>(residues: &[T], modulii: &'a [T]) -> Option<T>
where
    T: 'a
        + std::cmp::PartialEq
        + num::Signed
        + Copy
        + std::iter::Product<&'a T>
        + std::ops::AddAssign,
{
    let prod = modulii.iter().product::<T>();

    let mut sum = T::zero();

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

pub struct GridIteratorHelper {
    extents: (Point, Point),
    curr: Option<Point>,
}

impl Iterator for GridIteratorHelper {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some([x, y]) = self.curr {
            let c = if x < self.extents.1[0] {
                Some([x + 1, y])
            } else if y < self.extents.1[1] {
                Some([self.extents.0[0], y + 1])
            } else {
                None
            };
            let curr = self.curr;
            self.curr = c;
            curr
        } else {
            None
        }
    }
}

pub struct GridFlipIteratorHelper<G, T>
where
    G: Grid<T> + Clone,
    T: PartialEq + Copy,
{
    rot: usize,
    flip: bool,
    phantom: PhantomData<T>,
    grid: G,
}

impl<G, T> Iterator for GridFlipIteratorHelper<G, T>
where
    G: Grid<T> + Clone,
    T: PartialEq + Copy,
{
    type Item = G;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rot > 3 {
            return None;
        }
        let mut g = self.grid.clone();
        match self.rot {
            0 => {}
            1 => g.rotate_90_cw(),
            2 => g.rotate_180_cw(),
            3 => g.rotate_270_cw(),
            _ => panic!(),
        }
        if self.flip {
            g.flip_horizontal();
        }
        if !self.flip {
            self.flip = true;
        } else {
            self.flip = false;
            self.rot += 1;
        }
        Some(g)
    }
}

pub trait Grid<T>
where
    T: PartialEq + Copy,
{
    fn get_value(&self, pos: Point) -> Option<T>;
    fn set_value(&mut self, pos: Point, value: T);
    fn extents(&self) -> (Point, Point);
    fn points(&self) -> GridIteratorHelper {
        let extents = self.extents();
        GridIteratorHelper {
            extents,
            curr: Some(extents.0),
        }
    }
    fn flip_horizontal(&mut self);
    fn flip_vertical(&mut self);
    fn transpose(&mut self);
    fn rotate_90_cw(&mut self) {
        self.transpose();
        self.flip_horizontal();
    }
    fn rotate_180_cw(&mut self) {
        self.flip_vertical();
        self.flip_horizontal();
    }
    fn rotate_270_cw(&mut self) {
        self.transpose();
        self.flip_vertical();
    }
    fn fill(&mut self, pos: Point, value: T) {
        let ([min_x, min_y], [max_x, max_y]) = self.extents();
        if let Some(old) = self.get_value(pos) {
            if value != old {
                let mut todo = vec![];
                todo.push(pos);
                while let Some(p) = todo.pop() {
                    if let Some(curr) = self.get_value(p) {
                        if curr == old {
                            self.set_value(p, value);
                            if p[0] > min_x {
                                todo.push([p[0] - 1, p[1]]);
                            }
                            if p[0] < max_x {
                                todo.push([p[0] + 1, p[1]]);
                            }
                            if p[1] > min_y {
                                todo.push([p[0], p[1] - 1]);
                            }
                            if p[1] < max_y {
                                todo.push([p[0], p[1] + 1]);
                            }
                        }
                    }
                }
            }
        }
    }
    fn line(&mut self, a: Point, b: Point, value: T) {
        let line = plot_line(a, b);
        for p in line {
            self.set_value(p, value);
        }
    }
    fn blit(&mut self, pos: Point, g: &dyn Grid<T>) {
        let (start, end) = g.extents();
        self.blit_rect(pos, g, start, end);
    }
    // pos is position to blit to, start/end is the rect to copy from grid
    fn blit_rect(&mut self, pos: Point, g: &dyn Grid<T>, start: Point, end: Point) {
        let ([min_x, min_y], [max_x, max_y]) = g.extents();
        let min_xx = min_x.max(start[0]);
        let min_yy = min_y.max(start[1]);
        let max_xx = max_x.min(end[0]);
        let max_yy = max_y.min(end[1]);
        for (dy, yy) in (min_yy..=max_yy).enumerate() {
            for (dx, xx) in (min_xx..=max_xx).enumerate() {
                let [xxx, yyy] = point_add(pos, [dx as i64, dy as i64]);
                if let Some(v) = g.get_value([xx, yy]) {
                    self.set_value([xxx, yyy], v);
                }
            }
        }
    }
}

pub trait GridTranspose<G, T>
where
    Self: Grid<T> + Clone + Sized,
    T: PartialEq + Copy,
{
    // Note: consumes self
    fn into_transpositions(self) -> GridFlipIteratorHelper<Self, T>;
    fn transpositions(&self) -> GridFlipIteratorHelper<Self, T>;
}

impl<G, T> GridTranspose<G, T> for G
where
    G: Grid<T> + Clone + Sized,
    T: PartialEq + Copy,
{
    fn into_transpositions(self) -> GridFlipIteratorHelper<Self, T> {
        let grid = self;
        GridFlipIteratorHelper {
            rot: 0,
            flip: false,
            phantom: PhantomData,
            grid,
        }
    }
    fn transpositions(&self) -> GridFlipIteratorHelper<Self, T> {
        let grid = self.clone();
        GridFlipIteratorHelper {
            rot: 0,
            flip: false,
            phantom: PhantomData,
            grid,
        }
    }
}

impl<S: ::std::hash::BuildHasher, T> Grid<T> for HashMap<Point, T, S>
where
    T: Clone + Copy + Default + PartialEq,
{
    fn get_value(&self, pos: Point) -> Option<T> {
        if let Some(x) = self.get(&pos) {
            Some(*x)
        } else {
            None
        }
    }
    fn set_value(&mut self, pos: Point, value: T) {
        *self.entry(pos).or_insert(value) = value;
    }
    fn extents(&self) -> (Point, Point) {
        let min_x = self.iter().map(|(p, _v)| p[0]).min().unwrap();
        let min_y = self.iter().map(|(p, _v)| p[1]).min().unwrap();
        let max_x = self.iter().map(|(p, _v)| p[0]).max().unwrap();
        let max_y = self.iter().map(|(p, _v)| p[1]).max().unwrap();
        ([min_x, min_y], [max_x, max_y])
    }
    fn flip_horizontal(&mut self) {
        let ([min_x, _min_y], [max_x, _max_y]) = self.extents();
        let mut new_grid = HashMap::new();
        for ([x, y], v) in self.iter() {
            let new_x = max_x - (x - min_x);
            new_grid.insert([new_x, *y], *v);
        }
        self.clear();
        for (k, v) in new_grid {
            self.insert(k, v);
        }
    }
    fn flip_vertical(&mut self) {
        let ([_min_x, min_y], [_max_x, max_y]) = self.extents();
        let mut new_grid = HashMap::new();
        for ([x, y], v) in self.iter() {
            let new_y = max_y - (y - min_y);
            new_grid.insert([*x, new_y], *v);
        }
        self.clear();
        for (k, v) in new_grid {
            self.insert(k, v);
        }
    }
    fn transpose(&mut self) {
        let mut new_grid = HashMap::new();
        for ([x, y], v) in self.iter() {
            new_grid.insert([*y, *x], *v);
        }
        self.clear();
        for (k, v) in new_grid {
            self.insert(k, v);
        }
    }
}

impl<T> Grid<T> for Vec<Vec<T>>
where
    T: Clone + Copy + Default + PartialEq,
{
    fn get_value(&self, pos: Point) -> Option<T> {
        let [x, y] = pos;
        if let Some(line) = self.get(y as usize) {
            if let Some(c) = line.get(x as usize) {
                return Some(*c);
            }
        }
        None
    }
    fn set_value(&mut self, pos: Point, value: T) {
        let [x, y] = pos;
        if let Some(line) = self.get_mut(y as usize) {
            if let Some(c) = line.get_mut(x as usize) {
                *c = value;
            }
        }
    }
    fn extents(&self) -> (Point, Point) {
        if !self.is_empty() && !self[0].is_empty() {
            return (
                [0, 0],
                [(self[0].len() - 1) as i64, (self.len() - 1) as i64],
            );
        }
        ([0, 0], [0, 0])
    }
    fn flip_horizontal(&mut self) {
        let ([min_x, min_y], [max_x, max_y]) = self.extents();
        let mut new_vec = self.clone();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let v = self[y as usize][x as usize];
                let new_x = max_x - (x - min_x);
                new_vec[y as usize][new_x as usize] = v;
            }
        }
        *self = new_vec;
    }
    fn flip_vertical(&mut self) {
        let ([min_x, min_y], [max_x, max_y]) = self.extents();
        let mut new_vec = self.clone();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let v = self[y as usize][x as usize];
                let new_y = max_y - (y - min_y);
                new_vec[new_y as usize][x as usize] = v;
            }
        }
        *self = new_vec;
    }
    fn transpose(&mut self) {
        let ([min_x, min_y], [max_x, max_y]) = self.extents();
        let width = (max_x - min_x + 1) as usize;
        let height = (max_y - min_y + 1) as usize;
        // Make a vec with the transposed dimensions
        let mut new_vec = Vec::with_capacity(width);
        for _ in min_x..=max_x {
            let mut row = Vec::with_capacity(height);
            row.resize_with(height, Default::default);
            new_vec.push(row);
        }
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let v = self[y as usize][x as usize];
                new_vec[x as usize][y as usize] = v;
            }
        }
        *self = new_vec;
    }
}

pub trait GridDrawer<G, T>
where
    G: Grid<T>,
    T: PartialEq + Copy,
{
    fn draw(&mut self, area: &G);
}

pub struct NopGridDrawer {}

impl<G, T> GridDrawer<G, T> for NopGridDrawer
where
    G: Grid<T>,
    T: PartialEq + Copy,
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
    T: PartialEq + Copy,
{
    fn draw(&mut self, area: &G) {
        let ([min_x, min_y], [max_x, max_y]) = area.extents();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let ch = if let Some(x) = area.get_value([x, y]) {
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
    T: PartialEq + Copy,
{
    fn draw(&mut self, area: &G) {
        self.window.clear();
        let ([min_x, _], [min_y, _]) = area.extents();
        for p in area.points() {
            let ch = if let Some(x) = area.get_value(p) {
                self.to_char(x)
            } else {
                ' '
            };
            self.window
                .mvaddch((p[1] - min_y) as i32, (p[0] - min_x) as i32, ch);
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

pub struct BitmapGridDrawer<F, G, T>
where
    F: Fn(T) -> Vec<(u8, u8, u8)>,
    G: Grid<T>,
    T: PartialEq + Copy,
{
    sprite_dimension: (i64, i64),
    to_sprite: F,
    basename: String,
    frame: usize,
    rect: Option<(Point, Point)>,
    image: Option<RgbImage>,
    phantom: PhantomData<T>,
    phantom_g: PhantomData<G>,
}

// These can be converted to movies with:
// ffmpeg -framerate 25 -i "basename_%06d.png" output
// You can change the start number with the -start_number input option.
impl<F, G, T> BitmapGridDrawer<F, G, T>
where
    F: Fn(T) -> Vec<(u8, u8, u8)>,
    G: Grid<T>,
    T: PartialEq + Copy,
{
    pub fn new(
        sprite_dimension: (i64, i64),
        to_sprite: F,
        basename: &str,
    ) -> BitmapGridDrawer<F, G, T> {
        // TODO: error handling
        let path = Path::new(basename);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("could not create folder");
        }
        BitmapGridDrawer {
            sprite_dimension,
            to_sprite,
            frame: 1,
            basename: basename.into(),
            rect: None,
            image: None,
            phantom: PhantomData,
            phantom_g: PhantomData,
        }
    }

    pub fn set_rect(&mut self, r: (Point, Point)) {
        self.rect = Some(r);
    }

    pub fn save_image(&self) {
        let path = Path::new(&self.basename);
        let filename = if let Some(parent) = path.parent() {
            parent.join(&format!(
                "{}_{:06}.png",
                path.file_name().unwrap().to_str().unwrap(),
                self.frame
            ))
        } else {
            PathBuf::from(&format!("{}_{}.png", self.basename, self.frame))
        };
        if let Some(image) = &self.image {
            image.save(filename).unwrap();
        }
    }

    pub fn draw_grid(&mut self, area: &G) {
        self.frame += 1;
        let ([mut min_x, mut min_y], [mut max_x, mut max_y]) = area.extents();
        // "clip" to rect
        if let Some(([cmin_x, cmin_y], [cmax_x, cmax_y])) = self.rect {
            min_x = cmin_x;
            min_y = cmin_y;
            max_x = cmax_x;
            max_y = cmax_y;
        }
        let width = max_x - min_x + 1;
        let height = max_y - min_y + 1;
        let pixelw = width * self.sprite_dimension.0;
        let pixelh = height * self.sprite_dimension.1;
        // Default bg is white
        let buffer = vec![255; (3 * pixelw * pixelh) as usize];
        let mut image = RgbImage::from_raw(pixelw as u32, pixelh as u32, buffer).unwrap();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if let Some(value) = area.get_value([x, y]) {
                    let sprite = self.to_sprite(value);
                    let mut yy = (y - min_y) * self.sprite_dimension.1;
                    let mut xx = (x - min_x) * self.sprite_dimension.0;
                    let xxx = xx;
                    for col in &sprite {
                        let (r, g, b) = *col;
                        let rgb = Rgb([r, g, b]);
                        image.put_pixel(xx as u32, yy as u32, rgb);
                        xx += 1;
                        if xx - xxx >= self.sprite_dimension.0 {
                            xx = x * self.sprite_dimension.0;
                            yy += 1
                        }
                    }
                }
            }
        }
        self.image = Some(image);
    }

    pub fn put_pixel(&mut self, p: Point, rgb: (u8, u8, u8)) {
        if let Some(ref mut image) = self.image {
            let x = p[0] as u32;
            let y = p[1] as u32;
            if x < image.width() && y < image.height() {
                image.put_pixel(x, y, Rgb([rgb.0, rgb.1, rgb.2]));
            }
        }
    }

    fn to_sprite(&self, value: T) -> Vec<(u8, u8, u8)> {
        (self.to_sprite)(value)
    }
}

impl<F, G, T> GridDrawer<G, T> for BitmapGridDrawer<F, G, T>
where
    F: Fn(T) -> Vec<(u8, u8, u8)>,
    G: Grid<T>,
    T: PartialEq + Copy,
{
    fn draw(&mut self, area: &G) {
        self.draw_grid(area);
        self.save_image();
    }
}

// Bresenham
// https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
pub fn plot_line(a: Point, b: Point) -> Vec<Point> {
    let [mut x0, mut y0] = a;
    let [x1, y1] = b;
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy; /* error value e_xy */
    let mut out = vec![];
    loop {
        out.push([x0, y0]);
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        /* e_xy+e_x > 0 */
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        /* e_xy+e_y < 0 */
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
    out
}

pub fn read_lines_from(filename: &str) -> Vec<String> {
    let input = File::open(Path::new(filename)).unwrap();
    let buffered = BufReader::new(input);
    buffered
        .lines()
        .filter_map(Result::ok)
        .map(|x| x.trim_end_matches('\n').to_string())
        .collect()
}

pub fn read_lines() -> (i32, Vec<String>) {
    let args: Vec<String> = env::args().collect();
    let part = args[1].parse::<i32>().unwrap();
    let filename = &args[2];

    (part, read_lines_from(filename))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_by_empty_line() {
        let result =
            split_by_empty_line(&["apa", "giraff", "", "elefant", "", "lejon", "tiger", ""]);
        let expected = vec![
            vec!["apa", "giraff"],
            vec!["elefant"],
            vec!["lejon", "tiger"],
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_chinese_remainder() {
        let modulii = [3, 5, 7];
        let residues = [2, 3, 2];
        assert_eq!(chinese_remainder(&residues, &modulii), Some(23));
    }

    #[test]
    fn test_flip() {
        // Vecs
        let orig_g: Vec<Vec<char>> = vec!["####".chars().collect(), "#   ".chars().collect()];
        let mut g = orig_g.clone();
        let expected: Vec<Vec<char>> = vec!["#   ".chars().collect(), "####".chars().collect()];
        g.flip_vertical();
        assert_eq!(g, expected);
        let mut g = orig_g.clone();
        let expected: Vec<Vec<char>> = vec!["####".chars().collect(), "   #".chars().collect()];
        g.flip_horizontal();
        assert_eq!(g, expected);
        // Hashmaps
        let orig_g: HashMap<Point, char> = vec![
            ([-1, 0], '#'),
            ([0, 0], '#'),
            ([1, 0], '#'),
            ([2, 0], '#'),
            ([-1, 1], '#'),
        ]
        .into_iter()
        .collect();
        let mut g = orig_g.clone();
        let expected: HashMap<Point, char> = vec![
            ([-1, 1], '#'),
            ([0, 1], '#'),
            ([1, 1], '#'),
            ([2, 1], '#'),
            ([-1, 0], '#'),
        ]
        .into_iter()
        .collect();
        g.flip_vertical();
        assert_eq!(g, expected);
        let mut g = orig_g.clone();
        let expected: HashMap<Point, char> = vec![
            ([-1, 0], '#'),
            ([0, 0], '#'),
            ([1, 0], '#'),
            ([2, 0], '#'),
            ([2, 1], '#'),
        ]
        .into_iter()
        .collect();
        g.flip_horizontal();
        assert_eq!(g, expected);
    }
}
