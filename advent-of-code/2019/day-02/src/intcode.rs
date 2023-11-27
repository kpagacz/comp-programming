type IntUnit = i32;
pub struct IntCode {
    pub memory: Vec<IntUnit>,
}

const END_OP: IntUnit = 99;
const ADD_OP: IntUnit = 1;
const MULTIPLY_OP: IntUnit = 2;
impl IntCode {
    pub fn from(input: &str) -> Self {
        Self {
            memory: input
                .trim()
                .split(",")
                .map(|code| {
                    code.parse::<IntUnit>()
                        .or_else(|_| {
                            println!("Could not parse {code}");
                            Err(())
                        })
                        .expect("IntCode program always has ints only")
                })
                .collect(),
        }
    }

    pub fn run(&mut self) {
        let mut it = 0;
        while it < self.memory.len() && self.memory[it] != END_OP {
            let (operand1, operand2, target) = (
                self.memory[it + 1] as usize,
                self.memory[it + 2] as usize,
                self.memory[it + 3] as usize,
            );
            match self.memory[it] {
                ADD_OP => self.memory[target] = self.memory[operand1] + self.memory[operand2],
                MULTIPLY_OP => self.memory[target] = self.memory[operand1] * self.memory[operand2],
                _ => unreachable!(),
            }
            it += 4;
        }
    }
}
