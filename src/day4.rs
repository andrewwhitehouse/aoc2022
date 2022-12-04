#[derive(PartialEq, Debug)]
pub struct Range {
    from: u32,
    to: u32
}

pub fn parse(input: String) -> Vec<Range> {
    let mut ranges = Vec::new();
    for line in input.trim_end().split("\n") {
        for range_str in line.split(",") {
            let bounds = range_str.split("-").collect::<Vec<&str>>();
            ranges.push(Range{from: bounds[0].parse::<u32>().unwrap(),
                to: bounds[1].parse::<u32>().unwrap()});
        }
    }
    ranges
}

#[cfg(test)]
mod day4_tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let input = String::from("2-4,6-8\n2-3,4-5\n");
        let expected = vec!(
            Range{from:2, to:4},
            Range{from:6, to:8},
            Range{from:2, to:3},
            Range{from:4, to:5});
        assert_eq!(parse(input), expected);
    }
}