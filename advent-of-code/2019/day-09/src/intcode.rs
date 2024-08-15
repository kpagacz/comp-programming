use std::collections::HashMap;
use std::collections::VecDeque;

pub type IntUnit = i64;
type OpCode = i64;
type ParameterModes = Vec<IntUnit>;

const END_OP: IntUnit = 99;
const ADD_OP: IntUnit = 1;
const MULTIPLY_OP: IntUnit = 2;
const INPUT_OP: IntUnit = 3;
const OUTPUT_OP: IntUnit = 4;
const JUMP_IF_TRUE_OP: IntUnit = 5;
const JUMP_IF_FALSE_OP: IntUnit = 6;
const LESS_THAN_OP: IntUnit = 7;
const EQUALS_OP: IntUnit = 8;
const ADJUST_BASE: IntUnit = 9;

const POSITION_MODE: IntUnit = 0;
const IMMEDIATE_MODE: IntUnit = 1;
const RELATIVE_MODE: IntUnit = 2;

pub struct IntCode {
    pub memory: HashMap<usize, IntUnit>,
    pub input: VecDeque<IntUnit>,
    pub last_output: Option<IntUnit>,
    original_input: String,
    it: usize,
    relative_base: IntUnit,
}

impl IntCode {
    pub fn from(input: &str) -> Self {
        Self {
            memory: input
                .trim()
                .split(',')
                .enumerate()
                .map(|(pos, code)| {
                    (
                        pos,
                        code.parse::<IntUnit>()
                            .expect("IntCode program always has ints only"),
                    )
                })
                .collect(),
            input: VecDeque::new(),
            original_input: input.to_string(),
            it: 0,
            last_output: None,
            relative_base: 0,
        }
    }

    pub fn is_terminated(&self) -> bool {
        *self.memory.get(&self.it).unwrap_or(&0) == END_OP
    }

    pub fn run(&mut self, input: Vec<IntUnit>) -> Vec<IntUnit> {
        let mut output = vec![];
        self.input = input.into();

        while let Some(value) = self.run_interruptible() {
            output.push(value);
        }

        output
    }

    pub fn reset(&mut self) {
        self.memory = self
            .original_input
            .trim()
            .split(',')
            .enumerate()
            .map(|(pos, code)| {
                (
                    pos,
                    code.parse::<IntUnit>()
                        .expect("IntCode program always has ints only"),
                )
            })
            .collect();
        self.it = 0;
        self.input = VecDeque::default();
        self.last_output = None;
    }

