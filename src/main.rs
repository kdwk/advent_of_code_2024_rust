mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

mod prelude {
    pub use documents::prelude::*;
    pub use easy_ext::ext;
    pub use regex::Regex;
    pub use std::{collections::HashMap, env::current_dir, error::Error, fmt::Display};

    pub const OK: Result<(), Box<dyn Error>> = Ok(());
}

fn main() {
    println!("{}", day5::task2());
}
