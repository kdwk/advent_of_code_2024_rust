use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
struct Dimensions {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    /// Euclidean distance
    fn distance(a: &Location, b: &Location) -> f64 {
        (((a.x - b.x).pow(2) + (a.y - b.y).pow(2)) as f64).powf(0.5)
    }
    fn in_single_line(a: &Location, b: &Location, c: &Location) -> bool {
        // If any two points are the same then there are just two points left which must form a single line
        if a == b || b == c || a == c {
            return true;
        }
        let slope1 = (b.y - a.y) as f64 / (b.x - a.x) as f64;
        let slope2 = (c.y - a.y) as f64 / (c.x - a.x) as f64;
        slope1 == slope2
    }
}

#[derive(Debug, Clone, Default)]
struct AntennaMap {
    dimensions: Dimensions,
    antennas: HashMap<char, Vec<Location>>, // Grouped by frequency
    antinodes: HashSet<Location>,
}

impl Display for AntennaMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.dimensions.y {
            let mut line = "".to_string();
            for x in 0..self.dimensions.x {
                let location = Location { x, y };
                line += if self.antinodes.contains(&location) {
                    "#"
                } else {
                    "."
                }
            }
            writeln!(f, "{line}")?;
        }
        Ok(())
    }
}

fn parse(file: &str) -> AntennaMap {
    let mut result = AntennaMap::default();
    with(
        &[Document::at_path(
            current_dir().unwrap().join(file).display(),
            "input",
            Create::No,
        )],
        |d| {
            for (y, line) in d["input"].lines()?.into_iter().enumerate() {
                let y = y as i32;
                result.dimensions.y = y + 1;
                for (x, char) in line?.chars().into_iter().enumerate() {
                    let x = x as i32;
                    result.dimensions.x = x + 1;
                    if char != '.' {
                        result
                            .antennas
                            .entry(char)
                            .and_modify(|v| v.push(Location { x, y }))
                            .or_insert(vec![Location { x, y }]);
                    }
                }
            }
            OK
        },
    );
    result
}

fn task(
    mut map: AntennaMap,
    is_antinode: impl Fn(Location, Vec<&Location>) -> bool,
) -> impl Display {
    for y in 0..map.dimensions.y {
        for x in 0..map.dimensions.x {
            let location = Location { x, y };
            if !map.antinodes.contains(&location) {
                for frequency in map.antennas.keys() {
                    if map
                        .antennas
                        .get(frequency)
                        .unwrap()
                        .into_iter()
                        .combinations(2)
                        .any(|antenna_set| is_antinode(location, antenna_set))
                    {
                        map.antinodes.insert(location);
                    }
                }
            }
        }
    }
    map.antinodes.into_iter().count()
}

pub fn task1() -> impl Display {
    let map = parse("day8.txt");
    task(map, |location, antenna_set| {
        let distance_from_1 = Location::distance(&location, antenna_set[0]);
        let distance_from_2 = Location::distance(&location, antenna_set[1]);
        Location::in_single_line(&location, antenna_set[0], antenna_set[1])
            && (distance_from_1 == distance_from_2 * 2. || distance_from_2 == distance_from_1 * 2.)
    })
}

pub fn task2() -> impl Display {
    let map = parse("day8.txt");
    task(map, |location, antenna_set| {
        Location::in_single_line(&location, antenna_set[0], antenna_set[1])
    })
}
