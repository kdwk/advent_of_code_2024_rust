use crate::prelude::*;

fn parse(file: &str) -> Vec<Vec<i32>> {
    let mut result = vec![];
    with(
        &[Document::at_path(
            current_dir().unwrap().join(file).display(),
            "input",
            Create::No,
        )],
        |d| {
            for line in d["input"].lines()? {
                let report: Vec<i32> = line?
                    .split_whitespace()
                    .map(|str| str.parse().unwrap())
                    .collect();
                result.push(report);
            }
            OK
        },
    );
    result
}

#[derive(Debug, Clone, Copy)]
enum Order {
    Asc,
    Desc,
}

fn is_safe(report: &Vec<i32>) -> bool {
    let order = if report[0] < report[1] {
        Order::Asc
    } else {
        Order::Desc
    };
    for index in 0..report.len() - 1 {
        let current = report[index];
        let next = report[index + 1];
        let difference = next - current;
        match order {
            Order::Asc if difference >= 1 && difference <= 3 => {}
            Order::Desc if difference <= -1 && difference >= -3 => {}
            _ => return false,
        }
    }
    true
}

fn safe(start: i32, end: i32, order: Order) -> bool {
    let difference = end - start;
    match order {
        Order::Asc if difference >= 1 && difference <= 3 => true,
        Order::Desc if difference <= -1 && difference >= -3 => true,
        _ => false,
    }
}

fn order(report: &Vec<i32>) -> Order {
    if safe(report[0], report[1], Order::Asc)
    // || safe(report[0], report[2], Order::Asc)
    {
        Order::Asc
    } else {
        Order::Desc
    }
}

fn is_safe_with_dampener(report: &Vec<i32>) -> bool {
    let order = order(report);
    let mut bad = 0;
    for index in 0..report.len() - 1 {
        if !safe(report[index], report[index + 1], order) {
            let cannot_skip = index >= report.len() - 2;
            if cannot_skip {
                bad += 1;
            } else if !safe(report[index], report[index + 2], order) {
                bad += 1;
            }
        }
    }
    bad > 1
}

pub fn task1() -> impl Display {
    let reports = parse("day2.txt");
    reports.into_iter().filter(is_safe).count()
}

pub fn task2() -> impl Display {
    let reports = parse("day2.txt");
    reports.into_iter().filter(is_safe_with_dampener).count()
}
