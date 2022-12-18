use std::collections::HashSet;

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

#[derive(PartialEq, Debug, Clone, Copy, Hash, Eq)]
pub struct Position {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Debug, Clone, Hash, Eq)]
pub struct Rope {
    knots: Vec<Position>
}

#[derive(PartialEq, Debug)]
pub struct Journey {
    start: Rope,
    end: Rope,
    visited: Vec<Rope>
}

pub fn parse(input: String) -> Vec<Movement> {
    let mut result = Vec::new();
    for line in input.trim_end().split("\n") {
        let movement: Vec<&str> = line.trim().split(" ").collect();
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

fn move_head_one_step(rope: Rope, direction: Direction) -> Rope {
    let mut updated = Rope{knots: Vec::new()};
    updated.knots.push(match direction {
        Direction::RIGHT => Position {
            x: rope.knots[0].x + 1,
            y: rope.knots[0].y,
        },
        Direction::UP => Position {
            x: rope.knots[0].x,
            y: rope.knots[0].y - 1,
        },
        Direction::LEFT => Position {
            x: rope.knots[0].x-1,
            y: rope.knots[0].y,
        },
        Direction::DOWN => Position {
            x: rope.knots[0].x,
            y: rope.knots[0].y+1,
        }
    });

    for index in 1..rope.knots.len() {
        let current = rope.knots[index];
        let previous = updated.knots[index - 1];
        let knot_position = if previous.y.abs_diff(current.y) <= 1
            && previous.x.abs_diff(current.x) <= 1 {
            current
        } else if previous.y == current.y && previous.x.abs_diff(current.x) == 2
            || previous.x == current.x && previous.y.abs_diff(current.y) == 2 {
            Position {
                x: (previous.x + current.x) / 2,
                y: (previous.y + current.y) / 2,
            }
        } else {
            let x_change = if previous.x > current.x {
                1
            } else {
                -1
            };
            let y_change = if previous.y > current.y {
                1
            } else {
                -1
            };
            Position {
                x: current.x + x_change,
                y: current.y + y_change,
            }
        };
        updated.knots.push(knot_position);
    }
    updated
}

pub fn navigate(start: Rope, movements: Vec<Movement>) -> Vec<Position> {
    let mut rope = Rope{knots:start.knots.clone()};
    let mut tail_visited = Vec::new();
    for movement in movements {
        for _ in 0..movement.distance {
            rope = move_head_one_step(rope, movement.direction);
            tail_visited.push(rope.knots[rope.knots.len()-1]);
        }
    }
    tail_visited
}

pub fn count_visited(input: String) -> u32 {
    let moves = parse(input);
    let initial = Rope {knots: vec!(Position{x:0, y:0}, Position{x:0, y:0})};
    let tail_visited = navigate(initial, moves);
    let mut unique_visited = HashSet::new();
    for position in tail_visited {
        unique_visited.insert(position);
    }
    unique_visited.len() as u32
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
        let starting_positions = Rope {
            knots: vec!(Position { x: 0, y: 0 }, Position { x: 0, y: 0 })
        };
        let tail_visited = vec!(Position { x: 0, y: 0 });
        assert_eq!(
            navigate(
                starting_positions,
                vec!(Movement {
                    direction: Direction::RIGHT,
                    distance: 1
                })
            ),
            tail_visited
        );
    }

    #[test]
    fn test_second_move_right() {
        let starting_positions = Rope {
            knots: vec!(Position { x: 1, y: 0 }, Position { x: 0, y: 0 })
        };
        let tail_visited = vec!(Position { x: 1, y: 0 });
        assert_eq!(
            navigate(
                starting_positions,
                vec!(Movement {
                    direction: Direction::RIGHT,
                    distance: 1
                })
            ),
            tail_visited
        );
    }
    //
    // #[test]
    // fn test_third_move_right() {
    //     let starting_positions = Rope {
    //         head: Position { x: 2, y: 0 },
    //         tail: Position { x: 1, y: 0 },
    //     };
    //     let expected = Rope {
    //         head: Position { x: 3, y: 0 },
    //         tail: Position { x: 2, y: 0 },
    //     };
    //     assert_eq!(
    //         navigate(
    //             starting_positions,
    //             vec!(Movement {
    //                 direction: Direction::RIGHT,
    //                 distance: 1
    //             })
    //         ).end,
    //         expected
    //     );
    // }
    //
    // #[test]
    // fn test_fourth_move_right() {
    //     let starting_positions = Rope {
    //         head: Position { x: 3, y: 0 },
    //         tail: Position { x: 2, y: 0 },
    //     };
    //     let expected = Rope {
    //         head: Position { x: 4, y: 0 },
    //         tail: Position { x: 3, y: 0 },
    //     };
    //     assert_eq!(
    //         navigate(
    //             starting_positions,
    //             vec!(Movement {
    //                 direction: Direction::RIGHT,
    //                 distance: 1
    //             })
    //         ).end,
    //         expected
    //     );
    // }
    //
    // #[test]
    // fn test_first_move_up() {
    //     let starting_positions = Rope {
    //         head: Position { x: 4, y: 0 },
    //         tail: Position { x: 3, y: 0 },
    //     };
    //     let expected = Rope {
    //         head: Position { x: 4, y: -1 },
    //         tail: Position { x: 3, y: 0 },
    //     };
    //     assert_eq!(
    //         navigate(
    //             starting_positions,
    //             vec!(Movement {
    //                 direction: Direction::UP,
    //                 distance: 1
    //             })
    //         ).end,
    //         expected
    //     );
    // }
    //
    // #[test]
    // fn test_second_move_up() {
    //     let starting_positions = Rope {
    //         head: Position { x: 4, y: -1 },
    //         tail: Position { x: 3, y: 0 },
    //     };
    //     let expected = Rope {
    //         head: Position { x: 4, y: -2 },
    //         tail: Position { x: 4, y: -1 },
    //     };
    //     assert_eq!(
    //         navigate(
    //             starting_positions,
    //             vec!(Movement {
    //                 direction: Direction::UP,
    //                 distance: 1
    //             })
    //         ).end,
    //         expected
    //     );
    // }
    //
    // #[test]
    // fn test_third_move_up() {
    //     let starting_positions = Rope {
    //         head: Position { x: 4, y: -2 },
    //         tail: Position { x: 4, y: -1 },
    //     };
    //     let expected = Rope {
    //         head: Position { x: 4, y: -3 },
    //         tail: Position { x: 4, y: -2 },
    //     };
    //     assert_eq!(
    //         navigate(
    //             starting_positions,
    //             vec!(Movement {
    //                 direction: Direction::UP,
    //                 distance: 1
    //             })
    //         ).end,
    //         expected
    //     );
    // }
    //
    // #[test]
    // fn test_fourth_move_up() {
    //     let starting_positions = Rope {
    //         head: Position { x: 4, y: -3 },
    //         tail: Position { x: 4, y: -2 },
    //     };
    //     let expected = Rope {
    //         head: Position { x: 4, y: -4 },
    //         tail: Position { x: 4, y: -3 },
    //     };
    //     assert_eq!(
    //         navigate(
    //             starting_positions,
    //             vec!(Movement {
    //                 direction: Direction::UP,
    //                 distance: 1
    //             })
    //         ).end,
    //         expected
    //     );
    // }
    //
    // #[test]
    // fn test_first_move_left() {
    //     let starting_positions = Rope {
    //         head: Position { x: 4, y: -4 },
    //         tail: Position { x: 4, y: -3 },
    //     };
    //     let expected = Rope {
    //         head: Position { x: 3, y: -4 },
    //         tail: Position { x: 4, y: -3 },
    //     };
    //     assert_eq!(
    //         navigate(
    //             starting_positions,
    //             vec!(Movement {
    //                 direction: Direction::LEFT,
    //                 distance: 1
    //             })
    //         ).end,
    //         expected
    //     );
    // }
    //
    // #[test]
    // fn test_second_move_left() {
    //     let starting_positions = Rope {
    //         head: Position { x: 3, y: -4 },
    //         tail: Position { x: 4, y: -3 },
    //     };
    //     let expected = Rope {
    //         head: Position { x: 2, y: -4 },
    //         tail: Position { x: 3, y: -4 },
    //     };
    //     assert_eq!(
    //         navigate(
    //             starting_positions,
    //             vec!(Movement {
    //                 direction: Direction::LEFT,
    //                 distance: 1
    //             })
    //         ).end,
    //         expected
    //     );
    // }
    //
    // #[test]
    // fn test_third_move_left() {
    //     let starting_positions = Rope {
    //         head: Position { x: 2, y: -4 },
    //         tail: Position { x: 3, y: -4 },
    //     };
    //     let expected = Rope {
    //         head: Position { x: 1, y: -4 },
    //         tail: Position { x: 2, y: -4 },
    //     };
    //     assert_eq!(
    //         navigate(
    //             starting_positions,
    //             vec!(Movement {
    //                 direction: Direction::LEFT,
    //                 distance: 1
    //             })
    //         ).end,
    //         expected
    //     );
    // }
    //
    // #[test]
    // fn test_first_move_down() {
    //     let starting_positions = Rope {
    //         head: Position { x: 1, y: -4 },
    //         tail: Position { x: 2, y: -4 },
    //     };
    //     let expected = Rope {
    //         head: Position { x: 1, y: -3 },
    //         tail: Position { x: 2, y: -4 },
    //     };
    //     assert_eq!(
    //         navigate(
    //             starting_positions,
    //             vec!(Movement {
    //                 direction: Direction::DOWN,
    //                 distance: 1
    //             })
    //         ).end,
    //         expected
    //     );
    // }
    //
    // #[test]
    // fn test_all_moves() {
    //     let starting_positions = Rope {
    //         head: Position { x: 0, y: 0 },
    //         tail: Position { x: 0, y: 0 },
    //     };
    //     let expected = Rope {
    //         head: Position { x: 2, y: -2 },
    //         tail: Position { x: 1, y: -2 },
    //     };
    //     let moves = vec!(
    //         Movement{direction: Direction::RIGHT, distance: 4},
    //         Movement{direction: Direction::UP, distance: 4},
    //         Movement{direction: Direction::LEFT, distance: 3},
    //         Movement{direction: Direction::DOWN, distance: 1},
    //         Movement{direction: Direction::RIGHT, distance: 4},
    //         Movement{direction: Direction::DOWN, distance: 1},
    //         Movement{direction: Direction::LEFT, distance: 5},
    //         Movement{direction: Direction::RIGHT, distance: 2},
    //     );
    //     let result = navigate(
    //         starting_positions,
    //         moves
    //     );
    //     assert_eq!(result.start, starting_positions);
    //     assert_eq!(result.end, expected);
    //     assert_eq!(result.visited.len(), 25);
    // }
    //
    // #[test]
    // fn acceptance_test() {
    //     let moves = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
    //     let locations_visited =count_visited(moves.to_string());
    //     assert_eq!(locations_visited, 13);
    // }
}
