pub fn visible(heights: Vec<Vec<char>>) -> usize {
    0
}

#[cfg(test)]
mod day8_tests {
    use super::*;

    #[test]
    fn test_find_visible() {
        let heights = vec!(
            vec!('3', '0', '3', '7', '3'),
            vec!('2', '5', '5', '1', '2'),
            vec!('6', '5', '3', '3', '2'),
            vec!('3', '3', '5', '4', '9'),
            vec!('3', '5', '3', '9', '0'));
        assert_eq!(visible(heights), 16);
    }
}

