use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub struct File {
    name: String,
    size: u32,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Directory {
    name: String,
    files: HashMap<String, u32>,
    sub_directories: HashMap<String, Directory>,
    parent: Vec<Directory> // I can't think of another way to do this yet
}

pub fn parse(input: String) -> Directory {
    let locaion = "/"; // Assume we start from /
    let root = Directory {
        name: String::from("/"),
        files: HashMap::new(),
        sub_directories: HashMap::new(),
        parent: Vec::new()
    };
    let mut current = root;
    for line in input.trim().split("\n") {
        if line.starts_with('$') {
            let parts: Vec<&str> = line.split(" ").collect();
            if parts[1] == "cd" {
                if parts[2] == ".." {
                    current = current.parent[0];
                } else {
                    let target = current.sub_directories.get(parts[2]).unwrap();
                    current = *target;
                }
            }
        } else {
            let mut directories = Vec::new();
            let mut files = Vec::new();
            let parts: Vec<&str> = line.split(" ").collect();
            if parts[0] == "dir" {
                current.sub_directories.insert(parts[1].to_string(), Directory { name: parts[1].to_string(),
                    files: HashMap::new(),
                    sub_directories: HashMap::new(),
                    parent: vec!(current.clone())});
            } else {
                let size = parts[0].parse::<u32>().unwrap;
                current.files.insert(parts[1].to_string(), size);
            }
        }
    }
    root
}

#[cfg(test)]
mod day7_tests {
    use super::*;

    fn example_input() -> String {
        String::from(
            "
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
",
        )
    }

    fn example_directory() -> Directory {
        let root = Directory {
            name: "/".to_string(),
            sub_directories: HashMap::new(),
            files: HashMap::from([
                ("b.txt".to_string(), 14848514),
                ("c.txt".to_string(), 8504156)
            ]),
            parent: Vec::new()
        };
        let mut a = Directory {
            name: "a".to_string(),
            files: HashMap::from([
                ("f".to_string(), 29116),
                ("g".to_string(), 2557),
                ("h.lst".to_string(), 62596)
            ]),
            sub_directories: HashMap::from([
                ("a".to_string(), Directory {
                    name: "e".to_string(),
                    files: HashMap::from([("i".to_string(), 584)]),
                    sub_directories: HashMap::new(),
                    parent: todo!()
            })]),
            parent: vec!(root),
        };
        a.sub_directories["a".to_string()].parent = a;
        let d = Directory {
            name: "d".to_string(),
            sub_directories: HashMap::new(),
            files: HashMap::from([
                ("j".to_string(), 4060174),
                ("d.log".to_string(), 8033020),
                ("d.ext".to_string(), 5626152),
                ("k".to_string(), 7214296)
            ]),
            parent: vec!(root)
        };
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse(String::from(example_input())), example_directory());
    }
}
