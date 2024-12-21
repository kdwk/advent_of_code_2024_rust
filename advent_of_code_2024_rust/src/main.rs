mod day1;
mod day2;

mod prelude {
    pub use documents::prelude::*;
    pub use std::{env::current_dir, error::Error, fmt::Display};

    pub const OK: Result<(), Box<dyn Error>> = Ok(());
}

fn main() {
    println!("{}", day1::task2());
}
