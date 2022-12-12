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

pub fn is_visible_from_edge(heights: &Vec<Vec<char>>, row_index: usize, col_index: usize) -> bool {
    let height = heights[row_index][col_index];

    //println!("Index {},{} Height {}", row_index, col_index, height);

    let mut visible_from_left = true;
    for left_to_right_index in 0..col_index {
        if heights[row_index][left_to_right_index] >= height {
            visible_from_left = false;
            break;
        }
    }
    //println!("Visible from left {}", visible_from_left);
    if visible_from_left {
        return true;
    }

    let mut visible_from_right = true;
    for right_to_left_index in ((col_index+1)..heights[row_index].len()).rev() {
        if heights[row_index][right_to_left_index] >= height {
            visible_from_right = false;
            break;
        }
    }
    //println!("Visible from right {}", visible_from_right);
    if visible_from_right {
        return true;
    }

    let mut visible_from_top = true;
    for top_to_bottom_index in 0..row_index {
        if heights[top_to_bottom_index][col_index] >= height {
            visible_from_top = false;
            break;
        }
    }
    //println!("Visible from top {}", visible_from_top);
    if visible_from_top {
        return true;
    }

    let mut visible_from_bottom = true;
    for bottom_to_top_index in (row_index+1)..heights.len() {
        if heights[bottom_to_top_index][col_index] >= height {
            visible_from_bottom = false;
            break;
        }
    }
    //println!("Visible from bottom {}", visible_from_bottom);
    return visible_from_bottom
}

pub fn count_visible(heights: Vec<Vec<char>>) -> u32 {
    let mut visible = 0u32;
    for row_index in 0..heights.len() {
        for col_index in 0..heights[row_index].len() {
            if row_index == 0 || col_index == 0 {
                visible += 1;
            } else if is_visible_from_edge(&heights, row_index, col_index){
                visible += 1;
            }
        }
    }
    visible
}

pub fn solve_part1(input: String) -> u32 {
    let heights = parse(input);
    count_visible(heights)
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
        assert_eq!(count_visible(heights), 21);
    }

    #[test]
    fn test_parse() {
        let expected = vec!(
            vec!('3', '0', '3'),
            vec!('2', '5', '5'));
        assert_eq!(parse(String::from("303\n255\n")), expected);
    }

    #[test]
    fn test_part1() {
        let input = String::from("30373\n25512\n65332\n33549\n35390\n");
        assert_eq!(solve_part1(input), 21);
    }
}

