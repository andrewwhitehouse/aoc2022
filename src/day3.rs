use std::collections::HashSet;

pub fn common_item(contents: String) -> char {
    let (left, right) = contents.split_at(contents.len() / 2);
    let mut s1 = HashSet::<char>::new();
    s1.extend(left.chars());
    let common_chars = right.chars().filter(|ch| s1.contains(ch)).collect::<HashSet<_>>().into_iter().collect::<Vec<char>>();
    if common_chars.len() != 1 {
        panic!("Expected one common char, found {}", common_chars.len());
    }
    common_chars[0]
}

pub fn priority(item: char) -> u32 {
    if item >= 'a' && item <= 'z' {
        (item as u32) - 97 + 1
    } else if item >= 'A' && item <= 'Z' {
        (item as u32) - 65 + 27
    } else {
        panic!("Not a valid item character {}", item);
    }
}

pub fn parse(input: String) -> Vec<String> {
    input.trim_end().split("\n").map(|s| String::from(s)).collect()
}

pub fn solve_part1(input: String) -> u32 {
    let items_by_rucksack = parse(input);
    let mut priority_sum = 0u32;
    for items in items_by_rucksack {
        priority_sum += priority(common_item(items));
    }
    priority_sum
}


#[cfg(test)]
mod day3_tests {
    use super::*;

    #[test]
    fn test_finds_common_item1() {
        let contents = String::from("vJrwpWtwJgWrhcsFMMfFFhFp");
        assert_eq!(common_item(contents), 'p');
    }

    #[test]
    fn test_finds_common_item2() {
        let contents = String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"); // actually appears twice in both sides
        assert_eq!(common_item(contents), 'L');
    }

    #[test]
    fn test_finds_common_item3() {
        let contents = String::from("PmmdzqPrVvPwwTWBwg");
        assert_eq!(common_item(contents), 'P');
    }

    #[test]
    fn test_priority_lower_case() {
        assert_eq!(priority('c'), 3);
    }

    #[test]
    fn test_priority_upper_case() {
        assert_eq!(priority('L'), 38);
    }

    #[test]
    #[should_panic]
    fn test_invalid_item() {
        priority('&');
    }

    #[test]
    fn test_parsing() {
        let input = String::from("abc\njkl\nxyz\n");
        let expected = vec!(String::from("abc"), String::from("jkl"), String::from("xyz"));
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_example() {
        let input = format!("{}\n{}\n{}\n{}\n{}\n{}\n",
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw");
        let result = solve_part1(input);
        assert_eq!(result, 157);
    }
}