use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    ADV = 0, // Div of A and 2^Combo Op
    BXL = 1, //XOR of B and literal
    BST = 2, // Combo Op %8 and store to B
    JNZ = 3, // If A!=0, jump to literal operand
    BXC = 4, // Bitwise XOR of B and C, stores into B, ignores operand
    OUT = 5, // calculates the value of its combo operand % 8 and outputs it
    BDV = 6, // Div of B and 2^Combo Op
    CDV = 7, // Div of C and 2^Combo Op
}

impl From<u64> for Instruction {
    fn from(item: u64) -> Self {
        match item {
            0 => Instruction::ADV,
            1 => Instruction::BXL,
            2 => Instruction::BST,
            3 => Instruction::JNZ,
            4 => Instruction::BXC,
            5 => Instruction::OUT,
            6 => Instruction::BDV,
            7 => Instruction::CDV,
            _ => panic!("Invalid instruction `{}`", item),
        }
    }
}
#[derive(Default, Clone)]
struct RegisterFile {
    registers: [u64; 3],
}
#[derive(Clone)]
struct MiniPC {
    register_file: RegisterFile,
    instruction_pointer: usize,
    instructions: Vec<u64>,
    output: Vec<u64>,
}
impl MiniPC {
    pub fn from_file(path: &str) -> Self {
        let mut register_file = RegisterFile::default();
        let file_contents = std::fs::read_to_string(path).unwrap();
        let mut instructions = Vec::new();
        for line in file_contents.lines() {
            if line.is_empty() {
                continue;
            }
            //SPlit by colon to get name / value
            let parts = line.split(":").collect::<Vec<_>>();
            match parts[0] {
                "Register A" => register_file.registers[0] = parts[1].trim().parse().unwrap(),
                "Register B" => register_file.registers[1] = parts[1].trim().parse().unwrap(),
                "Register C" => register_file.registers[2] = parts[1].trim().parse().unwrap(),
                "Program" => {
                    instructions = parts[1]
                        .split(",")
                        .map(|x| x.trim().parse().unwrap())
                        .collect();
                }
                _ => panic!("Unknown line {}", parts[0]),
            }
        }
        let instructions_len = instructions.len();
        Self {
            register_file,
            instruction_pointer: 0,
            instructions,
            output: Vec::with_capacity(instructions_len),
        }
    }
    pub fn run_next_instruction(&mut self) {
        let instruction = Instruction::from(self.instructions[self.instruction_pointer]);
        let operand = self.instructions[self.instruction_pointer + 1];
        // println!(
        //     "Running instruction {}:{:?}, op {}",
        //     self.instruction_pointer, instruction, operand
        // );
        match instruction {
            Instruction::BXL => {
                // BXL: XOR of B and literal
                self.register_file.registers[1] ^= operand;
            }
            Instruction::BST => {
                // BST: Combo Op %8 and store to B
                self.register_file.registers[1] = self.get_combo_op(operand) % 8;
            }
            Instruction::JNZ => {
                // JNZ: If A!=0, jump to literal operand
                if self.register_file.registers[0] != 0 {
                    self.instruction_pointer = operand as usize;
                    return; // Do not increment program counter
                }
            }
            Instruction::BXC => {
                // BXC: Bitwise XOR of B and C, stores into B, ignores operand
                self.register_file.registers[1] ^= self.register_file.registers[2];
            }
            Instruction::OUT => {
                // OUT: calculates the value of its combo operand % 8 and outputs it
                let combo_op = self.get_combo_op(operand);
                self.output.push(combo_op % 8);
            }
            Instruction::ADV => {
                // ADV: Div of A and 2^Combo Op
                self.register_file.registers[0] =
                    self.register_file.registers[0] / 2_u64.pow(self.get_combo_op(operand) as u32);
            }
            Instruction::BDV => {
                // BDV: Div of B and 2^Combo Op
                self.register_file.registers[1] =
                    self.register_file.registers[0] / 2_u64.pow(self.get_combo_op(operand) as u32);
            }
            Instruction::CDV => {
                // CDV: Div of C and 2^Combo Op
                self.register_file.registers[2] =
                    self.register_file.registers[0] / 2_u64.pow(self.get_combo_op(operand) as u32);
            }
        }
        self.instruction_pointer += 2; // Next step
    }

