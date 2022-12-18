#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(PartialEq, Debug)]
pub struct Movement {
    direction: Direction,
    distance: u32,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Position {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Positions {
    head: Position,
    tail: Position,
}

#[derive(PartialEq, Debug)]
pub struct Journey {
    start: Positions,
    end: Positions,
    visited: Vec<Positions>
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
            &_ => todo!(),
        };
        result.push(Movement {
            direction: direction,
            distance: movement[1].parse::<u32>().unwrap(),
        });
    }
    result
}

fn move_one_step(start: Positions, direction: Direction) -> Positions {
    let head_position = match direction {
        Direction::RIGHT => Position {
            x: start.head.x + 1,
            y: start.head.y,
        },
        Direction::UP => Position {
            x: start.head.x,
            y: start.head.y - 1,
        },
        Direction::LEFT => Position {
            x: start.head.x-1,
            y: start.head.y,
        },
        Direction::DOWN => Position {
            x: start.head.x,
            y: start.head.y+1,
        },
        _ => start.head,
    };

    let tail_position = &start.tail.clone();
    if head_position.y.abs_diff(tail_position.y) <= 1
        && head_position.x.abs_diff(tail_position.x) <= 1
    {
        Positions {
            head: head_position,
            tail: start.tail,
        }
    } else {
        let mut new_tail_position = start.tail;
        // Check if tail is two steps away
        if head_position.y == tail_position.y && head_position.x.abs_diff(tail_position.x) == 2
            || head_position.x == tail_position.x && head_position.y.abs_diff(tail_position.y) == 2
        {
            new_tail_position = Position {
                x: (head_position.x + tail_position.x) / 2,
                y: (head_position.y + tail_position.y) / 2,
            };
        } else {
            let x_change = if head_position.x > tail_position.x {
                1
            } else {
                -1
            };
            let y_change = if head_position.y > tail_position.y {
                1
            } else {
                -1
            };
            new_tail_position = Position {
                x: tail_position.x + x_change,
                y: tail_position.y + y_change,
            };
        }

        Positions {
            head: head_position,
            tail: new_tail_position,
        }
    }
}

pub fn navigate(start: Positions, movements: Vec<Movement>) -> Journey {
    let mut positions = start.clone();
    let mut visited = vec!(start);
    for movement in movements {
        for _ in 0..movement.distance {
            positions = move_one_step(positions, movement.direction);
            visited.push(positions);
        }
    }
    Journey{start: start, end: positions, visited: visited}
}

