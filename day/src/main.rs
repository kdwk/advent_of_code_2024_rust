use std::{error::Error, path::PathBuf, process::Command};

use documents::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let day: usize = std::env::args().collect::<Vec<String>>()[1].parse()?;
    with(
        &[
            Document::at_path(
                PathBuf::from(format!("src/day{day}.rs")).display(),
                "day",
                Create::OnlyIfNotExists,
            ),
            Document::at_path(
                PathBuf::from(format!("src/main.rs")).display(),
                "main",
                Create::No,
            ),
        ],
        |mut d| {
            d["day"].append(
                br#"use crate::prelude::*;

fn task1() -> impl Display {
    "Not implemented"
}

fn task2() -> impl Display {
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
