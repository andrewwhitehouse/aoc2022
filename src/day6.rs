use std::collections::HashSet;

pub fn first_marker_position(buffer: String) -> usize {
    let chars: Vec<char> = buffer.as_bytes().iter().map(|b| *b as char).collect();
    for index in 5..chars.len() {
        let mut slice = HashSet::<char>::new();
        slice.insert(chars[index-1]);
        slice.insert(chars[index-2]);
        slice.insert(chars[index-3]);
        slice.insert(chars[index-4]);
        if slice.len() == 4 {
            return index;
        }
    }
    0
}

pub fn solve_part1(input: String) -> u32 {
    let buffer = parse(input);
    first_marker_position(buffer) as u32
}

pub fn parse(input: String) -> String {
    input.trim_end().to_string()
}

#[cfg(test)]
mod day6_tests {
    use super::*;

    #[test]
    fn test_first_marker1() {
        let input = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(first_marker_position(input), 5);
    }

    #[test]
    fn test_first_marker2() {
        let input = String::from("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(first_marker_position(input), 6);
    }

    #[test]
    fn test_first_marker3() {
        let input = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(first_marker_position(input), 10);
    }

    #[test]
    fn test_first_marker4() {
        let input = String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(first_marker_position(input), 11);
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse("abc\n"), "abc");
    }
}

