use crate::prelude::*;

#[derive(Debug)]
struct Equation {
    answer: i64,
    numbers: Vec<i64>,
}

fn parse(file: &str) -> Vec<Equation> {
    let mut result = vec![];
    with(
        &[Document::at_path(
            current_dir().unwrap().join(file).display(),
            "input",
            Create::No,
        )],
        |d| {
            for line in d["input"].lines()? {
                let line = line?;
                let [answer, numbers, ..] = line.split(": ").collect::<Vec<&str>>()[..] else {
                    panic!()
                };
                let answer: i64 = answer.parse().unwrap();
                let numbers: Vec<i64> = numbers.split(" ").map(|s| s.parse().unwrap()).collect();
                result.push(Equation { answer, numbers });
            }
            OK
        },
    );
    result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Operator {
    Add,
    Mul,
    Concat,
}

impl Operator {
    const fn values() -> [Self; 3] {
        [Self::Add, Self::Mul, Self::Concat]
    }
}

fn permutations(no_of_numbers: usize) -> Vec<Vec<Operator>> {
    repeat_n(Operator::values(), no_of_numbers)
        .multi_cartesian_product()
        .unique()
        .collect()
}

pub fn task1() -> impl Display {
    let equations = parse("inputs/day7.txt");
    equations
        .par_iter()
        .map(|equation| {
            for permutation in permutations(equation.numbers.len() - 1) {
                let mut intermediate = equation.numbers[0];
                for (index, operator) in permutation.clone().into_iter().enumerate() {
                    match operator {
                        Operator::Add => intermediate += equation.numbers[index + 1],
                        Operator::Mul => intermediate *= equation.numbers[index + 1],
                        Operator::Concat => {
                            intermediate = (intermediate.to_string()
                                + &equation.numbers[index + 1].to_string())
                                .parse()
                                .unwrap()
                        }
                    }
                }
                if intermediate == equation.answer {
                    return equation.answer;
                }
            }
            0
        })
        .sum::<i64>()
}
