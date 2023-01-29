use std::{cmp, collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./src/data").unwrap();
    let data: Vec<&str> = input.split("\n$ ").collect();
    println!("data: {:#?}", data);
    let mut current_dir = Vec::new();
    current_dir.push("");
    let mut file_tree = HashMap::new();
    let mut dir_tree = Vec::new();
    dir_tree.push("/".into());
    let mut dir_tree_size = HashMap::new();

    for chunk in data.iter() {
        let mut c = chunk.lines();
        let cmd = c.next().unwrap();
        // println!("command: {:#?} res: {}", cmd, response);
        // println!("{}", cmd);
        if cmd.starts_with("cd") {
            let arg = cmd.split_whitespace().nth(1).unwrap();
            if arg == ".." {
                current_dir.pop();
            } else {
                current_dir.push(arg);
            }
            // println!("/{}", current_dir.join("/"))
        } else if cmd.starts_with("ls") {
            // println!("{}", c.().unwrap());
            for l in c {
                // println!("{}", l);
                let size = l.split(" ").next().unwrap();
                let name = l.split(" ").nth(1).unwrap();
                let path = format!("{}/{}", current_dir.join("/"), name);

                if size == "dir" {
                    dir_tree.push(path);
                } else {
                    let size = size.parse::<usize>().unwrap();
                    file_tree.insert(path, size);
                }
                // file_tree.insert(path, size);
                // println!("{} {}", path, size)
            }
        }
    }

    // let mut all_sum = 0;
    // println!("{:#?}", dir_tree);
    // println!("{:#?}", file_tree);

    for dir in dir_tree {
        // let all_file = file_tree.fil
        let mut sum = 0;
        for (path, size) in file_tree.iter() {
            if path.starts_with(&dir) {
                sum = sum + size;
            }
        }
        dir_tree_size.insert(dir, sum);
        // if sum < 100000 {
        //     all_sum = all_sum + sum;
        // }
    }
    println!("{:#?}", dir_tree_size);

    let used_space = dir_tree_size.get("/".into()).unwrap();
    let freed_space = 70000000 - used_space;
    let need_to_delete = 30000000 - freed_space;

    let mut target_size = 300000000;

    // println!("{:?} , {:?}", need_to_delete, freed_space);
    // let mut least_diff = 100000000;
    for size in dir_tree_size.values() {
        // let diff = size - need_to_delete;
        if size > &need_to_delete {
            target_size = cmp::min(*size, target_size);
            // if size < &target_size {
            //     target_size = *size
            // }
            // least_diff = diff;
            println!("{:?}", size);
        }
    }
    println!("{:?}", target_size);
}
