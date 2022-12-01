pub fn parse(input: String) -> Vec<Vec<u16>> {
    vec!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let expected = vec!(
            vec!(1000, 2000, 3000),
            vec!(4000),
            vec!(5000, 6000),
            vec!(7000, 8000, 9000),
            vec!(10000));
        assert_eq!(parse(String::from(input)), expected);
    }
}

fn main() {
    println!("Hello, world!");
}
