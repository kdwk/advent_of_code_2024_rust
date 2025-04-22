use std::{
    error::Error,
    fmt::{Debug, Display},
    io::{self, Read, Write},
    ops::Range,
};

mod prelude {
    pub use super::{Apply, Dbg, Disp, IntoOpt, IntoWhoops, NoneError, OK, Whoops, input, wrap};
    pub use documents::prelude::*;
    pub use easy_ext::ext;
    pub use itertools::{Itertools, repeat_n};
    pub use memoize::memoize as cache;
    pub use rayon::prelude::*;
    pub use regex::Regex;
    pub use scanf::sscanf;
    pub use std::{
        collections::{HashMap, HashSet},
        convert::identity,
        env::current_dir,
        error::Error,
        fmt::{Debug, Display},
        ops::Index,
        path::PathBuf,
    };
}

pub const OK: Whoops = Ok(());

/// NoneError: Expected Some(...), got None.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct NoneError;

impl Error for NoneError {
    /// "NoneError: Expected Some(...), got None."
    fn description(&self) -> &str {
        "NoneError: Expected Some(...), got None."
    }
}

impl Display for NoneError {
    /// "NoneError: expected Some(...), got None."
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad("NoneError: expected Some(...), got None.")
    }
}

pub type Whoops = Result<(), Box<dyn Error>>;

pub trait IntoWhoops {
    fn into_whoops(self) -> Whoops;
}

impl IntoWhoops for () {
    fn into_whoops(self) -> Whoops {
        Ok(())
    }
}

impl<T> IntoWhoops for Result<T, Box<dyn Error>> {
    fn into_whoops(self) -> Whoops {
        match self {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }
}

impl<T> IntoWhoops for Option<T> {
    fn into_whoops(self) -> Whoops {
        match self {
            Some(_) => Ok(()),
            None => Err(NoneError)?,
        }
    }
}

pub trait IntoOpt<T> {
    fn into_opt(self) -> Option<T>;
}

impl<T> IntoOpt<T> for Option<T> {
    fn into_opt(self) -> Option<T> {
        self
    }
}

impl<T, E> IntoOpt<T> for Result<T, E> {
    fn into_opt(self) -> Option<T> {
        match self {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    }
}

pub trait Dbg {
    fn debug(self) -> Self;
}

impl<T: Debug> Dbg for T {
    fn debug(self) -> Self {
        println!("{self:?}");
        self
    }
}

pub trait Disp {
    fn display(self) -> Self;
}

impl<T: Display> Disp for T {
    fn display(self) -> Self {
        println!("{self}");
        self
    }
}

pub trait Apply<Closure>
where
    Self: Sized,
    Closure: FnOnce(&mut Self),
{
    fn apply(self, closure: Closure) -> Self;
}

impl<T, Closure> Apply<Closure> for T
where
    Self: Sized,
    Closure: FnOnce(&mut Self),
{
    fn apply(mut self, closure: Closure) -> Self {
        closure(&mut self);
        self
    }
}

/// Wrap an i32 in a range such that it wraps around to the other end of range as it leaves the range
///
/// range: inclusive..exclusive. End must be greater than (>) start.
///
/// Examples:
/// ```
/// assert_eq!(wrap(2, -7..-5), -6);
/// assert_eq!(wrap(5, -2..2), 1);
/// assert_eq!(wrap(-10, -7..-5), -6);
/// assert_eq!(wrap(21, 10..20), 11);
/// assert_eq!(wrap(-1, 10..20), 19);
/// ```
pub const fn wrap(i: i32, range: Range<i32>) -> i32 {
    let diff = range.end - range.start;
    assert!(diff > 0);
    if i >= range.end {
        let offset = (i - range.end) % diff;
        range.start + offset
    } else if i < range.start {
        let offset = (range.start - i) % diff;
        if offset == 0 {
            range.start
        } else {
            range.end - offset
        }
    } else {
        i
    }
}

pub fn input(prompt: impl Display) -> String {
    print!("{prompt}");
    _ = io::stdout().flush();
    let mut input = String::new();
    _ = io::stdin().read_line(&mut input);
    input
}

fn main() {
    println!("{}", day15::task1());
}

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
