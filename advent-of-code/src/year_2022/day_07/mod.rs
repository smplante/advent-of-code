use std::collections::HashMap;

mod inputs;

pub fn run() {
    let start = std::time::SystemTime::now();
    println!("--------------------------");
    println!("Advent of Code 2022 Day 07");
    println!("--------------------------\n");

    let result_test_first = process_input_first(inputs::TEST);
    let result_test_second = process_input_second(inputs::TEST);
    println!(
        "test:   first half: {:?}, second half: {:?}",
        result_test_first, result_test_second
    );

    let result_actual_first = process_input_first(inputs::ACTUAL);
    let result_actual_second = process_input_second(inputs::ACTUAL);
    println!(
        "actual: first half: {:?}, second half: {:?}\n",
        result_actual_first, result_actual_second
    );
    println!("Day 07 completed in: {:?}\n", start.elapsed().unwrap());
}

#[derive(Debug)]
enum FS {
    File {
        name: String,
        size: u32,
    },
    Dir {
        name: String,
        contents: HashMap<String, FS>,
    },
}

impl FS {
    fn dir(name: &str) -> FS {
        FS::Dir {
            name: name.to_string(),
            contents: HashMap::<String, FS>::new(),
        }
    }

    fn file(name: &str, size: u32) -> FS {
        FS::File {
            name: name.to_string(),
            size,
        }
    }

    fn name(&self) -> String {
        match self {
            FS::File { name, size: _ } => name.to_string(),
            FS::Dir { name, contents: _ } => name.to_string(),
        }
    }

    fn size(&self) -> u32 {
        match self {
            FS::File { name: _, size } => *size,
            FS::Dir { name: _, contents } => contents.values().map(|v| v.size()).sum(),
        }
    }

    fn push(&mut self, fs: FS) {
        if let FS::Dir { name: _, contents } = self {
            contents.insert(fs.name(), fs);
        }
    }

    fn get(&mut self, name: &str) -> Option<&mut FS> {
        match self {
            FS::Dir { name: _, contents } => contents.get_mut(&name.to_string()),
            _ => None,
        }
    }
}

fn process_input_first(input: &str) -> u32 {
    let mut cur_path: Vec<&str> = Vec::new();
    let mut root_dir = FS::dir("/");
    let mut cur_dir = &mut root_dir;

    let mut size_sum = 0;

    for line in input.split('\n') {
        let mut split_line = line.split_whitespace();
        if line.starts_with("$ ls") {
            continue;
        }
        if line.starts_with("$ cd ") {
            if line.ends_with('/') {
                continue;
            }
            if line.ends_with("..") {
                let size = cur_dir.size();
                if size <= 100000 {
                    size_sum += size;
                }
                cur_path.pop();
                cur_dir = &mut root_dir;
                for &d in &cur_path {
                    cur_dir = cur_dir.get(d).unwrap();
                }
                continue;
            }
            let d = split_line.nth(2).unwrap();
            cur_dir.push(FS::dir(d));
            cur_dir = cur_dir.get(d).unwrap();
            cur_path.push(d);
            continue;
        }
        if line.starts_with("dir") {
            cur_dir.push(FS::dir(split_line.nth(1).unwrap()));
            continue;
        }

        let size = split_line.next().unwrap().parse::<u32>().unwrap();
        let name = split_line.next().unwrap();
        cur_dir.push(FS::file(name, size));
    }

    let size = cur_dir.size();
    if size <= 100000 {
        size_sum += size;
    }

    size_sum
}

fn process_input_second(input: &str) -> u32 {
    let mut cur_path: Vec<&str> = Vec::new();
    let mut root_dir = FS::dir("/");
    let mut cur_dir = &mut root_dir;

    let mut dir_sizes = HashMap::new();

    for line in input.split('\n') {
        let mut split_line = line.split_whitespace();
        if line.starts_with("$ ls") {
            continue;
        }
        if line.starts_with("$ cd ") {
            if line.ends_with('/') {
                continue;
            }
            if line.ends_with("..") {
                let size = cur_dir.size();
                dir_sizes.insert(cur_path.last().unwrap().to_owned(), size);
                cur_path.pop();
                cur_dir = &mut root_dir;
                for &d in &cur_path {
                    cur_dir = cur_dir.get(d).unwrap();
                }
                continue;
            }
            let d = split_line.nth(2).unwrap();
            cur_dir.push(FS::dir(d));
            cur_dir = cur_dir.get(d).unwrap();
            cur_path.push(d);
            continue;
        }
        if line.starts_with("dir") {
            cur_dir.push(FS::dir(split_line.nth(1).unwrap()));
            continue;
        }

        let size = split_line.next().unwrap().parse::<u32>().unwrap();
        let name = split_line.next().unwrap();
        cur_dir.push(FS::file(name, size));
    }

    let size = cur_dir.size();
    dir_sizes.insert(cur_path.last().unwrap().to_owned(), size);

    let filesystem_max = 70000000;
    let filesystem_need = 30000000;
    let filesystem_current = root_dir.size();

    let dir_to_remove = dir_sizes
        .into_iter()
        .fold(("/", filesystem_current), |accum, dir| {
            if dir.1 >= filesystem_current - (filesystem_max - filesystem_need) && dir.1 < accum.1 {
                return dir;
            }
            accum
        });

    dir_to_remove.1
}

// #[cfg(test)]
// mod tests {
//     extern crate test;
//
//     use super::*;
//     use test::Bencher;
//
//     #[bench]
//     fn bench_process_input_first(b: &mut Bencher) {
//         b.iter(|| process_input_first(inputs::ACTUAL));
//     }
//
//     #[bench]
//     fn bench_process_input_second(b: &mut Bencher) {
//         b.iter(|| process_input_second(inputs::ACTUAL));
//     }
// }
