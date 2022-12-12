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

pub fn scenic_score(heights: &Vec<Vec<char>>, row_index: usize, col_index: usize) -> u32 {
    let height = heights[row_index][col_index];

    //println!("Index {},{} Height {}", row_index, col_index, height);

    let mut left_distance = 0u32;
    for left_index in (0..col_index).rev() {
        left_distance += 1;
        //println!("left row {} column {}", row_index, col_index);
        if heights[row_index][left_index] >= height {
            break;
        }
    }

    let mut right_distance = 0u32;
    for right_index in (col_index+1)..heights[row_index].len() {
        right_distance += 1;
        if heights[row_index][right_index] >= height {
            break;
        }
    }

    let mut up_distance = 0u32;
    for up_index in (0..row_index).rev() {
        up_distance += 1;
        if heights[up_index][col_index] >= height {
            break;
        }
    }

    let mut down_distance = 0u32;
    for down_index in (row_index+1)..heights.len() {
        down_distance += 1;
        if heights[down_index][col_index] >= height {
            break;
        }
    }

    //println!("up {} down {} left {} right {}", up_distance, down_distance, left_distance, right_distance);
    return left_distance * right_distance * up_distance * down_distance;
}

pub fn highest_scenic_score(heights: &Vec<Vec<char>>) -> u32 {
    let mut highest_score = 0u32;
    for row_index in 0..heights.len() {
        for col_index in 0..heights[row_index].len() {
            let score = scenic_score(heights, row_index, col_index);
            //println!("Score {}", score);
            if score > highest_score {
                highest_score = score;
            }
        }
    }
    highest_score
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

pub fn solve_part2(input: String) -> u32 {
    let heights = parse(input);
    highest_scenic_score(&heights)
}

#[cfg(test)]
mod day8_tests {
    use super::*;

    fn example_heights() -> Vec<Vec<char>> {
        vec!(
            vec!('3', '0', '3', '7', '3'),
            vec!('2', '5', '5', '1', '2'),
            vec!('6', '5', '3', '3', '2'),
            vec!('3', '3', '5', '4', '9'),
            vec!('3', '5', '3', '9', '0'))
    }

    #[test]
    fn test_find_visible() {
        assert_eq!(count_visible(example_heights()), 21);
    }

    #[test]
    fn test_scenic_score() {
        assert_eq!(scenic_score(&example_heights(), 1, 2), 4);
    }

    #[test]
    fn test_scenic_score2() {
        assert_eq!(scenic_score(&example_heights(), 3, 2), 8);
    }

    #[test]
    fn test_highest_score() {
        assert_eq!(highest_scenic_score(&example_heights()), 8);
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

