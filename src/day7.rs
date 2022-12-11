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
    sub_directories: HashMap<String, Directory>
}

pub fn parse(input: String) -> Directory {
    let mut parent: HashMap<String, Directory> = HashMap::new();
    let root = Directory {
        name: String::from("/"),
        files: HashMap::new(),
        sub_directories: HashMap::new()
    };
    let mut current = &root;
    for line in input.trim().split("\n") {
        if line.starts_with('$') {
            let parts: Vec<&str> = line.split(" ").collect();
            if parts[1] == "cd" {
                if parts[2] == ".." {
                    // do nothing yet
                } else {
                    let target = current.sub_directories.get(parts[2]).unwrap();
                    current = &(*target).clone();
                }
            }
        } else {
            let parts: Vec<&str> = line.split(" ").collect();
            if parts[0] == "dir" {
                current.sub_directories.insert(parts[1].to_string(), Directory { name: parts[1].to_string(),
                    files: HashMap::new(),
                    sub_directories: HashMap::new()});
            } else {
                let size = parts[0].parse::<u32>().unwrap();
                current.files.insert(parts[1].to_string(), size);
            }
        }
    }
    root.clone()
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
",
        )
    }

    fn example_directory() -> Directory {
        let a = Directory {
            name: "a".to_string(),
            files: HashMap::new(),
            sub_directories: HashMap::new()
        };
        let mut root = Directory {
            name: "/".to_string(),
            sub_directories: HashMap::new(),
            files: HashMap::from([
                ("b.txt".to_string(), 14848514),
                ("c.txt".to_string(), 8504156)
            ])
        };
        root.sub_directories.insert("a".to_string(), a);
        root
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse(String::from(example_input())), example_directory());
    }
}
