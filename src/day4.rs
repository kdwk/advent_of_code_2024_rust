use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Dimensions {
    width: usize,
    height: usize,
}

#[derive(Debug, Clone, Copy)]
struct Location {
    x: usize,
    y: usize,
}

#[ext]
impl Vec<String> {
    fn dimensions(&self) -> Dimensions {
        Dimensions {
            width: self[0].chars().count(),
            height: self.len(),
        }
    }
    fn get_at_location(&self, at: Location) -> char {
        self[at.y].chars().nth(at.x).unwrap()
    }
    fn diagonal_starting_from(
        &self,
        size: usize,
        location: Location,
        direction: Direction,
    ) -> String {
        let mut result = String::new();
        let dimensions = self.dimensions();
        for i in 0..size {
            let offset_location = location.step(i, direction, dimensions);
            if let Some(offset_location) = offset_location {
                result += &self.get_at_location(offset_location).to_string();
            }
        }
        result
    }
    fn x_starting_from(&self, location: Location, direction: Direction) -> [String; 2] {
        let dimensions = self.dimensions();
        let (other_start, other_direction) = match direction {
            Direction::NE => (location.step(2, Direction::E, dimensions), Direction::NW),
            Direction::SE => (location.step(2, Direction::E, dimensions), Direction::SW),
            Direction::SW => (location.step(2, Direction::W, dimensions), Direction::SE),
            Direction::NW => (location.step(2, Direction::W, dimensions), Direction::NE),
            _ => panic!(),
        };
        [
            self.diagonal_starting_from(3, location, direction),
            if let Some(other_start) = other_start {
                self.diagonal_starting_from(3, other_start, other_direction)
            } else {
                "".to_string()
            },
        ]
    }
}

impl Location {
    const fn step(&self, by: usize, direction: Direction, dimensions: Dimensions) -> Option<Self> {
        let _x = self.x as isize;
        let _y = self.y as isize;
        let _width = dimensions.width as isize;
        let _height = dimensions.height as isize;
        let offset = by as isize;
        match direction {
            Direction::N => Some(Self {
                x: self.x,
                y: if _y > offset - 1 {
                    self.y - by
                } else {
                    return None;
                },
            }),
            Direction::E => Some(Self {
                x: if _x < _width - offset {
                    self.x + by
                } else {
                    return None;
                },
                y: self.y,
            }),
            Direction::S => Some(Self {
                x: self.x,
                y: if _y < _height - offset {
                    self.y + by
                } else {
                    return None;
                },
            }),
            Direction::W => Some(Self {
                x: if _x > offset - 1 {
                    self.x - by
                } else {
                    return None;
                },
                y: self.y,
            }),
            Direction::NE => Some(Self {
                x: if _x < _width - offset {
                    self.x + by
                } else {
                    return None;
                },
                y: if _y > offset - 1 {
                    self.y - by
                } else {
                    return None;
                },
            }),
            Direction::SE => Some(Self {
                x: if _x < _width - offset {
                    self.x + by
                } else {
                    return None;
                },
                y: if _y < _height - offset {
                    self.y + by
                } else {
                    return None;
                },
            }),
            Direction::SW => Some(Self {
                x: if _x > offset - 1 {
                    self.x - by
                } else {
                    return None;
                },
                y: if _y < _height - offset {
                    self.y + by
                } else {
                    return None;
                },
            }),
            Direction::NW => Some(Self {
                x: if _x > offset - 1 {
                    self.x - by
                } else {
                    return None;
                },
                y: if _y > offset - 1 {
                    self.y - by
                } else {
                    return None;
                },
            }),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    N,
    E,
    S,
    W,
    NE,
    SE,
    SW,
    NW,
}

impl Direction {
    const fn values() -> &'static [Self] {
        &[
            Direction::N,
            Direction::E,
            Direction::S,
            Direction::W,
            Direction::NE,
            Direction::SE,
            Direction::SW,
            Direction::NW,
        ]
    }
    const fn non_cardinal() -> &'static [Self] {
        &[Direction::NE, Direction::SE, Direction::SW, Direction::NW]
    }
}

fn parse(file: &str) -> (Vec<String>) {
    let mut lines = vec![];
    with(
        &[Document::at_path(
            current_dir().unwrap().join(file).display(),
            "input",
            Create::No,
        )],
        |d| {
            for line in d["input"].lines()? {
                lines.push(line?);
            }
            OK
        },
    );
    lines
}

pub fn task1() -> impl Display {
    let input = parse("inputs/day4.txt");
    let mut result = 0;
    for (y, line) in (&input).into_iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == 'X' {
                for &direction in Direction::values() {
                    if input.diagonal_starting_from(4, Location { x, y }, direction) == "XMAS" {
                        result += 1;
                    }
                }
            }
        }
    }
    result
}

pub fn task2() -> impl Display {
    let input = parse("inputs/day4.txt");
    let mut result = 0;
    for (y, line) in (&input).into_iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == 'M' {
                for &direction in Direction::non_cardinal() {
                    let x_from_here = input.x_starting_from(Location { x, y }, direction);
                    if (x_from_here[0] == "MAS" || x_from_here[0] == "SAM")
                        && (x_from_here[1] == "MAS" || x_from_here[1] == "SAM")
                    {
                        result += 1;
                    }
                }
            }
        }
    }
    result / 2 // Account for double counting because every X has 2 "M"s
}
