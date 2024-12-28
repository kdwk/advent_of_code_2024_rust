use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Dimensions {
    width: isize,
    height: isize,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    N,
    E,
    S,
    W,
}
impl Default for Direction {
    fn default() -> Self {
        Direction::N
    }
}
impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Guard {
    position: Position,
    direction: Direction,
}
impl Guard {
    fn walk(&self) -> Position {
        match self.direction {
            Direction::N => Position {
                x: self.position.x,
                y: self.position.y - 1,
            },
            Direction::E => Position {
                x: self.position.x + 1,
                y: self.position.y,
            },
            Direction::S => Position {
                x: self.position.x,
                y: self.position.y + 1,
            },
            Direction::W => Position {
                x: self.position.x - 1,
                y: self.position.y,
            },
        }
    }
}

#[derive(Debug, Clone, Default)]
struct PatrolMap {
    dimensions: Dimensions,
    obstacles: HashSet<Position>,
    guard: Guard,
    unique_positions: HashSet<Position>,
}

impl PatrolMap {
    fn has_obstacle_at(&self, position: Position) -> bool {
        self.obstacles.contains(&position)
    }
    fn guard_in_bound(&self) -> bool {
        self.guard.position.x >= 0
            && self.guard.position.x < self.dimensions.width
            && self.guard.position.y >= 0
            && self.guard.position.y < self.dimensions.height
    }
    fn guard_met_obstacle(&self) -> bool {
        let next_position = self.guard.walk();
        if self.has_obstacle_at(next_position) {
            true
        } else {
            false
        }
    }
    fn guard_walk(&mut self) {
        while self.guard_met_obstacle() {
            self.guard.direction = self.guard.direction.turn_right();
        }
        self.guard.position = self.guard.walk();
        self.unique_positions.insert(self.guard.position);
    }
}

fn parse(file: &str) -> PatrolMap {
    let mut result = PatrolMap::default();
    with(
        &[Document::at_path(
            current_dir().unwrap().join(file).display(),
            "input",
            Create::No,
        )],
        |d| {
            result.dimensions = Dimensions {
                width: d["input"]
                    .lines()?
                    .next()
                    .unwrap()?
                    .chars()
                    .into_iter()
                    .count() as isize,
                height: d["input"].lines()?.count() as isize,
            };
            for (y, line) in d["input"].lines()?.enumerate() {
                for (x, char) in line?.chars().enumerate() {
                    let position = Position {
                        x: x as isize,
                        y: y as isize,
                    };
                    if char == '#' {
                        result.obstacles.insert(position);
                    } else if char != '.' {
                        result.guard = Guard {
                            position,
                            direction: if char == '^' {
                                Direction::N
                            } else if char == '>' {
                                Direction::E
                            } else if char == 'v' {
                                Direction::S
                            } else {
                                Direction::W
                            },
                        }
                    }
                }
            }
            OK
        },
    );
    result
}

pub fn task1() -> impl Display {
    let mut map = parse("day6.txt");
    while map.guard_in_bound() {
        map.guard_walk();
    }
    map.unique_positions.into_iter().count() - 1
}
