use crate::{assembler, instruction::Opcode};

pub struct VM {
    /// Array that simulates having hardware registers
    pub registers: [i32; 32],
    /// The bytecode of the program being run
    pub program: Vec<u8>,
    heap: Vec<u8>,
    /// Program counter that tracks which byte is being executed
    pc: usize,
    /// Contains the remainder of modulo division ops
    remainder: usize,
    /// Contains the result of the last comparison operation
    equal_flag: bool,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            heap: vec![],
            pc: 0,
            remainder: 0,
            equal_flag: false,
        }
    }

    pub fn run(&mut self) {
        if !self.verify_header() {
            std::process::exit(1);
        }
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    pub fn add_byte(&mut self, byte: u8) {
        self.program.push(byte);
    }

    pub fn add_bytes(&mut self, mut bytes: Vec<u8>) {
        self.program.append(&mut bytes);
    }

    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return true;
        }
        match self.decode_opcode() {
            Opcode::LOAD => {
                let target_register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u16;
                self.registers[target_register] = number as i32;
            }
            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
            }
            Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 - register2;
            }
            Opcode::MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 * register2;
            }
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as usize;
            }
            Opcode::HLT => {
                println!("HLT encoutered");
                return true;
            }
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
            }
            Opcode::JMPF => {
                let value = self.registers[self.next_8_bits() as usize] as usize;
                self.pc += value;
            }
            Opcode::JMPB => {
                let value = self.registers[self.next_8_bits() as usize] as usize;
                self.pc -= value;
            }
            Opcode::EQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 == register2;
                self.next_8_bits();
            }
            Opcode::NEQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 != register2;
                self.next_8_bits();
            }
            Opcode::GT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 > register2;
                self.next_8_bits();
            }
            Opcode::GTE => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 >= register2;
                self.next_8_bits();
            }
            Opcode::LT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 < register2;
                self.next_8_bits();
            }
            Opcode::LTE => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 <= register2;
                self.next_8_bits();
            }
            Opcode::JMPE => {
                let register = self.next_8_bits() as usize;
                let target = self.registers[register];
                if self.equal_flag {
                    self.pc = target as usize;
                } else {
                    // TODO: fix the bits
                }
            }
            Opcode::NOP => {
                self.next_8_bits();
                self.next_8_bits();
                self.next_8_bits();
            }
            Opcode::ALOC => {
                let register = self.next_8_bits() as usize;
                let bytes = self.registers[register];
                let new_end = self.heap.len() as i32 + bytes;
                self.heap.resize(new_end as usize, 0);
                // TODO: 1: check: Don't we need to read next 16 bits here?
            }
            Opcode::INC => {
                let register_number = self.next_8_bits() as usize;
                self.registers[register_number] += 1;
                self.next_8_bits();
                self.next_8_bits();
            }
            Opcode::DEC => {
                let register_number = self.next_8_bits() as usize;
                self.registers[register_number] -= 1;
                self.next_8_bits();
                self.next_8_bits();
            }
            Opcode::IGL => {
                println!("Illegal instruction encountered");
                return true;
            }
        }
        false
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        result
    }

    fn verify_header(&self) -> bool {
        if self.program[0..4] != assembler::PIE_HEADER_PREFIX {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::assembler::{PIE_HEADER_LENGTH, PIE_HEADER_PREFIX};

    use super::*;

    fn prepend_header(mut b: Vec<u8>) -> Vec<u8> {
        let mut prepension = vec![];
        for byte in PIE_HEADER_PREFIX.into_iter() {
            prepension.push(byte.clone());
        }
        while prepension.len() <= PIE_HEADER_LENGTH {
            prepension.push(0);
        }
        prepension.append(&mut b);
        prepension
    }

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_add_byte() {
        let mut test_vm = VM::new();
        assert_eq!(test_vm.program.len(), 0);
        test_vm.add_byte(10);
        assert_eq!(test_vm.program.len(), 1);
        test_vm.add_byte(0x0C);
        assert_eq!(test_vm.program.len(), 2);
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![5, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_load() {
        let mut test_vm = VM::new();
        test_vm.program = vec![0, 0, 1 /* 2^8 = 256*/, 244];
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 500 /* 256 + 244 */);
    }

    #[test]
    fn test_opcode_add() {
        let mut test_vm = VM::new();
        test_vm.program = vec![
            0,   /* LOAD */
            0,   /* dest: register 0 */
            1,   /* 2^8*1 = 256 */
            12,  /* 12 */
            0,   /* LOAD */
            1,   /* dest: register 1 */
            0,   /* 0 */
            255, /* 255 */
            1,   /* ADD */
            0,   /* register 0: 256 + 12 */
            1,   /* and register 1: 255 */
            2,   /* store in register 2*/
        ];
        test_vm.run_once(); // LOAD
        assert_eq!(test_vm.registers[0], 268);
        test_vm.run_once(); // LOAD
        assert_eq!(test_vm.registers[1], 255);
        test_vm.run_once(); // ADD
        assert_eq!(test_vm.registers[2], 523);
    }

    #[test]
    fn test_opcode_sub() {
        let mut test_vm = VM::new();
        test_vm.program = vec![
            0,   /* LOAD */
            0,   /* dest: register 0 */
            1,   /* 2^8*1 = 256 */
            12,  /* 12 */
            0,   /* LOAD */
            1,   /* dest: register 1 */
            0,   /* 0 */
            255, /* 255 */
            2,   /* SUB */
            0,   /* register 0: 256 + 12 */
            1,   /* and register 1: 255 */
            2,   /* store in register 2*/
        ];
        test_vm.run_once(); // LOAD
        assert_eq!(test_vm.registers[0], 268);
        test_vm.run_once(); // LOAD
        assert_eq!(test_vm.registers[1], 255);
        test_vm.run_once(); // SUB
        assert_eq!(test_vm.registers[2], 13);
    }

    #[test]
    fn test_opcode_mul() {
        let mut test_vm = VM::new();
        test_vm.program = vec![
            0,   /* LOAD */
            0,   /* dest: register 0 */
            1,   /* 2^8*1 = 256 */
            12,  /* 12 */
            0,   /* LOAD */
            1,   /* dest: register 1 */
            0,   /* 0 */
            255, /* 255 */
            3,   /* MUL */
            0,   /* register 0: 256 + 12 */
            1,   /* and register 1: 255 */
            2,   /* store in register 2*/
        ];
        test_vm.program = prepend_header(test_vm.program);
        test_vm.pc += 65;
        test_vm.run_once(); // LOAD
        assert_eq!(test_vm.registers[0], 268);
        test_vm.run_once(); // LOAD
        assert_eq!(test_vm.registers[1], 255);
        test_vm.run_once(); // MUL
        assert_eq!(test_vm.registers[2], 68340);
    }

    #[test]
    fn test_opcode_div() {
        let mut test_vm = VM::new();
        test_vm.program = vec![
            0,   /* LOAD */
            0,   /* dest: register 0 */
            1,   /* 2^8*1 = 256 */
            12,  /* 12 */
            0,   /* LOAD */
            1,   /* dest: register 1 */
            0,   /* 0 */
            255, /* 255 */
            4,   /* DIV */
            0,   /* register 0: 256 + 12 */
            1,   /* and register 1: 255 */
            2,   /* store in register 2*/
        ];
        test_vm.run_once(); // LOAD
        assert_eq!(test_vm.registers[0], 268);
        assert_eq!(test_vm.remainder, 0);
        test_vm.run_once(); // LOAD
        assert_eq!(test_vm.registers[1], 255);
        assert_eq!(test_vm.remainder, 0);
        test_vm.run_once(); // DIV
        assert_eq!(test_vm.registers[2], 1);
        assert_eq!(test_vm.remainder, 13);
    }

    #[test]
    fn test_opcode_jmp() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 1;
        test_vm.program = vec![6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_jmpf() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.program = vec![
            7, /* JMPF */
            0, /* increment pc by the number the register0 stores (+2) */
            0, /* pad */
            0, /* pad */
        ];
        test_vm.run_once();
        assert_eq!(
            test_vm.pc,
            4 /* 1. Read JMPF, 2. Read 0, then + 2 = 4 */
        );
    }

    #[test]
    fn test_opcode_jmpb() {
        let mut test_vm = VM::new();
        test_vm.registers[1] = 6;
        test_vm.program = vec![0, 0, 0, 10, 8, 1, 0, 0];
        test_vm.run_once(); // LOAD: pc += 4
        test_vm.run_once(); // Read JMPB and target (pc += 2), then JMPB to register1: 6 (pc -= 6)
        assert_eq!(test_vm.pc, 0);
    }

    #[test]
    fn test_opcode_eq() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![9 /* EQ */, 0, 1, 0, 9 /* EQ */, 0, 1, 0];
        test_vm.run_once(); // 10 == 10
        assert_eq!(test_vm.equal_flag, 10 == 10);
        test_vm.registers[1] = 20;
        test_vm.run_once(); // 10 == 20
        assert_eq!(test_vm.equal_flag, 10 == 20);
    }

    #[test]
    fn test_opcode_neq() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![10 /* NEQ */, 0, 1, 0, 10 /* NEQ */, 0, 1, 0];
        test_vm.run_once(); // 10 != 10
        assert_eq!(test_vm.equal_flag, 10 != 10);
        test_vm.registers[1] = 20;
        test_vm.run_once(); // 10 != 20
        assert_eq!(test_vm.equal_flag, 10 != 20);
    }

    #[test]
    fn test_opcode_gt() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![11 /* GT */, 0, 1, 0, 11 /* GT */, 0, 1, 0];
        test_vm.run_once(); // 10 > 10
        assert_eq!(test_vm.equal_flag, 10 > 10);
        test_vm.registers[0] = 99;
        test_vm.run_once(); // 99 > 10
        assert_eq!(test_vm.equal_flag, 99 > 10);
    }

    #[test]
    fn test_opcode_gte() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![12, 0, 1, 0, 12, 0, 1, 0, 12, 0, 1, 0];
        test_vm.run_once(); // 10 >= 10
        assert_eq!(test_vm.equal_flag, 10 >= 10);
        test_vm.registers[0] = 99;
        test_vm.run_once(); // 99 >= 10
        assert_eq!(test_vm.equal_flag, 99 >= 10);
        test_vm.registers[0] = 3;
        test_vm.run_once(); // 3 >= 10
        assert_eq!(test_vm.equal_flag, 3 >= 10);
    }

    #[test]
    fn test_opcode_lt() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![13 /* LT */, 0, 1, 0, 13 /* LT */, 0, 1, 0];
        test_vm.run_once(); // 10 < 10
        assert_eq!(test_vm.equal_flag, 10 < 10);
        test_vm.registers[0] = 3;
        test_vm.run_once(); // 3 < 10
        assert_eq!(test_vm.equal_flag, 3 < 10);
    }

    #[test]
    fn test_opcode_lte() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![14, 0, 1, 0, 14, 0, 1, 0, 14, 0, 1, 0];
        test_vm.run_once(); // 10 <= 10
        assert_eq!(test_vm.equal_flag, 10 <= 10);
        test_vm.registers[1] = 99;
        test_vm.run_once(); // 10 <= 99
        assert_eq!(test_vm.equal_flag, 10 <= 99);
        test_vm.registers[1] = 3;
        test_vm.run_once(); // 10 <= 3
        assert_eq!(test_vm.equal_flag, 10 <= 3);
    }

    #[test]
    fn test_opcode_jmpe_equal() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 7;
        test_vm.equal_flag = true;
        test_vm.program = vec![15, 0, 0, 0, 17, 0, 0, 17, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 7);
    }
    #[test]
    fn test_opcode_jmpe_not_equal() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 7;
        test_vm.equal_flag = false;
        test_vm.program = vec![15, 0, 0, 0, 17, 0, 0, 17, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 2);
        // TODO: fix the bits assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_opcode_aloc() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 1024;
        test_vm.program = vec![17, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.heap.len(), 1024);
        // TODO: 2: ref TODO 1
    }

    #[test]
    fn test_opcode_inc() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 4;
        test_vm.program = vec![18, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 5);
    }

    #[test]
    fn test_opcode_dec() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 4;
        test_vm.program = vec![19, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 3);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }
}
