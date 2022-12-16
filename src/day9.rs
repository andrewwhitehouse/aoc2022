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

#[derive(PartialEq, Debug)]
pub struct Position {
    x: u32,
    y: u32
}

#[derive(PartialEq, Debug)]
pub struct Positions {
    head: Position,
    tail: Position
}

pub fn parse(input: String) -> Vec<Movement> {
    let mut result = Vec::new();
    for line in input.trim_end().split("\n") {
        let movement: Vec<&str> = line.trim().split(" ").collect();
        println!("{:?}", movement);
        let direction = match movement[0] {
            "R" => Direction::RIGHT,
            "L" => Direction::LEFT,
            "U" => Direction::UP,
            "D" => Direction::DOWN,
            &_ => todo!()
        };
        result.push(Movement{direction: direction, distance: movement[1].parse::<u32>().unwrap()});
    }
    result
}

pub fn navigate(start: Positions, movements: Vec<Movement>) -> Positions {
    Positions{head: Position{x: 0, y: 0}, tail: Position{x: 0, y: 0}}
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

    fn starting_positions() -> Positions {
        Positions{head: Position{x: 0, y: 0}, tail: Position{x: 0, y: 0}}
    }

    #[test]
    fn test_navigate_right() {
        let expected = Positions{head: Position{x: 4, y: 0}, tail: Position{x: 3, y: 0}};
        assert_eq!(navigate(starting_positions(), vec!(Movement{direction: Direction::RIGHT, distance: 4})), expected);
    }
}