    pub fn run_interruptible(&mut self) -> Option<IntUnit> {
        while self.it < self.memory.len() && self.access_memory(self.it) != END_OP {
            let (opcode, parameter_modes) = Self::decode_instruction(self.access_memory(self.it));

            match opcode {
                ADD_OP => {
                    let (operand1, operand2, target) = (
                        self.translate_operand(parameter_modes[0], self.access_memory(self.it + 1)),
                        self.translate_operand(parameter_modes[1], self.access_memory(self.it + 2)),
                        self.translate_target(parameter_modes[2], self.access_memory(self.it + 3)),
                    );
                    self.write_memory(target, operand1 + operand2);
                    self.it += 4;
                }

                MULTIPLY_OP => {
                    let (operand1, operand2, target) = (
                        self.translate_operand(parameter_modes[0], self.access_memory(self.it + 1)),
                        self.translate_operand(parameter_modes[1], self.access_memory(self.it + 2)),
                        self.translate_target(parameter_modes[2], self.access_memory(self.it + 3)),
                    );
                    self.write_memory(target, operand1 * operand2);
                    self.it += 4;
                }

                INPUT_OP => {
                    let target = if parameter_modes[0] == RELATIVE_MODE {
                        (self.access_memory(self.it + 1) + self.relative_base) as usize
                    } else {
                        self.access_memory(self.it + 1) as usize
                    };
                    let value = self.input.pop_front()?;
                    self.write_memory(target, value);
                    self.it += 2;
                }

                OUTPUT_OP => {
                    let operand =
                        self.translate_operand(parameter_modes[0], self.access_memory(self.it + 1));
                    self.it += 2;
                    self.last_output = Some(operand);
                    return Some(operand);
                }

                JUMP_IF_TRUE_OP => {
                    let operand1 =
                        self.translate_operand(parameter_modes[0], self.access_memory(self.it + 1));
                    let operand2 =
                        self.translate_operand(parameter_modes[1], self.access_memory(self.it + 2));

                    if operand1 != 0 {
                        self.it = operand2 as usize;
                    } else {
                        self.it += 3;
                    }
                }

                JUMP_IF_FALSE_OP => {
                    let operand1 =
                        self.translate_operand(parameter_modes[0], self.access_memory(self.it + 1));
                    let operand2 =
                        self.translate_operand(parameter_modes[1], self.access_memory(self.it + 2));
                    if operand1 == 0 {
                        self.it = operand2 as usize;
                    } else {
                        self.it += 3;
                    }
                }

                LESS_THAN_OP => {
                    let (operand1, operand2, target) = (
                        self.translate_operand(parameter_modes[0], self.access_memory(self.it + 1)),
                        self.translate_operand(parameter_modes[1], self.access_memory(self.it + 2)),
                        self.translate_target(parameter_modes[2], self.access_memory(self.it + 3)),
                    );
                    if operand1 < operand2 {
                        self.write_memory(target, 1);
                    } else {
                        self.write_memory(target, 0);
                    }
                    self.it += 4;
                }

                EQUALS_OP => {
                    let (operand1, operand2, target) = (
                        self.translate_operand(parameter_modes[0], self.access_memory(self.it + 1)),
                        self.translate_operand(parameter_modes[1], self.access_memory(self.it + 2)),
                        self.translate_target(parameter_modes[2], self.access_memory(self.it + 3)),
                    );
                    if operand1 == operand2 {
                        self.write_memory(target, 1);
                    } else {
                        self.write_memory(target, 0);
                    }
                    self.it += 4;
                }

                ADJUST_BASE => {
                    let operand =
                        self.translate_operand(parameter_modes[0], self.access_memory(self.it + 1));
                    self.relative_base += operand;
                    self.it += 2;
                }

                _ => unreachable!(
                    "Unknown operation found at {}. Operation: {}",
                    self.it, opcode
                ),
            }
        }

        None
    }

    fn translate_operand(&self, parameter_mode: IntUnit, value: IntUnit) -> IntUnit {
        match parameter_mode {
            POSITION_MODE => self.access_memory(value as usize),
            IMMEDIATE_MODE => value,
            RELATIVE_MODE => self.access_memory((value + self.relative_base) as usize),
            _ => unreachable!("Unknown parameter mode found"),
        }
    }

    fn translate_target(&self, parameter_mode: IntUnit, value: IntUnit) -> usize {
        match parameter_mode {
            POSITION_MODE => value as usize,
            RELATIVE_MODE => (value + self.relative_base) as usize,
            _ => unreachable!("Illegal target parameter mode: {parameter_mode} when translating to target position")
        }
    }

    fn decode_instruction(mut value: IntUnit) -> (OpCode, ParameterModes) {
        let opcode = value % 100;
        value /= 100;
        let mut parameter_modes = vec![];

        while value > 0 {
            parameter_modes.push(value % 10);
            value /= 10;
        }

        match opcode {
            ADD_OP | MULTIPLY_OP | LESS_THAN_OP | EQUALS_OP => {
                while parameter_modes.len() < 3 {
                    parameter_modes.push(0);
                }
            }
            JUMP_IF_FALSE_OP | JUMP_IF_TRUE_OP => {
                while parameter_modes.len() < 2 {
                    parameter_modes.push(0);
                }
            }
            INPUT_OP | OUTPUT_OP | ADJUST_BASE => {
                while parameter_modes.is_empty() {
                    parameter_modes.push(0);
                }
            }
            _ => unreachable!("Unknown opcode while decoding instruction"),
        }

        (opcode, parameter_modes)
    }

    #[inline]
    fn access_memory(&self, position: usize) -> IntUnit {
        *self.memory.get(&position).unwrap_or(&0)
    }

