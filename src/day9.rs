use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum DiskObj {
    File { size: i32 },
    Free { size: i32 },
}

type Disk = Vec<DiskObj>;

fn parse(file: &str) -> Disk {
    let mut disk = vec![];
    with(
        &[Document::at_path(
            current_dir().unwrap().join(file).display(),
            "input",
            Create::No,
        )],
        |d| {
            for (index, char) in d["input"].content()?.chars().into_iter().enumerate() {
                if index % 2 == 0 {
                    // File
                    disk.push(DiskObj::File {
                        size: char.to_string().parse().unwrap(),
                    });
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
    let mut disk = parse("inputs/day9-test.txt");
    let disk_rev = disk.clone().into_iter().rev();
    for disk_obj in &mut disk {

    }
    ""
}
