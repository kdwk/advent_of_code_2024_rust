use std::{io, ops::Mul};

use crate::prelude::*;

const BOUNDS: Vector2 = Vector2 { x: 101, y: 103 };

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vector2 {
    x: i32,
    y: i32,
}

impl Vector2 {
    fn wrap_add(self, rhs: Self) -> Self {
        Self {
            x: wrap(self.x + rhs.x, 0..BOUNDS.x),
            y: wrap(self.y + rhs.y, 0..BOUNDS.y),
        }
    }
    fn manhattan_distance_from(&self, rhs: &Self) -> i32 {
        (self.x - rhs.x) + (self.y - rhs.y)
    }
}

impl Mul<i32> for Vector2 {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

type Location = Vector2;
type Velocity = Vector2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Robot {
    location: Location,
    velocity: Velocity,
}

impl Robot {
    fn is_in(&self, quadrant: Vector2) -> bool {
        self.location.x <= quadrant.x
            && self.location.y <= quadrant.y
            && self.location.x != BOUNDS.x / 2
            && self.location.y != BOUNDS.y / 2
    }
}

#[ext]
impl HashSet<Robot> {
    fn safety_factor(&self) -> usize {
        let (q1, mut q1_count) = (
            Vector2 {
                x: BOUNDS.x / 2 - 1,
                y: BOUNDS.y / 2 - 1,
            },
            0,
        );
        let (q2, mut q2_count) = (
            Vector2 {
                x: BOUNDS.x,
                y: BOUNDS.y / 2 - 1,
            },
            0,
        );
        let (q3, mut q3_count) = (
            Vector2 {
                x: BOUNDS.x / 2 - 1,
                y: BOUNDS.y,
            },
            0,
        );
        let (q4, mut q4_count) = (BOUNDS, 0);
        for robot in self {
            if robot.is_in(q1) {
                q1_count += 1;
            } else if robot.is_in(q2) {
                q2_count += 1;
            } else if robot.is_in(q3) {
                q3_count += 1;
            } else if robot.is_in(q4) {
                q4_count += 1;
            }
        }
        q1_count * q2_count * q3_count * q4_count
    }
    fn search_for_cluster(&self) -> bool {
        self.iter().any(|robot1| {
            self.iter()
                .filter(|robot2| (robot1.location.manhattan_distance_from(&robot2.location)) <= 6)
                .count()
                >= 50
        })
    }
    fn to_string(&self) -> String {
        (0..BOUNDS.y)
            .map(|y| {
                (0..BOUNDS.x)
                    .map(|x| {
                        let this_location = Vector2 { x, y };
                        let count = self
                            .iter()
                            .filter(|robot| robot.location == this_location)
                            .count();
                        if count > 0 {
                            count.to_string()
                        } else {
                            ".".to_string()
                        }
                    })
                    .collect_vec()
                    .concat()
                    + "\n"
            })
            .collect_vec()
            .concat()
    }
}

#[ext]
impl String {
    fn as_grid(self) -> Self {
        let width = self.lines().next().unwrap().chars().count();
        "  ".to_string()
            + &(0..width)
                .map(|i| i.to_string().chars().next_back().unwrap().to_string())
                .collect_vec()
                .concat()
            + "\n"
            + &self
                .lines()
                .enumerate()
                .map(|(i, line)| {
                    let line_no = i.to_string().chars().next_back().unwrap().to_string();
                    line_no + " " + line + "\n"
                })
                .collect_vec()
                .concat()
    }
}

fn parse(file: &str) -> HashSet<Robot> {
    let mut result = HashSet::new();
    with(
        &[Document::at_path(
            format!("inputs/{file}"),
            "input",
            Create::No,
        )],
        |d| {
            for line in d["input"].lines()? {
                let (mut px, mut py, mut vx, mut vy) = (0, 0, 0, 0);
                sscanf!(&line?, "p={},{} v={},{}", px, py, vx, vy)?;
                result.insert(Robot {
                    location: Location { x: px, y: py },
                    velocity: Velocity { x: vx, y: vy },
                });
            }
            OK
        },
    );
    result
}

pub fn task1() -> impl Display {
    let robots = parse("day14.txt");
    robots
        .into_iter()
        .map(|robot| Robot {
            location: robot.location.wrap_add(robot.velocity * 100),
            velocity: robot.velocity,
        })
        .collect::<HashSet<Robot>>()
        .safety_factor()
}

pub fn task2() -> impl Display {
    let robots = parse("day14.txt");
    for i in 0..2000 {
        let robots = robots
            .iter()
            .map(|robot| Robot {
                location: robot.location.wrap_add(robot.velocity * i),
                velocity: robot.velocity,
            })
            .collect::<HashSet<_>>();
        if robots.search_for_cluster() {
            i.display();
            robots.to_string().as_grid().display();
            break;
        }
    }
    "TODO: does not work"
}
