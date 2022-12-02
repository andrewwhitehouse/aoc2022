use std::fs;

pub fn parse(input: String) -> Vec<Vec<u32>> {
    let mut batches = Vec::new();
    let mut current: Vec<u32> = Vec::new();
    for line in input.trim_end().split("\n") {
        if line.trim().len() == 0 {
            if current.len() > 0 {
                batches.push(current);
                current = Vec::new();
            }
        } else {
            current.push(line.parse::<u32>().unwrap());
        }
    }
    if current.len() > 0 {
        batches.push(current);
    }
    batches
}

pub fn max_sum(batches: Vec<Vec<u32>>) -> u32 {
    let mut max = 0u32;
    for batch in batches {
        let batch_sum = batch.iter().sum();
        if batch_sum > max {
            max = batch_sum;
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses() {
        let input = format!(
            "{}{}{}{}{}",
            "1000\n2000\n3000\n\n", "4000\n\n", "5000\n6000\n\n", "7000\n8000\n9000\n\n", "10000\n"
        );
        let expected = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn find_max_sum() {
        let batches = vec!(
            vec!(1000, 2000, 3000),
            vec!(4000),
            vec!(5000, 6000),
            vec!(7000, 8000, 9000),
            vec!(10000));
        assert_eq!(max_sum(batches), 24_000);
    }
}

fn main() {
    let input = fs::read_to_string("input/day1.txt").expect("failed to read day1 input");
    let batches = parse(input);
    println!("Day 1 Part 1 {}", max_sum(batches));
}
