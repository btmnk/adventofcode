use core::panic;
use std::fmt;

use super::{
    file_path::FilePath,
    parse_line::{parse_line, ParsedLine},
};

pub struct File {
    pub name: String,
    pub size: i32,
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.size)
    }
}

#[derive(Debug)]
pub struct FileTree {
    pub name: String,
    pub files: Vec<File>,
    pub directories: Vec<FileTree>,
}

impl FileTree {
    pub fn from_lines(lines: Vec<String>) -> FileTree {
        let mut file_tree = FileTree {
            name: "/".to_string(),
            files: vec![],
            directories: vec![],
        };

        let mut current_file_path = FilePath {
            path: String::from("/"),
        };

        lines.iter().for_each(|line| match parse_line(line) {
            ParsedLine::MoveToRoot => {
                current_file_path.path = "/".to_string();
            }
            ParsedLine::MoveInto => {
                let (_, name) = line.rsplit_once(" ").unwrap();
                current_file_path.move_into(name);
            }
            ParsedLine::MoveOut => {
                current_file_path.move_out();
            }
            ParsedLine::File => {
                let (size, name) = line.split_once(" ").unwrap();
                file_tree.add_file(
                    &current_file_path.path,
                    File {
                        name: name.to_string(),
                        size: size.parse::<i32>().unwrap(),
                    },
                );
            }
            ParsedLine::Directory => {
                let (_, name) = line.split_once(" ").unwrap();
                file_tree.add_dir(&current_file_path.path, name);
            }
            _ => {}
        });

        return file_tree;
    }

    pub fn add_file(&mut self, path: &str, file: File) {
        if path == "/" {
            self.files.push(file);
            return;
        }

        let target_tree = self.get_dir(path);
        target_tree.files.push(file);
    }

    pub fn add_dir(&mut self, path: &str, dirname: &str) {
        let new_file_tree = FileTree {
            name: dirname.to_string(),
            files: vec![],
            directories: vec![],
        };

        if path == "/" {
            self.directories.push(new_file_tree);
            return;
        }

        let target_tree = self.get_dir(path);
        target_tree.directories.push(new_file_tree);
    }

    pub fn get_directory_size(&self) -> i32 {
        let file_sizes_sum = self.files.iter().fold(0, |sum, file| sum + file.size);
        let directory_sizes = self
            .directories
            .iter()
            .fold(0, |sum, directory| sum + directory.get_directory_size());

        return file_sizes_sum + directory_sizes;
    }

    pub fn get_all_directory_sizes(&self) -> Vec<i32> {
        let mut directory_sizes = vec![];

        directory_sizes.push(self.get_directory_size());

        let children_sizes: Vec<i32> = self.directories.iter().fold(vec![], |sizes, dir| {
            let mut x = sizes.clone();
            x.extend(dir.get_all_directory_sizes());
            return x;
        });

        directory_sizes.extend(children_sizes);

        return directory_sizes;
    }

    pub fn get_dir(&mut self, path: &str) -> &mut FileTree {
        let directory_names = path
            .split("/")
            .filter(|dir| *dir != "")
            .collect::<Vec<&str>>();

        directory_names.into_iter().fold(self, |tree, dirname| {
            let target_dir_opt = tree.directories.iter_mut().find(|dir| dir.name == dirname);

            if let Some(target_dir) = target_dir_opt {
                return target_dir;
            } else {
                panic!("Could not find dir {} via path {}", dirname, path);
            }
        })
    }
}
