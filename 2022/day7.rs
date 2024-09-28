#![allow(dead_code, unused_variables)]

use std::fs;
use std::collections::HashMap;

struct File {
    name: String,
    filesize: i32,
}

struct Dir {
    name: String,
    files: HashMap<String, File>,
    dirs: HashMap<String, Dir>,
}

impl Dir {
    fn new(name: String) -> Self {
        Self {
            name,
            files: HashMap::new(),
            dirs: HashMap::new(),
        }
    }
}

fn get_top_dir<'a>(current_stack: &Vec<String>, root: &'a mut Dir) -> &'a mut Dir {
    let mut c = root;
    for c_st_dir in current_stack {
        if c_st_dir == "/" {
            continue;
        }
        c = c.dirs.get_mut(c_st_dir).unwrap();
    }
    c
}

fn count_valid_dir_size(total: &mut i32, root: &Dir) -> i32 {
    let mut count: i32 = 0;
    for (_, v) in &root.files {
        count += v.filesize;
    }
    for (_, v) in &root.dirs {
        count += count_valid_dir_size(total, &v);
    }
    if count <= 100000 {
        *total += count;
    }
    return count;
}

fn find_dir_to_remove_size(dirs: &mut Vec<i32>, root: &Dir) -> i32 {
    let mut count: i32 = 0;
    for (_, v) in &root.files {
        count += v.filesize;
    }
    for (_, v) in &root.dirs {
        count += find_dir_to_remove_size(dirs, &v);
    }
    dirs.push(count);
    return count;
}

fn main() {
    let contents = fs::read_to_string("input").expect("File 'input' not found!");
    let mut root = Dir::new("/".to_string());
    let mut current_stack = Vec::new();
    current_stack.push("/".to_string());
    for line in contents.lines() {
        let text: Vec<&str> = line.split_whitespace().collect();
        if text[0] == "$" {
            match text[1] {
                "cd" => {
                    if text[2] == "/" {
                        current_stack.clear();
                        current_stack.push("/".to_string());
                    } else if text[2] == ".." {
                        current_stack.remove(current_stack.len()-1);
                    } else {
                        current_stack.push(
                            get_top_dir(&current_stack, &mut root).dirs.get(&text[2].to_string()).unwrap().name.clone()
                        );
                    }
                },
                _ => {}
            }
        } else if text[0] == "dir" {
            if !get_top_dir(&current_stack, &mut root).dirs.contains_key(text[1]) {
                let current_dir = get_top_dir(&current_stack, &mut root);
                current_dir.dirs.insert(text[1].to_string(), Dir::new(text[1].to_string()));
            }
        } else {
            if !get_top_dir(&current_stack, &mut root).files.contains_key(text[1]) {
                let current_dir = get_top_dir(&current_stack, &mut root);
                let filesize = text[0].parse::<i32>().unwrap();
                current_dir.files
                    .insert(text[1].to_string(), File { name: text[1].to_string(), filesize });
            }
        }
    }
    let mut part1 = 0;
    count_valid_dir_size(&mut part1, &root);
    println!("Part 1: {}", part1);

    let mut part2 = -1;
    let mut dirs = Vec::<i32>::new();
    let total = 70000000 - find_dir_to_remove_size(&mut dirs, &root);
    for dir in dirs {
        if total + dir >= 30000000 {
            if part2 > dir || part2 == -1 {
                part2 = dir;
            }
        }
    }
    print!("Part 2: {}", part2);
}