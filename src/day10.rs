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

#[derive(Debug, Clone, Default)]
struct Map {
    dimensions: Dimensions,
    locations: HashMap<Location, i32>,
}

impl Map {
    const fn in_bounds(&self, location: Location) -> bool {
        location.x >= 0
            && location.x < self.dimensions.width
            && location.y >= 0
            && location.y < self.dimensions.height
    }
    fn trailheads(&self) -> Vec<Location> {
        self.locations
            .iter()
            .filter(|(_, value)| **value == 0)
            .map(|(location, _)| *location)
            .collect_vec()
    }
    const fn go(&self, location: &Location, direction: Direction) -> Option<Location> {
        let next_location = location.go(direction);
        if self.in_bounds(next_location) {
            Some(next_location)
        } else {
            None
        }
    }
}

impl Index<Location> for Map {
    type Output = i32;
    fn index(&self, index: Location) -> &Self::Output {
        &self.locations[&index]
    }
}

impl Index<&Location> for Map {
    type Output = i32;
    fn index(&self, index: &Location) -> &Self::Output {
        &self.locations[index]
    }
}

fn parse(file: &str) -> Map {
    let mut result = Map::default();
    with(
        &[Document::at_path(
            current_dir().unwrap().join(file).display(),
            "input",
            Create::No,
        )],
        |d| {
            for (y, line) in d["input"].lines()?.into_iter().enumerate() {
                result.dimensions.height = (y + 1) as i32;
                for (x, char) in line?.chars().into_iter().enumerate() {
                    result.dimensions.width = (x + 1) as i32;
                    result
                        .locations
                        .entry(Location {
                            x: x as i32,
                            y: y as i32,
                        })
                        .or_insert(char.to_string().parse().unwrap());
                }
            }
            OK
        },
    );
    result
}

pub fn task1() -> impl Display {
    let map = parse("inputs/day10.txt");
    let mut score = 0;
    for trailhead in map.trailheads() {
        let mut endings: HashSet<Location> = HashSet::new();
        let mut frontier = vec![trailhead];
        while !frontier.is_empty() {
            let exploring = frontier.pop().unwrap();
            if map[exploring] == 9 {
                endings.insert(exploring);
                continue;
            }
            for next in Direction::values()
                .map(|direction| map.go(&exploring, direction))
                .into_iter()
                .filter(Option::is_some)
            {
                if map[next.unwrap()] - map[exploring] == 1 {
                    frontier.push(next.unwrap());
                }
            }
        }
        score += endings.iter().count();
    }
    score
}

pub fn task2() -> impl Display {
    let map = parse("inputs/day10.txt");
    let mut trails: HashSet<Vec<Location>> = HashSet::new();
    for trailhead in map.trailheads() {
        let mut frontier = vec![vec![trailhead]];
        while !frontier.is_empty() {
            let exploring = frontier.pop().unwrap();
            if map[exploring.last()] == 9 {
                trails.insert(exploring);
                continue;
            }
            for next in Direction::values()
                .map(|direction| map.go(exploring.last(), direction))
                .into_iter()
                .filter(|next| next.is_some() && map[next.unwrap()] - map[exploring.last()] == 1)
                .map(|next| next.unwrap())
            {
                let mut new_path = exploring.clone();
                new_path.push(next);
                frontier.push(new_path);
            }
        }
    }
    trails.into_iter().count()
}
