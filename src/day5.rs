use crate::prelude::*;

enum ParseMode {
    OrderingRules,
    Updates,
}

fn parse(file: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut ordering_rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates = vec![];
    with(
        &[Document::at_path(
            current_dir().unwrap().join(file).display(),
            "input",
            Create::No,
        )],
        |d| {
            let mut mode = ParseMode::OrderingRules;
            for line in d["input"].lines()? {
                let line = line?;
                if line == "" {
                    mode = ParseMode::Updates;
                    continue;
                }
                match mode {
                    ParseMode::OrderingRules => {
                        let mut ints = line
                            .split("|")
                            .map(|int_str| int_str.parse::<i32>().unwrap());
                        let int1 = ints.next().unwrap();
                        let int2 = ints.next().unwrap();
                        ordering_rules
                            .entry(int1)
                            .and_modify(|v| v.push(int2))
                            .or_insert(vec![int2]);
                    }
                    ParseMode::Updates => {
                        let ints: Vec<i32> = line
                            .split(",")
                            .map(|int_str| int_str.parse().unwrap())
                            .collect();
                        updates.push(ints);
                    }
                }
            }
            OK
        },
    );
    (ordering_rules, updates)
}

pub fn task1() -> impl Display {
    let (ordering_rules, updates) = parse("day5.txt");
    let mut result = 0;
    for update in updates {
        let mut update_is_correct = true;
        for (index, page) in (&update).into_iter().enumerate() {
            let ordering_rule_for_page = &ordering_rules.get(page);
            if let Some(ordering_rule_for_page) = ordering_rule_for_page {
                let pages_before_this = &update[..index];
                if ordering_rule_for_page
                    .into_iter()
                    .any(|page| pages_before_this.contains(page))
                {
                    update_is_correct = false;
                    break;
                }
            }
        }
        if update_is_correct {
            result += update[update.len() / 2]
        }
    }
    result
}

fn move_up<T: PartialEq<T>>(element: T, vec: &mut Vec<T>) -> usize {
    let index = vec.into_iter().position(|e| e == &element).unwrap();
    let new_index = if index > 0 { index - 1 } else { index };
    vec.swap(index, new_index);
    new_index
}

pub fn task2() -> impl Display {
    let (ordering_rules, updates) = parse("day5.txt");
    let mut result = 0;
    for update in updates {
        let mut update_did_change = false;
        let mut update_clone = update.clone();
        for (index, page) in (&update).into_iter().enumerate() {
            let ordering_rule_for_page = &ordering_rules.get(page);
            if let Some(ordering_rule_for_page) = ordering_rule_for_page {
                let mut pages_before_this = &update[..index];
                while ordering_rule_for_page
                    .into_iter()
                    .any(|page| pages_before_this.contains(page))
                {
                    let new_index = move_up(*page, &mut update_clone);
                    pages_before_this = &update_clone[..new_index];
                    update_did_change = true;
                }
            }
        }
        if update_did_change {
            result += update_clone[update_clone.len() / 2]
        }
    }
    result
}
