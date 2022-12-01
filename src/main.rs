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

pub fn maximum_elf_calories(batches: Vec<Vec<u32>>) -> u32 {
    let mut elf_max = 0u32;
    for batch in batches.iter() {
        let total = batch.iter().sum();
        if total > elf_max {
            elf_max = total;
        }
    }
    elf_max
}

pub fn top_three_sum(batches: Vec<Vec<u32>>) -> u32 {
    let mut totals = batches.iter().map(|batch| batch.iter().sum()).collect::<Vec<u32>>();
    totals.sort_by(|a, b| b.cmp(a));
    totals[0] + totals[1] + totals[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses() {
        let input = format!("{}{}{}{}{}",
                    "1000\n2000\n3000\n\n",
                    "4000\n\n",
                    "5000\n6000\n\n",
                    "7000\n8000\n9000\n\n",
                    "10000\n");
        let expected = vec!(
            vec!(1000, 2000, 3000),
            vec!(4000),
            vec!(5000, 6000),
            vec!(7000, 8000, 9000),
            vec!(10000));
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn finds_maximum_elf_calories() {
        let calories = vec!(
            vec!(1000, 2000, 3000),
            vec!(4000),
            vec!(5000, 6000),
            vec!(7000, 8000, 9000),
            vec!(10000));
        assert_eq!(maximum_elf_calories(calories), 24_000);
    }

    #[test]
    fn finds_top_three_sum() {
        let calories = vec!(
            vec!(23, 55),
            vec!(14),
            vec!(101),
            vec!(30));
        assert_eq!(top_three_sum(calories), 30+23+55+101);
    }
}

fn main() {
    let input = fs::read_to_string("day1.txt")
        .expect("failed to read day1 input");
    let batches = parse(input);
    println!("Day 1 Part 1 {}", maximum_elf_calories(batches.clone()));
    println!("Day 1 Part 2 {}", top_three_sum(batches));
}
