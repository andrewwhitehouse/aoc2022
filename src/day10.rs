#[derive(PartialEq, Debug)]
pub enum InstructionType {
    ADDX,
    NOOP
}

#[derive(PartialEq, Debug)]
pub struct Instruction {
    instruction_type: InstructionType,
    parmeter: Option<i32>
}

pub fn parse(input: String) -> Vec<Instruction> {
    let mut result = Vec::new();
    for line in input.trim_end().split("\n") {
        let parts: Vec<&str> = line.split(" ").collect();
        match parts[0] {
            "addx" => result.push(Instruction{
                instruction_type: InstructionType::ADDX,
                parmeter: Some(parts[1].parse::<i32>().unwrap())}),
            "noop" => result.push(Instruction{
                instruction_type: InstructionType::NOOP,
                parmeter: None}),
            _ => panic!("Unrecognised instruction {}", line)
        };
    }
    result
}

pub fn execute(instructions: Vec<Instruction>, sample_at_cycle: Vec<usize>) -> Vec<i32> {
    let mut x = 1;
    let mut cycle = 1;
    let mut sample_index = 0;
    let mut sample_values: Vec<i32> = Vec::new();
    for instruction in instructions {
        let new_x;
        let cycles_for_instruction = match instruction.instruction_type {
            InstructionType::NOOP => {
                new_x = x;
                1
            },
            InstructionType::ADDX => {
                new_x = x + instruction.parmeter.unwrap();
                2
            }
        };
        for i in 0..cycles_for_instruction {
            if sample_index < sample_at_cycle.len() && cycle+i == sample_at_cycle[sample_index] {
                sample_values.push(x);
                sample_index += 1;
            }
        }
        x = new_x;
        cycle += cycles_for_instruction;
    }
    sample_values.push(x);
    sample_values
}

pub fn solve_part1(input: String) -> i32 {
    let sample_points: Vec<usize> = vec!(20, 60, 100, 140, 180, 220);
    let instructions = parse(input);
    let result = execute(instructions, sample_points.clone());
    let mut total = 0i32;
    for i in 0..sample_points.len() {
        let value: i32 = result[i] * (*&sample_points[i] as i32);
        total += value;
    }
    total
}

pub fn solve_part2(input: String) -> String {
    let instructions = parse(input);
    let lines = crt_display(instructions);
    lines.join("\n")
}

pub fn crt_display(instructions: Vec<Instruction>) -> Vec<String> {
    let mut x = 1;
    let mut output: String = String::from("");
    let mut cycle = 1;
    for instruction in instructions {
        let new_x;
        let cycles_for_instruction = match instruction.instruction_type {
            InstructionType::NOOP => {
                new_x = x;
                1
            },
            InstructionType::ADDX => {
                new_x = x + instruction.parmeter.unwrap();
                2
            }
        };
        for _ in 0..cycles_for_instruction {
            let col = (cycle - 1) % 40;
            if x-1 == col || x == col || x+1 == col {
                output.push_str("#")
            } else {
                output.push_str(".");
            }
            cycle += 1;
        }
        x = new_x;
    }
    output.chars()
        .collect::<Vec<char>>()
        .chunks(40)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod day10_tests {
    use super::*;

    fn example_instructions() -> Vec<Instruction> {
        vec!(
            Instruction{instruction_type: InstructionType::NOOP, parmeter: None},
            Instruction{instruction_type: InstructionType::ADDX, parmeter: Some(3)},
            Instruction{instruction_type: InstructionType::ADDX, parmeter: Some(-5)},
        )
    }

    #[test]
    fn simple_example() {
        let result = execute(example_instructions(), (1..=6).collect());
        assert_eq!(result, vec!(1, 1, 1, 4, 4, -1));
    }

    #[test]
    fn simple_parse() {
        let input = "noop\naddx 3\naddx -5\n";
        assert_eq!(parse(input.to_string()), example_instructions());
    }

    fn longer_input() -> String {
        format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\n",
            "noop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\n",
            "addx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\n",
            "noop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\n",
            "addx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\n",
            "addx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\n",
            "noop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\n",
            "addx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\n",
            "addx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\n",
            "noop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\n",
            "addx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\n",
            "addx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\n",
            "addx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\n",
            "noop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\n",
            "noop\nnoop\nnoop\n")
    }

    #[test]
    fn longer_parse() {
        assert_eq!(parse(longer_input()).len(), 146);
    }

    #[test]
    fn check_signals() {
        let sample_points = vec!(20, 60, 100, 140, 180, 220);
        let instructions = parse(longer_input());
        let result = execute(instructions, sample_points);
        assert_eq!(result, vec!(420/20, 1140/60, 1800/100, 2940/140, 2880/180, 3960/220, 17));
    }

    #[test]
    fn test_solve_part1_example() {
        assert_eq!(solve_part1(longer_input()), 13140);
    }

    #[test]
    fn test_crt_display() {
        let output = crt_display(parse(longer_input()));
        let expected = vec!(
            "##..##..##..##..##..##..##..##..##..##..",
            "###...###...###...###...###...###...###.",
            "####....####....####....####....####....",
            "#####.....#####.....#####.....#####.....",
            "######......######......######......####",
            "#######.......#######.......#######....."
        );
        assert_eq!(output, expected);
    }
}

