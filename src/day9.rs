use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum DiskObj {
    File { size: i64, id: i64 },
    Free { size: i64 },
}

impl DiskObj {
    fn is_free(&self) -> bool {
        match self {
            DiskObj::File { size: _, id: _ } => false,
            DiskObj::Free { size: _ } => true,
        }
    }
    fn size(&self) -> i64 {
        match self {
            DiskObj::File { size, id: _ } => *size,
            DiskObj::Free { size } => *size,
        }
    }
    fn resize(&mut self, new_size: i64) {
        match self {
            DiskObj::File { size, id: _ } => *size = new_size,
            DiskObj::Free { size } => *size = new_size,
        }
    }
}

type Disk = Vec<DiskObj>;

type DiskRepr = Vec<Option<i64>>;

#[ext]
impl Disk {
    fn repr(&self) -> DiskRepr {
        let mut result = vec![];
        for disk_obj in self.iter() {
            match disk_obj {
                DiskObj::File { size, id } => {
                    for _ in 0..*size {
                        result.push(Some(*id));
                    }
                }
                DiskObj::Free { size } => {
                    for _ in 0..*size {
                        result.push(None);
                    }
                }
            }
        }
        result
    }
}

#[ext]
impl DiskRepr {
    fn to_string(&self) -> String {
        self.iter()
            .map(|id| {
                if let Some(id) = id {
                    id.to_string()
                } else {
                    ".".to_string()
                }
            })
            .collect()
    }
    fn checksum(&self) -> i64 {
        self.iter()
            .enumerate()
            .map(|(index, id)| index as i64 * id.unwrap_or(0))
            .sum()
    }
}

fn parse(file: &str) -> Disk {
    let mut disk = vec![];
    with(
        &[Document::at_path(
            current_dir().unwrap().join(file).display(),
            "input",
            Create::No,
        )],
        |d| {
            let mut id = 0;
            for (index, char) in d["input"].content()?.chars().into_iter().enumerate() {
                if index % 2 == 0 {
                    // File
                    disk.push(DiskObj::File {
                        size: char.to_string().parse().unwrap(),
                        id: id,
                    });
                    id += 1;
                } else {
                    // Free space
                    disk.push(DiskObj::Free {
                        size: char.to_string().parse().unwrap(),
                    });
                }
            }
            OK
        },
    );
    disk
}

pub fn task1() -> impl Display {
    let mut disk = parse("inputs/day9.txt").repr();
    let disk_len = disk.len();
    let mut disk_rev = disk.clone().into_iter().rev();
    let mut prev_find_result = disk_len - 1;
    for index in 0..disk_len - 1 {
        if index >= prev_find_result {
            break;
        }
        if let None = disk[index] {
            let find_result = prev_find_result - disk_rev.position(|int| int.is_some()).unwrap(); // .position() returns offset of hit from current iterator position
            if index < find_result {
                disk.swap(index, find_result);
                prev_find_result = find_result - 1; // Advance the iterator so it doesn't become stuck at a position
            }
        }
    }
    disk.checksum()
}

pub fn task2() -> impl Display {
    let mut disk = parse("inputs/day9.txt");
    let files: Vec<_> = disk
        .clone()
        .into_iter()
        .filter(|disk_obj| !disk_obj.is_free())
        .rev()
        .collect();
    for file in files {
        let file_pos = disk.iter().position(|disk_obj| disk_obj == &file).unwrap();
        let free_space_pos = disk
            .iter()
            .position(|disk_obj| disk_obj.is_free() && disk_obj.size() >= file.size());
        if let Some(free_space_pos) = free_space_pos {
            let free_space = disk[free_space_pos];
            if free_space_pos < file_pos {
                disk[file_pos] = DiskObj::Free { size: file.size() };
                disk[free_space_pos].resize(free_space.size() - file.size());
                disk.insert(free_space_pos, file);
            }
        }
    }
    disk.repr().checksum()
}
