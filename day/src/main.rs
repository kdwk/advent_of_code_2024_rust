use std::{error::Error, process::Command};

use documents::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let day: usize = std::env::args().collect::<Vec<String>>()[1].parse()?;
    with(
        &[
            Document::at_path(format!("src/day{day}.rs"), "day", Create::OnlyIfNotExists),
            Document::at_path(format!("src/main.rs"), "main", Create::No),
            Document::at_path(
                format!("inputs/day{day}.txt"),
                "input",
                Create::OnlyIfNotExists,
            ),
        ],
        |mut d| {
            d["day"].append(
                br#"use crate::prelude::*;

pub fn task1() -> impl Display {
    "Not implemented"
}

pub fn task2() -> impl Display {
    "Not implemented"
}
"#,
            )?;
            d["main"].append(format!("mod day{day};").as_bytes())?;
            Ok(())
        },
    );
    _ = Command::new("cargo").arg("fmt").arg("--all").status();
    Ok(())
}
