use std::collections::HashSet;

pub fn distinct_chars_marker(buffer: String, buffer_len: usize) -> usize {
    let chars: Vec<char> = buffer.as_bytes().iter().map(|b| *b as char).collect();
    for index in (buffer_len+1)..chars.len() {
        let mut slice = HashSet::<char>::new();
        for i in 1..=buffer_len {
            slice.insert(chars[index-i]);
        }
        if slice.len() == buffer_len {
            return index;
        }
    }
    0
}

pub fn start_of_packet_marker(buffer: String) -> usize {
    distinct_chars_marker(buffer, 4)
}

pub fn start_of_message_marker(buffer: String) -> usize {
    distinct_chars_marker(buffer, 14)
}

pub fn solve_part1(input: String) -> u32 {
    let buffer = parse(input);
    start_of_packet_marker(buffer) as u32
}

pub fn solve_part2(input: String) -> u32 {
    let buffer = parse(input);
    start_of_message_marker(buffer) as u32
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
        assert_eq!(start_of_packet_marker(input), 5);
    }

    #[test]
    fn test_first_marker2() {
        let input = String::from("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(start_of_packet_marker(input), 6);
    }

    #[test]
    fn test_first_marker3() {
        let input = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(start_of_packet_marker(input), 10);
    }

    #[test]
    fn test_first_marker4() {
        let input = String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(start_of_packet_marker(input), 11);
    }

    #[test]
    fn test_message_marker1() {
        let input = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(start_of_message_marker(input), 29);
    }


    #[test]
    fn test_parse() {
        assert_eq!(parse(String::from("abc\n")), "abc");
    }
}

