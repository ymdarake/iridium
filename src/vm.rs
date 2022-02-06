use crate::instruction::Opcode;

pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    remainder: u32,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            pc: 0,
            remainder: 0,
        }
    }

    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return false;
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
                self.remainder = (register1 % register2) as u32;
            }
            Opcode::HLT => {
                println!("HLT encoutered");
                return false;
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
            Opcode::IGL => {
                println!("Illegal instruction encountered");
                return false;
            }
        }
        true
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return opcode;
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        return result;
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
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
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }
}
