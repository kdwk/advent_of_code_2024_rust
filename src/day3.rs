use crate::prelude::*;

fn parse(file: &str) -> String {
    let mut result = String::new();
    with(
        &[Document::at_path(
            current_dir().unwrap().join(file).display(),
            "input",
            Create::No,
        )],
        |d| {
            result = d["input"].content()?;
            OK
        },
    );
    result
}

pub fn task1() -> impl Display {
    let input = parse("inputs/day3.txt");
    let mut int_pairs = vec![];
    || -> Result<(), Box<dyn Error>> {
        let mul_pattern = Regex::new(r"mul\([0123456789]+,[0123456789]+\)")?;
        let int_pattern = Regex::new(r"[0123456789]+")?;
        for match_result in mul_pattern.find_iter(&input) {
            let substr = match_result.as_str();
            let mut ints = int_pattern
                .find_iter(substr)
                .map(|int_str| int_str.as_str().parse::<i32>().unwrap());
            int_pairs.push((ints.next().unwrap(), ints.next().unwrap()));
        }
        OK
    }()
    .unwrap();
    int_pairs
        .into_iter()
        .map(|(int1, int2)| int1 * int2)
        .sum::<i32>()
}

pub fn task2() -> impl Display {
    let input = parse("inputs/day3.txt");
    let mut int_pairs = vec![];
    let mut valid = true;
    || -> Result<(), Box<dyn Error>> {
        let mul_or_command_pattern =
            Regex::new(r"(mul\([0123456789]+,[0123456789]+\)|(do(n\'t)?))")?;
        let int_pattern = Regex::new(r"[0123456789]+")?;
        for match_result in mul_or_command_pattern.find_iter(&input) {
            let substr = match_result.as_str();
            if substr == "do" {
                valid = true;
                continue;
            } else if substr == "don't" {
                valid = false;
                continue;
            }
            if valid {
                let mut ints = int_pattern
                    .find_iter(substr)
                    .map(|int_str| int_str.as_str().parse::<i32>());
                if let Some(Ok(int1)) = ints.next() {
                    if let Some(Ok(int2)) = ints.next() {
                        int_pairs.push((int1, int2));
                    }
                }
            }
        }
        OK
    }()
    .unwrap();
    int_pairs
        .into_iter()
        .map(|(int1, int2)| int1 * int2)
        .sum::<i32>()
}