#[cfg(test)]
mod day9_tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = String::from("U 1\nR 2\nL 7\n D 8\nU 4");
        let expected = vec![
            Movement {
                distance: 1,
                direction: Direction::UP,
            },
            Movement {
                distance: 2,
                direction: Direction::RIGHT,
            },
            Movement {
                distance: 7,
                direction: Direction::LEFT,
            },
            Movement {
                distance: 8,
                direction: Direction::DOWN,
            },
            Movement {
                distance: 4,
                direction: Direction::UP,
            },
        ];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_first_move_right() {
        let starting_positions = Positions {
            head: Position { x: 0, y: 0 },
            tail: Position { x: 0, y: 0 },
        };
        let expected = Positions {
            head: Position { x: 1, y: 0 },
            tail: Position { x: 0, y: 0 },
        };
        assert_eq!(
            navigate(
                starting_positions,
                vec!(Movement {
                    direction: Direction::RIGHT,
                    distance: 1
                })
            ).end,
            expected
        );
    }

    #[test]
    fn test_second_move_right() {
        let starting_positions = Positions {
            head: Position { x: 1, y: 0 },
            tail: Position { x: 0, y: 0 },
        };
        let expected = Positions {
            head: Position { x: 2, y: 0 },
            tail: Position { x: 1, y: 0 },
        };
        assert_eq!(
            navigate(
                starting_positions,
                vec!(Movement {
                    direction: Direction::RIGHT,
                    distance: 1
                })
            ).end,
            expected
        );
    }

    #[test]
    fn test_third_move_right() {
        let starting_positions = Positions {
            head: Position { x: 2, y: 0 },
            tail: Position { x: 1, y: 0 },
        };
        let expected = Positions {
            head: Position { x: 3, y: 0 },
            tail: Position { x: 2, y: 0 },
        };
        assert_eq!(
            navigate(
                starting_positions,
                vec!(Movement {
                    direction: Direction::RIGHT,
                    distance: 1
                })
            ).end,
            expected
        );
    }

    #[test]
    fn test_fourth_move_right() {
        let starting_positions = Positions {
            head: Position { x: 3, y: 0 },
            tail: Position { x: 2, y: 0 },
        };
        let expected = Positions {
            head: Position { x: 4, y: 0 },
            tail: Position { x: 3, y: 0 },
        };
        assert_eq!(
            navigate(
                starting_positions,
                vec!(Movement {
                    direction: Direction::RIGHT,
                    distance: 1
                })
            ).end,
            expected
        );
    }

    #[test]
    fn test_first_move_up() {
        let starting_positions = Positions {
            head: Position { x: 4, y: 0 },
            tail: Position { x: 3, y: 0 },
        };
        let expected = Positions {
            head: Position { x: 4, y: -1 },
            tail: Position { x: 3, y: 0 },
        };
        assert_eq!(
            navigate(
                starting_positions,
                vec!(Movement {
                    direction: Direction::UP,
                    distance: 1
                })
            ).end,
            expected
        );
    }

    #[test]
    fn test_second_move_up() {
        let starting_positions = Positions {
            head: Position { x: 4, y: -1 },
            tail: Position { x: 3, y: 0 },
        };
        let expected = Positions {
            head: Position { x: 4, y: -2 },
            tail: Position { x: 4, y: -1 },
        };
        assert_eq!(
            navigate(
                starting_positions,
                vec!(Movement {
                    direction: Direction::UP,
                    distance: 1
                })
            ).end,
            expected
        );
    }

    #[test]
    fn test_third_move_up() {
        let starting_positions = Positions {
            head: Position { x: 4, y: -2 },
            tail: Position { x: 4, y: -1 },
        };
        let expected = Positions {
            head: Position { x: 4, y: -3 },
            tail: Position { x: 4, y: -2 },
        };
        assert_eq!(
            navigate(
                starting_positions,
                vec!(Movement {
                    direction: Direction::UP,
                    distance: 1
                })
            ).end,
            expected
        );
    }

    #[test]
    fn test_fourth_move_up() {
        let starting_positions = Positions {
            head: Position { x: 4, y: -3 },
            tail: Position { x: 4, y: -2 },
        };
        let expected = Positions {
            head: Position { x: 4, y: -4 },
            tail: Position { x: 4, y: -3 },
        };
        assert_eq!(
            navigate(
                starting_positions,
                vec!(Movement {
                    direction: Direction::UP,
                    distance: 1
                })
            ).end,
            expected
        );
    }

    #[test]
    fn test_first_move_left() {
        let starting_positions = Positions {
            head: Position { x: 4, y: -4 },
            tail: Position { x: 4, y: -3 },
        };
        let expected = Positions {
            head: Position { x: 3, y: -4 },
            tail: Position { x: 4, y: -3 },
        };
        assert_eq!(
            navigate(
                starting_positions,
                vec!(Movement {
                    direction: Direction::LEFT,
                    distance: 1
                })
            ).end,
            expected
        );
    }

    #[test]
    fn test_second_move_left() {
        let starting_positions = Positions {
            head: Position { x: 3, y: -4 },
            tail: Position { x: 4, y: -3 },
        };
        let expected = Positions {
            head: Position { x: 2, y: -4 },
            tail: Position { x: 3, y: -4 },
        };
        assert_eq!(
            navigate(
                starting_positions,
                vec!(Movement {
                    direction: Direction::LEFT,
                    distance: 1
                })
            ).end,
            expected
        );
    }

    #[test]
    fn test_third_move_left() {
        let starting_positions = Positions {
            head: Position { x: 2, y: -4 },
            tail: Position { x: 3, y: -4 },
        };
        let expected = Positions {
            head: Position { x: 1, y: -4 },
            tail: Position { x: 2, y: -4 },
        };
        assert_eq!(
            navigate(
                starting_positions,
                vec!(Movement {
                    direction: Direction::LEFT,
                    distance: 1
                })
            ).end,
            expected
        );
    }

    #[test]
    fn test_first_move_down() {
        let starting_positions = Positions {
            head: Position { x: 1, y: -4 },
            tail: Position { x: 2, y: -4 },
        };
        let expected = Positions {
            head: Position { x: 1, y: -3 },
            tail: Position { x: 2, y: -4 },
        };
        assert_eq!(
            navigate(
                starting_positions,
                vec!(Movement {
                    direction: Direction::DOWN,
                    distance: 1
                })
            ).end,
            expected
        );
    }

    #[test]
    fn test_all_moves() {
        let starting_positions = Positions {
            head: Position { x: 0, y: 0 },
            tail: Position { x: 0, y: 0 },
        };
        let expected = Positions {
            head: Position { x: 2, y: -2 },
            tail: Position { x: 1, y: -2 },
        };
        let moves = vec!(
            Movement{direction: Direction::RIGHT, distance: 4},
            Movement{direction: Direction::UP, distance: 4},
            Movement{direction: Direction::LEFT, distance: 3},
            Movement{direction: Direction::DOWN, distance: 1},
            Movement{direction: Direction::RIGHT, distance: 4},
            Movement{direction: Direction::DOWN, distance: 1},
            Movement{direction: Direction::LEFT, distance: 5},
            Movement{direction: Direction::RIGHT, distance: 2},
        );
        let result = navigate(
            starting_positions,
            moves
        );
        assert_eq!(result.start, starting_positions);
        assert_eq!(result.end, expected);
        assert_eq!(result.visited.len(), 25);
    }
}
