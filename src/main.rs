mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;

mod prelude {
    pub use documents::prelude::*;
    pub use easy_ext::ext;
    pub use regex::Regex;
    pub use std::{collections::{HashMap, HashSet}, env::current_dir, error::Error, fmt::Display};
    pub use itertools::{repeat_n, Itertools};
    pub use rayon::prelude::*;
    pub use std::ops::Index;

    pub const OK: Result<(), Box<dyn Error>> = Ok(());
}

fn main() {
    println!("{}", day10::task2());
}
