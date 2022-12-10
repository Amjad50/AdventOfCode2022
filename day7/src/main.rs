use std::{collections::HashMap, env, fs, io, iter::repeat};

const TOTAL_FILESYSTEM: usize = 70000000;
const REQUIRED_FREE: usize = 30000000;

fn main() -> io::Result<()> {
    let file_data = fs::read_to_string(env::args().nth(1).expect("Input file arg"))?;

    let mut all_folders = HashMap::<String, usize>::new();
    all_folders.insert("/".to_string(), 0);

    let mut folder_stack = vec![];
    let mut current_folder = folder_stack.join("/");

    // will append multiple "cd .." until root
    for line in file_data.lines().chain(repeat("$ cd ..")) {
        if line.starts_with("$ ls") {
            // do nothing
        } else if line.starts_with("$ cd") {
            let dir_name = line.split_at(5).1.trim();
            if dir_name == ".." {
                let previous_folder = current_folder.clone();
                folder_stack.pop();
                if folder_stack.is_empty() {
                    break;
                }
                current_folder = folder_stack.join("/");

                let previous_folder_size = *all_folders.get(&previous_folder).unwrap();
                *all_folders.get_mut(&current_folder).unwrap() += previous_folder_size;
            } else {
                folder_stack.push(dir_name.to_string());
                current_folder = folder_stack.join("/");
            }
        } else {
            if line.starts_with("dir") {
                let dir_name = line.split_at(4).1.trim().to_string();
                all_folders.insert(format!("{current_folder}/{dir_name}"), 0);
            } else {
                let mut words = line.split_whitespace();
                let file_size: usize = words.next().unwrap().parse().unwrap();
                *all_folders.get_mut(&current_folder).unwrap() += file_size;
            }
        }
    }

    //println!("{:?}", all_folders);
    //println!("{}", current_folder);

    let current_used = *all_folders.get(&current_folder).unwrap();
    let needed_to_free = REQUIRED_FREE - (TOTAL_FILESYSTEM - current_used);
    //println!("{} bytes needed to free", needed_to_free);

    let mut p1_size_sum = 0;
    let mut p2_single_to_free = usize::max_value();
    for (_folder, size) in all_folders {
        if size <= 100000 {
            p1_size_sum += size;
        }
        if size >= needed_to_free {
            p2_single_to_free = p2_single_to_free.min(size);
        }
    }
    println!("Part 1: {}", p1_size_sum);
    println!("Part 2: {}", p2_single_to_free);

    Ok(())
}