    fn get_combo_op(&self, op: u64) -> u64 {
        match op {
            0 => 0,                               // Literally 0
            1 => 1,                               // Literally 1
            2 => 2,                               // Literally 2
            3 => 3,                               // Literally 3
            4 => self.register_file.registers[0], // Reg A
            5 => self.register_file.registers[1], // Reg B
            6 => self.register_file.registers[2], // Reg C
            _ => unreachable!("Bad combo op"),
        }
    }
    pub fn is_halted(&self) -> bool {
        self.instruction_pointer >= self.instructions.len()
    }
}
fn main() {
    let t_a = std::thread::spawn(|| {
        println!("PART A: {}", part_a("input.txt"));
    });
    let t_b = std::thread::spawn(|| {
        println!("PART B: {}", part_b("input.txt"));
    });
    t_a.join().unwrap();
    t_b.join().unwrap();
}
fn part_a(path: &str) -> String {
    let mut machine = MiniPC::from_file(path);
    while !machine.is_halted() {
        machine.run_next_instruction();
    }
    format!("{}", machine.output.iter().format(","))
}
fn part_b(path: &str) -> u64 {
    // Hand rolling the hashing unwrap
    // Feels a lot like we are reverseing a hash to find the correlation from the input (A reg) to output
    /*
    // 1: 2,4
    // 2: 1,5
    // 3: 7,5
    // 4: 1,6
    // 5: 4,2
    // 6: 5,5
    // 7: 0,3
    // 8: 3,0
       1: B = A%8
       2: B = B^5
       3: C = A/2^B
       4: B = 6^B
       5: B = B^C
       6: OUT B%8
       7: A = A/2^3
       8: Jump to start if A!=0

       // First notes, fairly linear; length is going to scale by A/2^3 seach step
       // Therefore, A = 2^3^8 in size, so approx 2^24
       // A has to be between 1<<(3*8) and 111<<(8*8)

       // Simplied expression
       B = (A%8)^5
       C = A>>B
       B = (B^6)^C
       OUT B%8
       A >>= 3

       // We Loop over A, and we effectively process 3 bits at a time

       B = ((A%8)^5)
       OUT = ((6^B)^(A>>B))%8
       OUT = ((6^((A%8)^5))^(A>>((A%8)^5)))%8

    */
    let machine = MiniPC::from_file(path);

    fn find_matching_bits_recursively(
        instructions: &[u64],
        postion: usize,
        accumulator: u64,
    ) -> u64 {
        // After looking at the code above, we process the accumulator in 3 bit chunks
        for possible_input in 0..=0b111u64 {
            let temporary_accumulator = accumulator << 3 | possible_input;

            //A and B are seeded on each run, so no state is carried over
            let mut B = (temporary_accumulator % 8) ^ 5;
            let C = temporary_accumulator.wrapping_shr(B as u32);
            B = (B ^ 6) ^ C;
            let output = B % 8;
            println!("Step {postion}, A B C {temporary_accumulator} {B} {C} -> {output}");
            if output == instructions[postion] {
                let out = if postion == 0 {
                    // Have solved last bit slice
                    temporary_accumulator
                } else {
                    find_matching_bits_recursively(instructions, postion - 1, temporary_accumulator)
                };
                if out != 0 {
                    return out;
                }
            }
        }
        //We have failed to match the pattern??
        unreachable!(
            "Could not match {} at step {postion}",
            instructions[postion]
        );
    }
    find_matching_bits_recursively(&machine.instructions, machine.instructions.len() - 1, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_demo() {
        let results = part_a("test.txt");
        assert_eq!(results, "4,6,3,5,6,3,5,2,1,0");
    }
    #[test]
    fn test_part_a_real() {
        let results = part_a("input.txt");
        assert_eq!(results, "2,7,6,5,6,0,2,3,1");
    }
    // #[test]
    // fn test_part_b_demo() {
    //     let results = part_b("test2.txt");
    //     assert_eq!(results, 117440);
    // }
    #[test]
    fn test_part_b_real() {
        let results = part_b("input.txt");
        assert_eq!(results, 0);
    }
}
