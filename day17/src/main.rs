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

impl From<i64> for Instruction {
    fn from(item: i64) -> Self {
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
#[derive(Default)]
struct RegisterFile {
    registers: [i64; 3],
}
struct MiniPC {
    register_file: RegisterFile,
    instruction_pointer: usize,
    instructions: Vec<i64>,
    output: String,
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
            println!("Parse > `{}` -> `{}`", parts[0], parts[1]);
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

        Self {
            register_file,
            instruction_pointer: 0,
            instructions,
            output: "".to_owned(),
        }
    }
    pub fn run_next_instruction(&mut self) {
        let instruction = Instruction::from(self.instructions[self.instruction_pointer]);
        let operand = self.instructions[self.instruction_pointer + 1];
        println!(
            "Running instruction {}:{:?}, op {}",
            self.instruction_pointer, instruction, operand
        );
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

                self.output += &format!("{},", combo_op % 8);
            }
            Instruction::ADV => {
                // ADV: Div of A and 2^Combo Op
                self.register_file.registers[0] =
                    self.register_file.registers[0] / 2_i64.pow(self.get_combo_op(operand) as u32);
            }
            Instruction::BDV => {
                // BDV: Div of B and 2^Combo Op
                self.register_file.registers[1] =
                    self.register_file.registers[0] / 2_i64.pow(self.get_combo_op(operand) as u32);
            }
            Instruction::CDV => {
                // CDV: Div of C and 2^Combo Op
                self.register_file.registers[2] =
                    self.register_file.registers[0] / 2_i64.pow(self.get_combo_op(operand) as u32);
            }
        }
        self.instruction_pointer += 2; // Next step
    }

    fn get_combo_op(&self, op: i64) -> i64 {
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
    machine.output[0..(machine.output.len() - 1)].to_owned()
}
fn part_b(path: &str) -> i64 {
    0
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
    #[test]
    fn test_part_b_demo() {
        let results = part_b("test.txt");
        assert_eq!(results, 0);
    }
    #[test]
    fn test_part_b_real() {
        let results = part_b("input.txt");
        assert_eq!(results, 0);
    }
}