    #[inline]
    fn write_memory(&mut self, position: usize, value: IntUnit) {
        self.memory.insert(position, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2examples() {
        let inputs = [
            "1,0,0,0,99",
            "2,3,0,3,99",
            "2,4,4,5,99,0",
            "1,1,1,4,99,5,6,0,99",
        ];
        let final_memory = [
            vec![2, 0, 0, 0, 99],
            vec![2, 3, 0, 6, 99],
            vec![2, 4, 4, 5, 99, 9801],
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
        ];

        for (input, final_state) in inputs.iter().zip(final_memory) {
            let mut intcode = IntCode::from(input);
            intcode.run(vec![]);
            // assert_eq!(intcode.memory[..final_state.len()], final_state);
        }
    }

    #[test]
    fn day5examples() {
        let programs = ["3,0,4,0,99"];
        let input = [vec![5]];
        let output = [vec![5]];

        for i in 0..programs.len() {
            let mut intcode = IntCode::from(programs[i]);
            assert_eq!(intcode.run(input[i].clone()), output[i].clone());
        }
    }

    #[test]
    fn day5examples2() {
        println!("Program 1002,4,3,4,33");
        let mut intcode = IntCode::from("1002,4,3,4,33");
        intcode.run(vec![]);
        println!("Program 1101,100,-1,4,0");
        let mut intcode = IntCode::from("1101,100,-1,4,0");
        intcode.run(vec![]);
    }

    #[test]
    fn day5part2examples() {
        // Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not)
        let test_cases = [
            ("3,9,8,9,10,9,4,9,99,-1,8", vec![1], vec![0]),
            ("3,9,8,9,10,9,4,9,99,-1,8", vec![8], vec![1]),
        ];
        for (program, input, output) in test_cases {
            let mut intcode = IntCode::from(program);
            assert_eq!(intcode.run(input), output);
        }
        // Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
        let test_cases = [
            ("3,9,7,9,10,9,4,9,99,-1,8", vec![1], vec![1]),
            ("3,9,7,9,10,9,4,9,99,-1,8", vec![8], vec![0]),
        ];
        for (program, input, output) in test_cases {
            let mut intcode = IntCode::from(program);
            assert_eq!(intcode.run(input), output);
        }
        // Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
        let test_cases = [
            ("3,3,1108,-1,8,3,4,3,99", vec![1], vec![0]),
            ("3,3,1108,-1,8,3,4,3,99", vec![8], vec![1]),
        ];
        for (program, input, output) in test_cases {
            let mut intcode = IntCode::from(program);
            assert_eq!(intcode.run(input), output);
        }
        // Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
        let test_cases = [
            ("3,3,1107,-1,8,3,4,3,99", vec![1], vec![1]),
            ("3,3,1107,-1,8,3,4,3,99", vec![8], vec![0]),
        ];
        for (program, input, output) in test_cases {
            let mut intcode = IntCode::from(program);
            assert_eq!(intcode.run(input), output);
        }
    }

    #[test]
    fn day5jumps() {
        // Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero
        let test_cases = [
            ("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", vec![0], vec![0]),
            ("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", vec![8], vec![1]),
        ];
        for (program, input, output) in test_cases {
            let mut intcode = IntCode::from(program);
            assert_eq!(intcode.run(input), output);
        }
        let test_cases = [
            ("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", vec![0], vec![0]),
            ("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", vec![8], vec![1]),
        ];
        for (program, input, output) in test_cases {
            let mut intcode = IntCode::from(program);
            assert_eq!(intcode.run(input), output);
        }
    }

    #[test]
    fn day5largerexample() {
        // The above example program uses an input instruction to ask for a single number.
        // The program will then output 999 if the input value is below 8, output 1000
        // if the input value is equal to 8, or output 1001 if the input value is greater than 8.
        let test_cases = [
            (
                "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,\
                    0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,\
                    20,4,20,1105,1,46,98,99",
                vec![0],
                vec![999],
            ),
            (
                "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,\
                21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,\
                98,99",
                vec![8],
                vec![1000],
            ),
            (
                "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,\
                20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
                vec![9],
                vec![1001],
            ),
        ];
        for (program, input, output) in test_cases {
            let mut intcode = IntCode::from(program);
            assert_eq!(intcode.run(input), output);
        }
    }

    #[test]
    fn day9tests() {
        let test_cases = [(
            "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99",
            vec![
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
            ],
        )];

        for (program, output) in test_cases {
            let mut intcode = IntCode::from(program);
            assert_eq!(intcode.run(vec![]), output);
        }

        let mut intcode = IntCode::from("1102,34915192,34915192,7,4,7,99,0");
        let num = intcode.run(vec![])[0].to_string();
        assert_eq!(num.len(), 16);

        let mut intcode = IntCode::from("104,1125899906842624,99");
        let num = intcode.run(vec![])[0];
        assert_eq!(num, 1125899906842624);
    }
}
