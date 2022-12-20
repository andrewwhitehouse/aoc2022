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
    Vec::new()
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
        let result = execute(instructions, (1..=5).collect());
        assert_eq!(result, vec!(1, 1, 1, 4, 4, -1));
    }
}

