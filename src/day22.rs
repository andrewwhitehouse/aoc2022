#[derive(PartialEq, Debug)]
pub struct Row {
    offset: u32,
    open_tiles: Vec<char>,
}

#[derive(PartialEq, Debug)]
pub struct BoardMap {
    rows: Vec<Row>,
}

#[derive(PartialEq, Debug)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST
}

#[derive(PartialEq, Debug)]
pub struct Movement {
    direction: Direction,
    distance: u32,
}

#[derive(PartialEq, Debug)]
pub struct Instructions {
    map: BoardMap,
    movements: Vec<Movement>,
}

pub fn parse(input: String) -> Instructions {
    todo!()
}

#[cfg(test)]
mod day22_tests {
    use super::*;

    fn example_input() -> String {
        format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "        ...#",
            "        .#..",
            "        #...",
            "        ....",
            "...#.......#",
            "........#...",
            "..#....#....",
            "..........#.",
            "        ...#....",
            "        .....#..",
            "        .#......",
            "        ......#.",
            "",
            "10R5L5R10L4R5L5"
        )
    }

    fn string_chars(s: &str) -> Vec<char> {
        s.as_bytes().iter().map(|b| *b as char).collect()
    }

    #[test]
    fn test_parse() {
        let expected_rows = vec![
            Row {
                offset: 8,
                open_tiles: string_chars("...#"),
            },
            Row {
                offset: 8,
                open_tiles: string_chars(".#.."),
            },
            Row {
                offset: 8,
                open_tiles: string_chars("#..."),
            },
            Row {
                offset: 8,
                open_tiles: string_chars("...."),
            },
            Row {
                offset: 0,
                open_tiles: string_chars("...#......."),
            },
            Row {
                offset: 0,
                open_tiles: string_chars("........#..."),
            },
            Row {
                offset: 0,
                open_tiles: string_chars("..#....#...."),
            },
            Row {
                offset: 0,
                open_tiles: string_chars("..........#."),
            },
            Row {
                offset: 8,
                open_tiles: string_chars("...#...."),
            },
            Row {
                offset: 8,
                open_tiles: string_chars(".....#.."),
            },
            Row {
                offset: 8,
                open_tiles: string_chars(".#......"),
            },
            Row {
                offset: 8,
                open_tiles: string_chars("......#."),
            },
        ];
        let expected_movements = vec![
            Movement {
                direction: Direction::EAST,
                distance: 10,
            },
            Movement {
                direction: Direction::SOUTH,
                distance: 5,
            },
            Movement {
                direction: Direction::EAST,
                distance: 5,
            },
            Movement {
                direction: Direction::SOUTH,
                distance: 10,
            },
            Movement {
                direction: Direction::EAST,
                distance: 4,
            },
            Movement {
                direction: Direction::SOUTH,
                distance: 5,
            },
            Movement {
                direction: Direction::EAST,
                distance: 5,
            },
        ];
        let expected_instructions = Instructions {
            map: BoardMap {
                rows: expected_rows,
            },
            movements: expected_movements,
        };
        assert_eq!(parse(example_input()), expected_instructions);
    }
}
