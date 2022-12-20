use std::option::*;

pub enum InstructionType {
    ADDX,
    NOOP
}

pub struct Instruction {
    instruction_type: InstructionType,
    parmeter: Option<i32>
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

    #[test]
    fn simple_example() {
        let instructions = vec!(
            Instruction{instruction_type: InstructionType::NOOP, parmeter: None},
            Instruction{instruction_type: InstructionType::ADDX, parmeter: Some(3)},
            Instruction{instruction_type: InstructionType::ADDX, parmeter: Some(-5)},
        );
        let result = execute(instructions, (1..=6).collect());
        assert_eq!(result, vec!(1, 1, 1, 4, 4, -1));
    }
}

