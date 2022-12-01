pub fn parse(input: String) -> Vec<Vec<u32>> {
    let mut batches = Vec::new();
    let mut current: Vec<u32> = Vec::new();
    for line in input.split("\n") {
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
}

fn main() {
    println!("Hello, world!");
}
