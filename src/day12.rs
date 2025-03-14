use std::hash::Hash;

use crate::prelude::*;

#[ext]
impl<T> Vec<T> {
    fn last(&self) -> &T {
        &self[self.len() - 1]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    const fn values() -> [Self; 4] {
        [Self::N, Self::E, Self::S, Self::W]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Dimensions {
    width: i32,
    height: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    const fn go(&self, direction: Direction) -> Self {
        match direction {
            Direction::N => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::E => Self {
                x: self.x + 1,
                y: self.y,
            },
            Direction::S => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::W => Self {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq)]
enum Side {
    Horizontal {
        start_x: usize,
        end_x: usize,
        y: usize,
    },
    Vertical {
        start_y: usize,
        end_y: usize,
        x: usize,
    },
}

impl PartialEq for Side {
    fn eq(&self, other: &Self) -> bool {
        self.forms_line_with(other)
    }
}

impl Side {
    fn horizontal(start_x: usize, end_x: usize, y: usize) -> Self {
        Self::Horizontal { start_x, end_x, y }
    }
    fn vertical(start_y: usize, end_y: usize, x: usize) -> Self {
        Self::Vertical { start_y, end_y, x }
    }
    fn forms_line_with(&self, other: &Self) -> bool {
        match self {
            Self::Horizontal {
                start_x: this_start_x,
                end_x: this_end_x,
                y: this_y,
            } => match other {
                Self::Horizontal {
                    start_x: other_start_x,
                    end_x: other_end_x,
                    y: other_y,
                } => {
                    this_y == other_y
                        && (this_start_x == other_end_x || this_end_x == other_start_x)
                        || (this_start_x == other_start_x && this_end_x == other_end_x)
                }
                Self::Vertical {
                    start_y: _,
                    end_y: _,
                    x: _,
                } => false,
            },
            Self::Vertical {
                start_y: this_start_y,
                end_y: this_end_y,
                x: this_x,
            } => match other {
                Self::Horizontal {
                    start_x: _,
                    end_x: _,
                    y: _,
                } => false,
                Self::Vertical {
                    start_y: other_start_y,
                    end_y: other_end_y,
                    x: other_x,
                } => {
                    this_x == other_x
                        && (this_start_y == other_end_y || this_end_y == other_start_y)
                        || (this_start_y == other_start_y && this_end_y == other_end_y)
                }
            },
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Map {
    dimensions: Dimensions,
    locations: HashMap<Location, char>,
}

impl Map {
    const fn in_bounds(&self, location: Location) -> bool {
        location.x >= 0
            && location.x < self.dimensions.width
            && location.y >= 0
            && location.y < self.dimensions.height
    }
    const fn go(&self, location: &Location, direction: Direction) -> Option<Location> {
        let next_location = location.go(direction);
        if self.in_bounds(next_location) {
            Some(next_location)
        } else {
            None
        }
    }
    fn surrounding_notthis_count(&self, location: &Location) -> usize {
        let this = self[location];
        Direction::values()
            .map(|direction| self.go(location, direction))
            .into_iter()
            .filter(|neighbour| {
                if let Some(neighbour) = neighbour {
                    self[neighbour] != this
                } else {
                    true
                }
            })
            .count()
    }
    fn surrounding_horizontal_sides(&self, location: &Location) -> Vec<Side> {
        let this = self[location];
        let mut result = vec![];
        if let Some(neighbour) = self.go(location, Direction::N) {
            if self[neighbour] != this {
                result.push(Side::horizontal(
                    location.x as usize,
                    (location.x + 1) as usize,
                    location.y as usize,
                ));
            }
        } else {
            result.push(Side::horizontal(
                location.x as usize,
                (location.x + 1) as usize,
                location.y as usize,
            ));
        }
        if let Some(neighbour) = self.go(location, Direction::S) {
            if self[neighbour] != this {
                result.push(Side::horizontal(
                    location.x as usize,
                    (location.x + 1) as usize,
                    (location.y + 1) as usize,
                ));
            }
        } else {
            result.push(Side::horizontal(
                location.x as usize,
                (location.x + 1) as usize,
                (location.y + 1) as usize,
            ));
        }
        result
    }
    fn surrounding_vertical_sides(&self, location: &Location) -> Vec<Side> {
        let this = self[location];
        let mut result = vec![];
        if let Some(neighbour) = self.go(location, Direction::E) {
            if self[neighbour] != this {
                result.push(Side::vertical(
                    location.y as usize,
                    (location.y + 1) as usize,
                    (location.x + 1) as usize,
                ));
            }
        } else {
            result.push(Side::vertical(
                location.y as usize,
                (location.y + 1) as usize,
                (location.x + 1) as usize,
            ));
        }
        if let Some(neighbour) = self.go(location, Direction::W) {
            if self[neighbour] != this {
                result.push(Side::vertical(
                    location.y as usize,
                    (location.y + 1) as usize,
                    location.x as usize,
                ));
            }
        } else {
            result.push(Side::vertical(
                location.y as usize,
                (location.y + 1) as usize,
                location.x as usize,
            ));
        }
        result
    }
    /// Return one Location that has the same food as current location
    fn surrounding_this(&self, location: Location) -> impl Iterator<Item = Location> + use<'_> {
        let this = self[location];
        Direction::values()
            .map(|direction| self.go(&location, direction))
            .into_iter()
            .filter(Option::is_some)
            .map(|opt| opt.unwrap())
            .filter(move |neighbour| self[neighbour] == this)
    }
}

impl Index<Location> for Map {
    type Output = char;
    fn index(&self, index: Location) -> &Self::Output {
        &self.locations[&index]
    }
}

impl Index<&Location> for Map {
    type Output = char;
    fn index(&self, index: &Location) -> &Self::Output {
        &self.locations[index]
    }
}

#[ext]
impl HashSet<Location> {
    fn area(&self) -> usize {
        self.len()
    }
    fn perimeter(&self, map: &Map) -> usize {
        self.iter()
            .map(|location| map.surrounding_notthis_count(location))
            .sum()
    }
}

fn parse(file: &str) -> Map {
    let mut map = Map::default();
    with(
        &[Document::at_path(
            current_dir().unwrap().join(file).display(),
            "input",
            Create::No,
        )],
        |d| {
            for (y, line) in d["input"].lines()?.into_iter().enumerate() {
                map.dimensions.height = (y + 1) as i32;
                for (x, char) in line?.chars().into_iter().enumerate() {
                    map.dimensions.width = (x + 1) as i32;
                    let location = Location {
                        x: x as i32,
                        y: y as i32,
                    };
                    map.locations.entry(location).or_insert(char);
                }
            }
            OK
        },
    );
    map
}

pub fn task1() -> impl Display {
    let map = parse("inputs/day12.txt");
    let mut result = 0;
    let mut explored: HashSet<Location> = HashSet::new();
    for location in map.locations.keys() {
        if !explored.contains(location) {
            let mut region_perimeter = 0;
            let mut region_area = 0;
            let mut frontier: Vec<Location> = vec![*location];
            while !frontier.is_empty() {
                let same_region_location = frontier.pop().unwrap();
                if explored.contains(&same_region_location) {
                    continue;
                }
                if map[same_region_location] == map[location] {
                    region_perimeter += map.surrounding_notthis_count(&same_region_location);
                    region_area += 1;
                }
                frontier.extend(map.surrounding_this(same_region_location));
                explored.insert(same_region_location);
            }
            result += region_area * region_perimeter;
        }
    }
    result
}

pub fn task2() -> impl Display {
    let map = parse("inputs/day12-test.txt");
    let mut result = 0;
    let mut explored: HashSet<Location> = HashSet::new();
    for location in map.locations.keys() {
        if !explored.contains(location) {
            let mut region_area = 0;
            let mut sides_in_region: HashSet<Side> = HashSet::new();
            let mut frontier: Vec<Location> = vec![*location];
            while !frontier.is_empty() {
                let same_region_location = frontier.pop().unwrap();
                if explored.contains(&same_region_location) {
                    continue;
                }
                if map[same_region_location] == map[location] {
                    region_area += 1;
                    sides_in_region.extend(map.surrounding_vertical_sides(&same_region_location));
                }
                frontier.extend(map.surrounding_this(same_region_location));
                explored.insert(same_region_location);
            }
            let region_perimeter = sides_in_region.iter().count();
            result += region_area * region_perimeter;
            println!(
                "Region {}:\nsides\n\t{:?}\narea\n\t{}\nprice\n\t{}",
                map[location], sides_in_region, region_area, result
            );
        }
    }
    result
}
