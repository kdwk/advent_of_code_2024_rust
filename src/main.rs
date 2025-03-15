use documents::prelude::*;
use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::{BufReader, Lines},
    path::PathBuf,
};

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

mod prelude {
    pub use super::{IntoOpt, IntoWhoops, NoneError, Whoops, OK};
    pub use documents::prelude::*;
    pub use easy_ext::ext;
    pub use itertools::{repeat_n, Itertools};
    pub use memoize::memoize as cache;
    pub use rayon::prelude::*;
    pub use regex::Regex;
    pub use std::{
        collections::{HashMap, HashSet},
        convert::identity,
        env::current_dir,
        error::Error,
        fmt::Display,
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

fn main() {
    println!("{}", day13::task2());
}
