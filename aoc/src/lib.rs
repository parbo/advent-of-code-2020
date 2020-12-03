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

pub use mod_exp::mod_exp;
pub use modinverse::modinverse;
pub use num::integer::*;
pub use serde_scan::from_str;
pub use serde_scan::scan;

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
