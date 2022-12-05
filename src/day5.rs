use regex::Regex;

#[derive(PartialEq, Debug)]
pub struct Movement {
    quantity: u8,
    from_stack: u8,
    to_stack: u8
}

#[derive(PartialEq, Debug)]
pub struct Parameters {
    stacks: Vec<Vec<char>>,
    movements: Vec<Movement>
}

fn string_chars(s: &str) -> Vec<char> {
    s.as_bytes().iter().map(|b| *b as char ).collect()
}

fn parse_stacks(lines: Vec<&str>) -> Vec<Vec<char>> {
    let mut populated_column_indexes: Vec<usize> = Vec::new();
    let mut stacks: Vec<Vec<char>> = Vec::new();
    populated_column_indexes.push(0);
    stacks.push(vec!()); // Populate it, but it's unused
    let layer0 = string_chars(lines[0]);

    //println!("Layer0 {:?}", layer0);
    for i in 0..layer0.len() {
        if layer0[i] != ' ' {
            populated_column_indexes.push(i); // Assume consecutive
            stacks.push(vec!());
        }
    }

    //println!("populated_column_indexes {:?}", populated_column_indexes);
    for line_index in 1..lines.len() {
        for column_index in 1..populated_column_indexes.len() {
            let layer = string_chars(lines[line_index]);
            let populated_column_index = populated_column_indexes[column_index];
            if layer[populated_column_index] != ' ' {
                //println!("column_index {} layer {}", column_index, layer[populated_column_index]);
                let stack_identifier = layer[populated_column_index];
                stacks[column_index].push(stack_identifier);
            }
        }
    }
    stacks
}

fn parse_movements(lines: Vec<&str>) -> Vec<Movement> {
    let mut movements = Vec::new();
    for line in lines {
        let re = Regex::new(r"(\d+).*(\d+).*(\d+)").unwrap();
        let caps = re.captures(line).unwrap();

        let quantity = caps.get(1).map_or("", |m| m.as_str()).parse::<u8>().unwrap();
        let from = caps.get(2).map_or("", |m| m.as_str()).parse::<u8>().unwrap();
        let to = caps.get(3).map_or("", |m| m.as_str()).parse::<u8>().unwrap();
        movements.push(Movement{quantity: quantity, from_stack: from, to_stack: to});
    }
    movements
}

pub fn parse(input: String) -> Parameters {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let stacks = parse_stacks(sections[0].lines().rev().collect());
    let movements = parse_movements(sections[1].lines().collect());
    Parameters{stacks: stacks, movements: movements}
}

#[cfg(test)]
mod day5_tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let input = format!("{}\n{}\n{}\n{}\n\n{}",
            "    [D]    ",
            "[N] [C]    ",
            "[Z] [M] [P]",
            " 1   2   3 ",
            "move 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2\n");
        let parameters = parse(input);
        assert_eq!(parameters.stacks[1], vec!['Z', 'N']);
        assert_eq!(parameters.stacks[2], vec!['M', 'C', 'D']);
        assert_eq!(parameters.stacks[3], vec!['P']);
        assert_eq!(parameters.movements[0], Movement{quantity:1, from_stack:2, to_stack:1});
        assert_eq!(parameters.movements[1], Movement{quantity:3, from_stack:1, to_stack:3});
        assert_eq!(parameters.movements[2], Movement{quantity:2, from_stack:2, to_stack:1});
        assert_eq!(parameters.movements[3], Movement{quantity:1, from_stack:1, to_stack:2});
    }

    #[test]
    fn test_string_chars() {
        assert_eq!(string_chars("helloworld"), vec!['h', 'e', 'l', 'l', 'o', 'w', 'o', 'r', 'l', 'd']);
    }
}