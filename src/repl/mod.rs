use crate::assembler::program_parsers::program;
pub use crate::vm::VM;
use nom::types::CompleteStr;
use std;
use std::io::{BufRead, Write};
// use std::num::ParseIntError;
// use std::result::Result;

pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            vm: VM::new(),
            command_buffer: vec![],
        }
    }

    pub fn run<R, W>(&mut self, mut reader: R, mut writer: W)
    where
        R: BufRead,
        W: Write,
    {
        println!("Welcome to Iridium! Let's be productive!");
        let mut is_done = false;
        while !is_done {
            is_done = self.run_once(&mut reader, &mut writer);
        }
    }

    pub fn run_once<R, W>(&mut self, mut reader: R, mut writer: W) -> bool
    where
        R: BufRead,
        W: Write,
    {
        let mut buffer = String::new();
        write!(&mut writer, ">>> ").expect("Unable to write");
        writer.flush().unwrap();
        reader
            .read_line(&mut buffer)
            .expect("Unable to read line from user");
        let buffer = buffer.trim();

        self.command_buffer.push(buffer.to_string());

        match buffer {
            ".quit" => {
                writeln!(&mut writer, "Farewell! Have a great day!")
                    .expect("Unable to execute .quit");
                writer.flush().unwrap();
                true
            }
            ".history" => {
                for command in &self.command_buffer {
                    writeln!(&mut writer, "{}", command).expect("Unable to execute .history");
                    writer.flush().unwrap();
                }
                false
            }
            ".program" => {
                println!("Listing instructions currently in VM's program vector:");
                writer.flush().unwrap();
                for instruction in &self.vm.program {
                    writeln!(&mut writer, "{}", instruction).expect("Unable to execute .program");
                    writer.flush().unwrap();
                }
                writeln!(&mut writer, "End of Program Listing")
                    .expect("Unable to write ending message of  .program");
                writer.flush().unwrap();
                false
            }
            ".registers" => {
                writeln!(&mut writer, "Listing registers and all contents:")
                    .expect("Unable to execute .registers");
                writeln!(&mut writer, "{:#?}", self.vm.registers)
                    .expect("Unable to write registers");
                writeln!(&mut writer, "End of Program Listing")
                    .expect("Unable to write ending message of .registers");
                writer.flush().unwrap();
                false
            }
            _ => {
                let parsed_program = program(CompleteStr(buffer));
                if !parsed_program.is_ok() {
                    println!("Unable to parse input");
                    return true; // done
                }
                let (_, result) = parsed_program.unwrap();
                let bytecode = result.to_bytes();
                for byte in bytecode {
                    self.vm.add_byte(byte);
                }
                self.vm.run_once();
                false
            }
        }
    }

    // fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError> {
    //     let split = i.split(" ").collect::<Vec<&str>>();
    //     let mut results: Vec<u8> = vec![];
    //     for hex_string in split {
    //         let byte = u8::from_str_radix(&hex_string, 16);
    //         match byte {
    //             Ok(result) => {
    //                 results.push(result);
    //             }
    //             Err(e) => return Err(e),
    //         }
    //     }
    //     Ok(results)
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_parse_hex() {
    //     let mut test_repl = REPL::new();
    //     let hex_string = "00 01 03 E8";
    //     let hex_vec = test_repl.parse_hex(hex_string).unwrap();
    //     assert_eq!(hex_vec.len(), 4);
    //     assert_eq!(hex_vec[0], 0x00);
    //     assert_eq!(hex_vec[1], 0x01);
    //     assert_eq!(hex_vec[2], 0x03);
    //     assert_eq!(hex_vec[3], 0xE8);
    // }
    #[test]
    fn test_run_quit() {
        let input = b".quit";
        let mut output = Vec::new();
        let mut test_repl = REPL::new();
        test_repl.run_once(&input[..], &mut output);
        let output = String::from_utf8(output).expect("Not UTF-8");
        assert_eq!(">>> Farewell! Have a great day!\n", output);
    }

    #[test]
    fn test_run_history() {
        let input = b".history";
        let mut output = Vec::new();
        let mut test_repl = REPL::new();
        test_repl.command_buffer.push(".registers".to_string());
        test_repl.command_buffer.push(".program".to_string());
        test_repl.run_once(&input[..], &mut output);
        let output = String::from_utf8(output).expect("Not UTF-8");
        assert_eq!(">>> .registers\n.program\n.history\n", output);
    }

    #[test]
    fn test_run_program() {
        let input = b".program";
        let mut output = Vec::new();
        let mut test_repl = REPL::new();
        test_repl.vm.program = vec![0, 1, 2, 3];
        test_repl.run_once(&input[..], &mut output);
        let output = String::from_utf8(output).expect("Not UTF-8");
        assert_eq!(">>> 0\n1\n2\n3\nEnd of Program Listing\n", output);
    }

    #[test]
    fn test_run_registers() {
        let input = b".registers";
        let mut output = Vec::new();
        let mut test_repl = REPL::new();
        test_repl.vm.registers = [
            1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5,
            6, 7, 8,
        ];
        test_repl.run_once(&input[..], &mut output);
        let output = String::from_utf8(output).expect("Not UTF-8");
        assert_eq!(">>> Listing registers and all contents:\n[\n    1,\n    2,\n    3,\n    4,\n    5,\n    6,\n    7,\n    8,\n    1,\n    2,\n    3,\n    4,\n    5,\n    6,\n    7,\n    8,\n    1,\n    2,\n    3,\n    4,\n    5,\n    6,\n    7,\n    8,\n    1,\n    2,\n    3,\n    4,\n    5,\n    6,\n    7,\n    8,\n]\nEnd of Program Listing\n", output);
    }

    // #[test]
    // fn test_run_parse_hex() {
    //     let input = b"00 01 03 E8";
    //     let mut output = Vec::new();
    //     let mut test_repl = REPL::new();
    //     test_repl.run_once(&input[..], &mut output);
    //     assert_eq!(test_repl.vm.registers[1], 1000);
    // }

    // #[test]
    // fn test_run_parse_hex_error() {
    //     let input = b"kaboom";
    //     let mut output = Vec::new();
    //     let mut test_repl = REPL::new();
    //     test_repl.run_once(&input[..], &mut output);
    //     let output = String::from_utf8(output).expect("Not UTF-8");
    //     assert_eq!(
    //         ">>> Unable to decode hex string. Please enter 4 groups of 2 hex charracters.\n",
    //         output
    //     );
    // }
}
