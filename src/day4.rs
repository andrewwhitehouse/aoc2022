#[derive(PartialEq, Debug)]
pub struct Range {
    from: u32,
    to: u32,
}

pub fn parse(input: String) -> Vec<Range> {
    let mut ranges = Vec::new();
    for line in input.trim_end().split("\n") {
        for range_str in line.split(",") {
            let bounds = range_str.split("-").collect::<Vec<&str>>();
            ranges.push(Range {
                from: bounds[0].parse::<u32>().unwrap(),
                to: bounds[1].parse::<u32>().unwrap(),
            });
        }
    }
    ranges
}

pub fn fully_contained(ranges: Vec<Range>) -> u32 {
    let mut count = 0u32;
    for i in (0..ranges.len()).step_by(2) {
        let left = &ranges[i];
        let right = &ranges[i + 1];
        if (left.from <= right.from && left.to >= right.to) // right fully contains left
            || (right.from <= left.from && right.to >= left.to)
        {
            // left fully contains right
            count = count + 1;
        }
    }
    count
}

pub fn solve_part1(input: String) -> u32 {
    let ranges = parse(input);
    fully_contained(ranges)
}

#[cfg(test)]
mod day4_tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let input = String::from("2-4,6-8\n2-3,4-5\n");
        let expected = vec![
            Range { from: 2, to: 4 },
            Range { from: 6, to: 8 },
            Range { from: 2, to: 3 },
            Range { from: 4, to: 5 },
        ];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_fully_contained() {
        let ranges = vec![
            Range { from: 2, to: 4 },
            Range { from: 6, to: 8 },
            Range { from: 2, to: 3 },
            Range { from: 4, to: 5 },
            Range { from: 5, to: 7 },
            Range { from: 7, to: 9 },
            Range { from: 2, to: 8 },
            Range { from: 3, to: 7 },
            Range { from: 6, to: 6 },
            Range { from: 4, to: 6 },
            Range { from: 2, to: 6 },
            Range { from: 4, to: 8 },
        ];
        assert_eq!(fully_contained(ranges), 2);
    }
}
