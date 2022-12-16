#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

#[derive(PartialEq, Debug)]
pub struct Movement {
    direction: Direction,
    distance: u32
}

pub struct Position {
    x: u32,
    y: u32
}

pub struct Positions {
    head: Position,
    tail: Position
}

pub fn parse(input: String) -> Vec<Movement> {
    Vec::new()
}

#[cfg(test)]
mod day9_tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = String::from("U 1\nR 2\nL 7\n D 8\nU 4");
        let expected = vec!(
            Movement{distance: 1, direction: Direction::UP},
            Movement{distance: 2, direction: Direction::RIGHT},
            Movement{distance: 7, direction: Direction::LEFT},
            Movement{distance: 8, direction: Direction::DOWN},
            Movement{distance: 4, direction: Direction::UP});
        assert_eq!(parse(input), expected);
    }
}

