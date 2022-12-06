pub fn first_marker_position(buffer: &str) -> u32 {
    0
}

#[cfg(test)]
mod day6_tests {
    use super::*;

    #[test]
    fn test_first_marker1() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(first_marker_position(input), 5);
    }
}

