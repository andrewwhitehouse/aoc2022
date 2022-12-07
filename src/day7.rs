#[derive(PartialEq, Debug)]
pub struct File {
    name: String,
    size: u32,
}

#[derive(PartialEq, Debug)]
pub struct Directory {
    name: String,
    files: Vec<File>,
    sub_directories: Vec<Directory>,
}

pub fn parse(input: String) -> Directory {
    Directory {
        name: String::from("/"),
        files: vec![],
        sub_directories: vec![],
    }
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
        let a = Directory {
            name: "a".to_string(),
            sub_directories: vec![Directory {
                name: "e".to_string(),
                files: vec![File {
                    name: "i".to_string(),
                    size: 584,
                }],
                sub_directories: vec![],
            }],
            files: vec![
                File {
                    name: "f".to_string(),
                    size: 29116,
                },
                File {
                    name: "g".to_string(),
                    size: 2557,
                },
                File {
                    name: "h.lst".to_string(),
                    size: 62596,
                },
            ],
        };
        let d = Directory {
            name: "d".to_string(),
            sub_directories: vec![],
            files: vec![
                File {
                    name: "j".to_string(),
                    size: 4060174,
                },
                File {
                    name: "d.log".to_string(),
                    size: 8033020,
                },
                File {
                    name: "d.ext".to_string(),
                    size: 5626152,
                },
                File {
                    name: "k".to_string(),
                    size: 7214296,
                },
            ],
        };
        Directory {
            name: "/".to_string(),
            sub_directories: vec![a, d],
            files: vec![
                File {
                    name: "b.txt".to_string(),
                    size: 14848514,
                },
                File {
                    name: "c.txt".to_string(),
                    size: 8504156,
                },
            ],
        }
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse(String::from(example_input())), example_directory());
    }
}
