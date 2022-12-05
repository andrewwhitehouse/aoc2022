pub struct Movement {
    quantity: u8,
    from_stack: u8,
    to_stack: u8
}

pub struct Parameters {
    stacks: Vec<Vec<char>>,
    movements: Vec<Movement>
}

fn string_chars(s: &str) -> Vec<char> {
    s.as_bytes().iter().map(|b| *b as char ).collect()
}

// fn parse_stacks(lines: Vec<&str>) -> Vec<Vec<char>> {
//     let mut column_indexes: Vec<u8> = Vec::new();
//     column_indexes.push(0); // Populate it, but it's unused
//     let layer0 : bytes = lines[0].as_bytes();
//     for i in 0..lines[0].len() {
//         if lines[0][i] != ' ' {
//             let column_number = (lines[0][i] as usize) - 48;
//             column_indexes[column_number] = i;
//         }
//     }
//
//     let mut stacks: Vec<Vec<char>> = Vec::new();
//     stacks.push(vec!()); // index zero
//     for layer in lines[1..] {
//         for column_index in column_indexes {
//             if layer[column_index] != ' ' {
//                 let stack_identifier = layer.as_bytes()[column_index] as char;
//                 stacks[column_index].push(stack_identifier);
//             }
//         }
//     }
//     stacks
// }


// pub fn parse(input: String) -> Parameters {
//     let sections: Vec<&str> = input.split("\n\n").collect();
//     let stacks = parse_stacks(sections[0].lines().collect());
//     Parameters{stacks: stacks, movements: vec!()}
// }

#[cfg(test)]
mod day5_tests {
    use super::*;

    // #[test]
    // fn test_parsing() {
    //     let input = format!("{}\n{}\n{}\n{}\n\n{}",
    //         "    [D]    ",
    //         "[N] [C]    ",
    //         "[Z] [M] [P]",
    //         " 1   2   3 ",
    //         "move 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2\n");
    //     let parameters = parse(input);
    //     assert_eq!(parameters.stacks[1], vec!['N', 'Z']);
    //     assert_eq!(parameters.stacks[1], vec!['D', 'C', 'M']);
    //     assert_eq!(parameters.stacks[1], vec!['P']);
    // }

    #[test]
    fn test_string_chars() {
        assert_eq!(string_chars("helloworld"), vec!['h', 'e', 'l', 'l', 'o', 'w', 'o', 'r', 'l', 'd']);
    }
}