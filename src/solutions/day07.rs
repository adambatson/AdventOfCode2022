use std::{iter::Peekable, str::Split};

const INPUT: &'static str = include_str!("../../inputs/day07.txt");

#[derive(Debug, Copy, Clone)]
enum FileType {
    File,
    Directory
}

#[derive(Debug, Clone)]
struct FileSystem {
    name: String,
    size: usize,
    fileType: FileType,
    children: Vec<Box<&'static FileSystem>>,
    parent: Option<Box<&'static FileSystem>>
}

fn create_file_node(name: String, size: usize, file_type: FileType, parent: Option<&'static FileSystem>) -> FileSystem {
    let p: Option<Box<&'static FileSystem>> = match parent {
        Some(fs) => Some(Box::new(fs)),
        None => None
    };
    return FileSystem { name: name, size: size, fileType: file_type, children: Vec::new(), parent: p };
}

fn handle_list_command(iter: &mut Peekable<Split<&str>>, pwd: &FileSystem) {

}

fn build_file_directory(input: &'static str) -> FileSystem {
    let mut pwd: FileSystem = create_file_node("/".to_string(), 0, FileType::Directory, None).to_owned();

    let mut iter = input.split("\n").collect::<Vec<&str>>();

    for (idx, window) in iter.windows(iter.len() - 1).enumerate() {
        if idx == 0 {
            // will always be $ cd /
            continue;
        }
        if window[0] == "$ ls" {
            let mut win_idx = 1;
            loop {
                let line = window[win_idx];
                if line.starts_with("dir") {
                    let splt = line.split("dir ");
                    let name = splt.last().unwrap().to_string();
                    let child: &'static FileSystem = &create_file_node(name, 0, FileType::Directory, Some(&pwd));
                    pwd.children.push(Box::new(child));
                }
            }
        }

    }

    return pwd;
}

pub fn solve() {
    println!("Solving Day 7!");
}