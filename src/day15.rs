use crate::prelude::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
struct Vector2 {
    x: i32,
    y: i32,
}

type Location = Vector2;
type Dimensions = Vector2;

impl Location {
    fn increment(&self, direction: Direction) -> Self {
        Self {
            x: match direction {
                Direction::N | Direction::S => 0,
                _ => 1,
            },
            y: match direction {
                Direction::E | Direction::W => 0,
                _ => 1,
            },
        }
    }
}

#[derive(Debug, Default)]
struct Map {
    dimensions: Dimensions,
    walls: HashSet<Location>,
    boxes: HashSet<Location>,
    robot: Location,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.dimensions.y {
            for x in 0..self.dimensions.x {
                let location = Location { x, y };
                if self.is_wall(location) {
                    write!(f, "#")?;
                } else if self.is_box(location) {
                    write!(f, "O")?;
                } else if location == self.robot {
                    write!(f, "@")?;
                } else {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[ext]
impl bool {
    #[inline]
    fn toggle(&mut self) {
        match self {
            true => {
                *self = false;
            }
            false => {
                *self = true;
            }
        }
    }
}

impl Map {
    fn is_box(&self, at: Location) -> bool {
        debug_assert!(self.is_in_bounds(at));
        self.boxes.contains(&at)
    }
    fn is_wall(&self, at: Location) -> bool {
        debug_assert!(self.is_in_bounds(at));
        self.walls.contains(&at)
    }
    fn is_in_bounds(&self, at: Location) -> bool {
        at.x < self.dimensions.x && at.x >= 0 && at.y < self.dimensions.y && at.y >= 0
    }
    fn boxes_until_wall(
        &self,
        from: Location,
        direction: Direction,
        mut acc: Vec<Option<Location>>,
    ) -> Vec<Option<Location>> {
        let adjacent = from.increment(direction);
        if self.is_wall(adjacent) {
            acc
        } else {
            let adjacent_is_box = self.is_box(adjacent);
            if adjacent_is_box {
                acc.push(Some(adjacent));
            } else {
                acc.push(None);
            }
            self.boxes_until_wall(adjacent, direction, acc)
        }
    }
    // fn distance_from_wall(&self, from: Location, direction: Direction) -> i32 {
    //     let adjacent = from.increment(direction);
    //     if self.is_wall(adjacent) {
    //         0
    //     } else {
    //         1 + self.distance_from_wall(adjacent, direction)
    //     }
    // }
    // fn has_boxes_until_wall(&self, from: Location, direction: Direction) -> bool {
    //     let adjacent = from.increment(direction);
    //     if self.is_wall(adjacent) {
    //         true
    //     } else {
    //         let adjacent_is_box = self.is_box(adjacent);
    //         adjacent_is_box && self.has_boxes_until_wall(adjacent, direction)
    //     }
    // }
    fn move_robot(&mut self, direction: Direction) {
        let boxes_until_wall = self.boxes_until_wall(self.robot, direction, vec![]);
        if boxes_until_wall.iter().any(Option::is_none) {
            for b in boxes_until_wall {
                match b {
                    Some(b) => {
                        self.boxes.remove(&b);
                        self.boxes.insert(b.increment(direction));
                    }
                    _ => break,
                }
            }
            self.robot = self.robot.increment(direction);
        }
    }
    fn gps_sum(&self) -> i32 {
        self.boxes.iter().map(|b| b.x + b.y * 100).sum()
    }
}

fn parse(file: &str) -> (Map, Vec<Direction>) {
    let mut map = Map::default();
    let mut directions = vec![];
    with(
        &[Document::at_path(
            format!("inputs/{file}"),
            "input",
            Create::No,
        )],
        |d| {
            let mut is_parsing_map = false;
            for (y, line) in d["input"].lines()?.into_iter().enumerate() {
                let line = line?;
                if line.clone().chars().all(|c| c == '#') {
                    is_parsing_map.toggle();
                }
                if is_parsing_map {
                    for (x, c) in line.chars().into_iter().enumerate() {
                        let v = Vector2 {
                            x: x as i32,
                            y: y as i32,
                        };
                        match c {
                            '#' => {
                                map.walls.insert(v);
                            }
                            'O' => {
                                map.boxes.insert(v);
                            }
                            '@' => {
                                map.robot = v;
                            }
                            _ => {}
                        }
                        map.dimensions = Vector2 {
                            x: v.x + 1,
                            y: v.y + 1,
                        };
                    }
                } else {
                    for c in line.chars() {
                        match c {
                            '^' => {
                                directions.push(Direction::N);
                            }
                            '>' => {
                                directions.push(Direction::E);
                            }
                            'v' => {
                                directions.push(Direction::S);
                            }
                            '<' => {
                                directions.push(Direction::W);
                            }
                            _ => {}
                        }
                    }
                }
            }
            OK
        },
    );
    (map, directions)
}

pub fn task1() -> impl Display {
    let (mut map, directions) = parse("day15-test.txt");
    println!("{map}");
    for direction in directions {
        map.move_robot(direction);
    }
    map.gps_sum()
}

pub fn task2() -> impl Display {
    "Not implemented"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_map_test() {
        assert_eq!(
            parse("day15-test.txt").0.to_string(),
            "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########"
        )
    }
}
