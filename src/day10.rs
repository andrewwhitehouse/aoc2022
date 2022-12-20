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

pub fn execute(instructions: Vec<Instruction>, sample_at_cycle: Vec<u32>) -> Vec<i32> {
    let mut x = 1;
    let mut cycle = 1;
    let mut sample_index = 0;
    let mut sample_values: Vec<i32> = Vec::new();
    for instruction in instructions {
        let mut new_x;
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
            if cycle+i == sample_at_cycle[sample_index] {
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
}

