use crate::prelude::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Stone {
    value: i64,
}

impl Stone {
    fn new(value: i64) -> Self {
        Self { value }
    }
    fn split(&self) -> Option<(Stone, Stone)> {
        let value_string = self.value.to_string();
        if value_string.chars().count() % 2 != 0 {
            None?;
        }
        let value_split = value_string.split_at(value_string.len() / 2);
        Some((
            Stone::new(value_split.0.parse().unwrap()),
            Stone::new(value_split.1.parse().unwrap()),
        ))
    }
}

#[cache]
fn score(stone: Stone, depth: u32, max_depth: u32) -> i64 {
    if depth == max_depth {
        1
    } else if stone.value == 0 {
        score(Stone::new(1), depth + 1, max_depth)
    } else if let Some((stone1, stone2)) = stone.split() {
        score(stone1, depth + 1, max_depth)
            + score(stone2, depth + 1, max_depth)
    } else {
        score(Stone::new(stone.value * 2024), depth + 1, max_depth)
    }
}

/// Rock value map to number of rocks
fn parse<'stone>(file: &str) -> Vec<Stone> {
    let mut result = vec![];
    with(
        &[Document::at_path(
            current_dir().unwrap().join(file).display(),
            "input",
            Create::No,
        )],
        |d| {
            result = d["input"]
                .content()?
                .split_whitespace()
                .map(|str| Stone::new(str.parse().unwrap()))
                .collect();
            OK
        },
    );
    result
}

pub fn task1() -> impl Display {
    let stones = parse("inputs/day11.txt");
    stones
        .into_iter()
        .map(|stone| score(stone, 0, 25))
        .sum::<i64>()
}

pub fn task2() -> impl Display {
    let stones = parse("inputs/day11.txt");
    stones
        .into_iter()
        .map(|stone| score(stone, 0, 75))
        .sum::<i64>()
}
