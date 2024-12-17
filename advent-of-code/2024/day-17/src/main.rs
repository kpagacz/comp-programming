fn input() -> (usize, usize, usize, Vec<usize>) {
    (
        66245665,
        0,
        0,
        vec![2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5, 3, 0],
    )
    // (729, 0, 0, vec![0, 1, 5, 4, 3, 0])
}

struct Computer {
    a: usize,
    b: usize,
    c: usize,
    insts: Vec<usize>,
}

impl Computer {
    fn new(a: usize, b: usize, c: usize, insts: Vec<usize>) -> Self {
        Self { a, b, c, insts }
    }

    fn run(&mut self) -> Vec<usize> {
        let mut out = Vec::default();
        let mut it = 0;

        while it < self.insts.len() {
            let (opcode, operand) = (self.insts[it], self.insts[it + 1]);

            match opcode {
                0 => {
                    self.a /= 2usize.pow(self.combo_operand(operand) as u32);
                    it += 2;
                }
                1 => {
                    self.b ^= operand;
                    it += 2;
                }
                2 => {
                    self.b = self.combo_operand(operand) % 8;
                    it += 2;
                }
                3 => {
                    if self.a != 0 {
                        it = operand;
                    } else {
                        it += 2;
                    }
                }
                4 => {
                    self.b ^= self.c;
                    it += 2;
                }
                5 => {
                    out.push(self.combo_operand(operand) % 8);
                    it += 2;
                }
                6 => {
                    self.b = self.a / 2usize.pow(self.combo_operand(operand) as u32);
                    it += 2;
                }
                7 => {
                    self.c = self.a / 2usize.pow(self.combo_operand(operand) as u32);
                    it += 2;
                }
                _ => unreachable!(),
            }
        }
        out
    }

    fn combo_operand(&self, operand: usize) -> usize {
        match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => panic!("Not allowed"),
            _ => unreachable!(),
        }
    }
}

fn part1() -> String {
    let (a, b, c, insts) = input();
    let mut comp = Computer::new(a, b, c, insts);

    let out = comp.run();
    let mut res = String::default();
    for num in out {
        res.push_str(&num.to_string());
    }

    res
}

// What is happening here?
// vec![2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5, 3, 0],
// 1. B = A % 8 (last 3 bits of A are written to B)
// 2. Bits of B are XORed - essentially flipped
// 3. C = A / 2 ^ B
// 4. Bits of B are flipped again
// 5. B = B ^ C
// 6. A = A / 8 (essentially bit shifted to right by 3)
// 7. Output B
// 8. If A == 0 { break } else { goto 1 }
//
// This means that the output is equal to the XOR of
// two values - the last 3 bits of A and the last
// 3 bits of A bit shifted by the flip of the last
// 3 bits of A.
// So given:
// T = last 3 bits of A
// output = A ^ (A >> ~T)
//
// The plan:
// 1. Precalculate possible values of T for a given output
// 2. Start from the end of the instructions because
// the end corresponds to the beginning of A
// 3. Figure out possible values of bits in the current 3 bit
// window.
// 4. Remember the bits of A
// 5. Consider next 3 bits of A
// 6. If there is a clash - backtrack
// 7. Do the above until no more digits in the output

fn backtrack(output: &[usize], a: usize) -> Option<usize> {
    if output.is_empty() {
        return Some(a);
    }

    let last_digit = *output.last().unwrap();

    for t in 0..=7 {
        let candidate_a = (a << 3) | t;
        let bitshifted = candidate_a >> (t ^ 7);

        if (candidate_a ^ bitshifted) % 8 == last_digit {
            if let Some(final_a) = backtrack(&output[..output.len() - 1], candidate_a) {
                return Some(final_a);
            }
        }
    }

    None
}

fn part2() -> usize {
    let insts = vec![2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5, 3, 0];
    let new_a = backtrack(&insts, 0).unwrap();
    let mut comp = Computer::new(new_a, 0, 0, insts.clone());
    let out = comp.run();
    println!("Got {out:?}");
    println!("Expected {insts:?}");
    new_a
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
