use crate::prelude::*;

#[ext]
impl f64 {
    fn is_int(self) -> bool {
        self.floor() == self
    }
}

#[derive(Debug, Clone, Copy)]
struct Vector2 {
    x: i64,
    y: i64,
}

impl Vector2 {
    fn add_big_num(self) -> Self {
        Self {
            x: self.x + 10_000_000_000_000,
            y: self.y + 10_000_000_000_000,
        }
    }
}

type Button = Vector2;

type Prize = Vector2;

// 94x + 22y = 8400
// 34x + 67y = 5400
struct Equation((Vector2, f64), (Vector2, f64));

impl Equation {
    fn solve(&self) -> (f64, f64) {
        let &Equation(equation1, equation2) = self;
        let (Vector2 { x, y }, c) = equation1;
        let (a, b) = (x as f64, y as f64);
        let (Vector2 { x, y }, f) = equation2;
        let (d, e) = (x as f64, y as f64);
        let (g, h) = (b * d - e * a, c * d - f * a);
        let y = h / g;
        let x = (c - b * y) / a;
        (x, y)
    }
    // fn solve(&self) -> (f64, f64) {
    //     let &Equation(equation1, equation2) = self;
    //     let (Vector2 { x, y }, xprize) = equation1;
    //     let (ax, bx) = (x as f64, y as f64);
    //     let (Vector2 { x, y }, yprize) = equation2;
    //     let (ay, by) = (x as f64, y as f64);

    //     let ax_with_by = ax * by;
    //     let xprize_with_by = xprize * by;

    //     let ay_with_bx = ay * bx;
    //     let yprize_with_bx = yprize * bx;

    //     let x = (xprize_with_by - yprize_with_bx) / (ax_with_by - ay_with_bx);
    //     let y = (yprize - ay * x) / by;
    //     (x, y)
    // }
}

fn parse(file: &str) -> Vec<(Button, Button, Prize)> {
    let mut result = vec![];
    with(
        &[Document::at_path(
            PathBuf::from(format!("inputs/{file}")).display(),
            "file",
            Create::No,
        )],
        |d| {
            for (line1, line2, line3, _) in d["file"].lines()?.tuples() {
                let [but_a_x, but_a_y] = line1?
                    .split(", ")
                    .map(|s| s.split("+").nth(1).unwrap().parse().unwrap())
                    .collect_vec()[..]
                else {
                    Err("A")?
                };
                let [but_b_x, but_b_y] = line2?
                    .split(", ")
                    .map(|s| s.split("+").nth(1).unwrap().parse().unwrap())
                    .collect_vec()[..]
                else {
                    Err("B")?
                };
                let [prize_x, prize_y] = line3?
                    .split(", ")
                    .map(|s| s.split("=").nth(1).unwrap().parse().unwrap())
                    .collect_vec()[..]
                else {
                    Err("Prize")?
                };
                result.push((
                    Button {
                        x: but_a_x,
                        y: but_a_y,
                    },
                    Button {
                        x: but_b_x,
                        y: but_b_y,
                    },
                    Prize {
                        x: prize_x,
                        y: prize_y,
                    },
                ));
            }
            OK
        },
    );
    result
}

pub fn task1() -> impl Display {
    let machines = parse("day13.txt");
    machines
        .into_iter()
        .map(|(a, b, prize)| {
            let (a_push_times, b_push_times) = Equation(
                (Vector2 { x: a.x, y: b.x }, prize.x as f64),
                (Vector2 { x: a.y, y: b.y }, prize.y as f64),
            )
            .solve();
            if a_push_times.is_int() && b_push_times.is_int() {
                Some(a_push_times as i64 * 3 + b_push_times as i64)
            } else {
                None
            }
        })
        .filter_map(identity)
        .sum::<i64>()
}

pub fn task2() -> impl Display {
    let machines = parse("day13.txt");
    machines
        .into_iter()
        .map(|(a, b, prize)| {
            let prize = prize.add_big_num();
            let (a_push_times, b_push_times) = Equation(
                (Vector2 { x: a.x, y: b.x }, prize.x as f64),
                (Vector2 { x: a.y, y: b.y }, prize.y as f64),
            )
            .solve();
            if a_push_times.is_int() && b_push_times.is_int() {
                Some(a_push_times as i64 * 3 + b_push_times as i64)
            } else {
                None
            }
        })
        .filter_map(identity)
        .sum::<i64>()
}
