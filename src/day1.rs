use crate::prelude::*;

fn parse(file: &str) -> (Vec<u32>, Vec<u32>) {
    let (mut first_list, mut second_list) = (vec![], vec![]);
    with(
        &[Document::at_path(
            std::env::current_dir().unwrap().join(file).display(),
            "input",
            Create::No,
        )],
        |d| {
            for line in d["input"].lines()? {
                let parsed = line?
                    .split_whitespace()
                    .map(|str| str.parse().unwrap())
                    .collect::<Vec<u32>>();
                let first = parsed[0];
                let second = parsed[1];
                first_list.push(first);
                second_list.push(second);
            }
            OK
        },
    );
    (first_list, second_list)
}

pub fn task1() -> impl Display {
    // let mut answer = 0;
    let (mut first_list, mut second_list) = parse("inputs/day1.txt");
    first_list.sort();
    second_list.sort();
    first_list
        .into_iter()
        .enumerate()
        .map(|(index, number)| number.abs_diff(second_list[index]))
        .sum::<u32>()
}

pub fn task2() -> impl Display {
    let mut answer: usize = 0;
    let (first_list, second_list) = parse("inputs/day1.txt");
    for number in first_list.into_iter() {
        answer += number as usize
            * (&second_list)
                .into_iter()
                .filter(|num| **num == number)
                .count()
    }
    answer
}
