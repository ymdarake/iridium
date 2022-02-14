use super::opcode_parsers::opcode;
use super::operand_parsers::integer_operand;
use super::register_parsers::register;
use super::Token;
use nom::multispace;
use nom::types::CompleteStr;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    opcode: Token,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results = vec![];
        match self.opcode {
            Token::Op { code } => {
                results.push(code as u8);
            }
            _ => {
                println!("Non-opcode found in opcode field");
                std::process::exit(1);
            }
        };

        for operand in &[&self.operand1, &self.operand2, &self.operand3] {
            if let Some(token) = operand {
                AssemblerInstruction::extract_operand(token, &mut results)
            }
        }

        results
    }

    fn extract_operand(t: &Token, results: &mut Vec<u8>) {
        match t {
            Token::Register { reg_num } => {
                results.push(*reg_num);
            }
            Token::IntegerOperand { value } => {
                let converted = *value as u16;
                let byte1 = converted;
                let byte2 = converted >> 8;
                results.push(byte2 as u8);
                results.push(byte1 as u8);
            }
            _ => {
                println!("Opcode found in operand field");
                std::process::exit(1);
            }
        }
    }
}

// TODO: rename all macro-defined parses to represent their meanings
// Handles instructions of the following form:
// LOAD $0 #100
named!(instruction_one<CompleteStr, AssemblerInstruction>,
    do_parse!(
        o: opcode >>
        r: register >>
        i: integer_operand >>
        (
            AssemblerInstruction{
                opcode: o,
                operand1: Some(r),
                operand2: Some(i),
                operand3: None
            }
        )
    )
);

// typo in the blog post part 10. this is 'the' instruction_two parser
named!(instruction_two<CompleteStr, AssemblerInstruction>,
    do_parse!(
        o: opcode >>
        opt!(multispace) >>
        (
            AssemblerInstruction{
                opcode: o,
                operand1: None,
                operand2: None,
                operand3: None
            }
        )
    )
);

// homework form the blog post part 10
// parses '<opcode> <register> <register> <register>'
named!(instruction_three<CompleteStr, AssemblerInstruction>,
    do_parse!(
        o: opcode >>
        r1: register >>
        r2: register >>
        r3: register >>
        (
            AssemblerInstruction{
                opcode: o,
                operand1: Some(r1),
                operand2: Some(r2),
                operand3: Some(r3)
            }
        )
    )
);

named!(pub instruction<CompleteStr, AssemblerInstruction>,
    do_parse!(
        ins: alt!(
            instruction_one |
            instruction_two |
            instruction_three
        ) >>
        (
            ins
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Opcode;

    #[test]
    fn test_parse_instruction_form_one() {
        let result = instruction_one(CompleteStr("load $0 #100\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::LOAD },
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::IntegerOperand { value: 100 }),
                    operand3: None
                }
            ))
        );
    }

    #[test]
    fn test_parse_instruction_form_two() {
        let result = instruction_two(CompleteStr("hlt\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::HLT },
                    operand1: None,
                    operand2: None,
                    operand3: None
                }
            ))
        );
    }

    #[test]
    fn test_parse_instruction_form_three() {
        let result = instruction_three(CompleteStr("add $0 $1 $2\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::ADD },
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::Register { reg_num: 1 }),
                    operand3: Some(Token::Register { reg_num: 2 }),
                }
            ))
        );
    }
}
