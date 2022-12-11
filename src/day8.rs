fn string_chars(s: &str) -> Vec<char> {
    s.as_bytes().iter().map(|b| *b as char).collect()
}

pub fn parse(input: String) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    for line in input.trim_end().split("\n") {
        result.push(string_chars(line))
    }
    result
}

pub fn visible(heights: Vec<Vec<char>>) -> u32 {
    let edge_count = heights[0].len()*2 + 2*(heights.len()-2);
    let mut row_visible = 0u32;
    for row_index in 1..(heights.len()-1) {
        for column_index in 1..heights[row_index].len() {
            if heights[row_index][column_index] > heights[row_index][column_index-1] {
                row_visible += 1;
            } else {
                break;
            }
        }
    }
    for row_index in 1..(heights.len()-1) {
        for column_index in (1..heights[row_index].len()).rev() {
            if heights[row_index][column_index-1] > heights[row_index][column_index] {
                row_visible += 1;
            } else {
                break;
            }
        }
    }
    let mut columns_visible = 0u32;
    for column_index in 1..(heights.len()-1) {
        for row_index in 1..heights[column_index].len() {
            if heights[row_index][column_index] > heights[row_index-1][column_index] {
                columns_visible += 1;
            } else {
                break;
            }
        }
    }
    for column_index in 1..(heights.len()-1) {
        for row_index in (1..heights[column_index].len()).rev() {
            if heights[row_index-1][column_index] > heights[row_index][column_index] {
                columns_visible += 1;
            } else {
                break;
            }
        }
    }
    edge_count as u32 + row_visible + columns_visible
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
        assert_eq!(visible(heights), 21);
    }

    #[test]
    fn test_parse() {
        let expected = vec!(
            vec!('3', '0', '3'),
            vec!('2', '5', '5'));
        assert_eq!(parse(String::from("303\n255\n")), expected);
    }
}

