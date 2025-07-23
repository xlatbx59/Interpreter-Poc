//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/Interpreter-Poc
use crate::lexapro::*;
use crate::error::*;
use crate::mnemonics::*;

#[derive(Debug)]
pub enum Operand{
    Lab(String), Imm(u64)
}

#[derive(Debug)]
pub struct Instruction{
    pub label: Option<String>,
    pub mnemonic: IaMnemonic,
    pub operand: Option<Operand>
}

pub struct Parser{
    tokens: Vec<Token>,
    label: Option<Token>,
    index: usize,
    line: usize,
}

impl Parser{
    fn verify_instruction(&mut self) -> Result<Instruction, IaGrammarError>{
        let operand: Option<Operand>;
        let mnemonic = match self.tokens[self.index].token.as_str(){
            IA_ADD => IaMnemonic::IaAdd, IA_SUB => IaMnemonic::IaSub, IA_SUBI => IaMnemonic::IaSubi,
            IA_AND => IaMnemonic::IaAnd, IA_XOR => IaMnemonic::IaXor, IA_LDI => IaMnemonic::IaLdi,
            IA_OR => IaMnemonic::IaOr, IA_BL => IaMnemonic::IaBl, IA_SHL => IaMnemonic::IaShl,
            IA_SHR => IaMnemonic::IaShr, IA_STI => IaMnemonic::IaSti, IA_PUSH => IaMnemonic::IaPush,
            IA_POP => IaMnemonic::IaPop, IA_DUP => IaMnemonic::IaDup, IA_ADDI => IaMnemonic::IaAddi,
            IA_BNZ => IaMnemonic::IaBnz, IA_BZ => IaMnemonic::IaBz, IA_RET => IaMnemonic::IaRet,
            IA_NOT => IaMnemonic::IaNot, IA_MUL => IaMnemonic::IaMul, IA_MULI => IaMnemonic::IaMuli,
            IA_NOP => IaMnemonic::IaNop, IA_SYSCALL => IaMnemonic::IaSyscall,
            _ => panic!(),
        };
        self.index += 1;

        //Checks if the instruction has operands
        if MNEMONIC_WITH_OPERAND.contains(&mnemonic){
            //Returns an error when there's no operands
            if self.tokens.len() - self.index < 1{
                return Err(throw_endfile_error(self.tokens[self.index - 1].clone(), self.line))
            }
            operand = if MNEMONIC_WITH_LABEL.contains(&mnemonic) && self.tokens[self.index].token_type == TokenType::Label{
                Some(Operand::Lab(self.tokens[self.index].token.clone()))
            }
            else if MNEMONIC_WITH_IMM.contains(&mnemonic){
                if self.tokens[self.index].token_type == TokenType::DecLiteral{
                    Some(Operand::Imm(self.tokens[self.index].token.parse::<u64>().unwrap()))
                }
                else if self.tokens[self.index].token_type == TokenType::HexLiteral{
                    Some(Operand::Imm(u64::from_str_radix(&self.tokens[self.index].token.clone().as_str(), 16).unwrap()))
                }else {
                    return Err(throw_syntax_error(self.tokens[self.index].clone(),self.tokens[self.index - 1].clone(), self.line))
                }
            }
            else{
                return Err(throw_syntax_error(self.tokens[self.index].clone(),self.tokens[self.index - 1].clone(), self.line))
            };
            self.index += 1;
        }
        else{
            operand = None;
        }

        //Only one instruction by line
        if self.tokens.len() != self.index && self.tokens[self.index].token_type != TokenType::EndOfLine {
            return Err(throw_syntax_error(self.tokens[self.index].clone(),self.tokens[self.index - 1].clone(), self.line))
        }

        Ok(Instruction{
            mnemonic, 
            label: match &self.label{
                None => None,
                Some(lab) => Some(lab.token.clone()),
            },
            operand: match operand{
                None => None,
                Some(op) => Some(op),
            },
        })
    }

    fn verify_label(&mut self) -> Result<Option<Instruction>, IaGrammarError>{
        if self.tokens.len() - self.index < 2{
            return Err(throw_endfile_error(self.tokens[self.index].clone(), self.line))
        }
        self.label = Some(self.tokens[self.index].clone());
        self.index += 1;
        return match self.tokens[self.index].token_type{
        TokenType::Colon => {
                if self.tokens.len() - 1< 3{
                    return Err(throw_endfile_error(self.tokens[self.index].clone(), self.line))
                }
                self.index += 1;
                match self.tokens[self.index].token_type{
                TokenType::EndOfLine => {
                        self.line += 1;
                        Ok(None)
                    },  //If the line is only the label, it has to have a colon and nothing else
                TokenType::Instruction => match self.verify_instruction(){
                    Ok(i) => {
                        self.label = None;
                        Ok(Some(i))
                    },
                    Err(e) => Err(e),
                }, //except an instruction
                    _ => Err(throw_syntax_error(self.tokens[self.index - 1].clone(),self.tokens[self.index].clone(), self.line)),
                }
            },
        TokenType::Instruction => match self.verify_instruction(){ //The label doesn't need a colon when follow by an instruction
                    Ok(i) => {
                        self.label = None;
                        Ok(Some(i))
                    },
                    Err(e) => Err(e),
        },
            _ => Err(throw_syntax_error(self.tokens[self.index - 1].clone(),self.tokens[self.index].clone(), self.line)),
        };
    }

    pub fn new_parser(tokens: Vec<Token>) -> Parser{
        Parser{tokens: tokens, line: 1, index: 0, label: None}
    }
    pub fn parsing(&mut self) -> Result<Option<Instruction>, IaGrammarError>{
        while self.index < self.tokens.len(){
            match self.tokens[self.index].token_type{
                TokenType::EndOfLine => {
                    self.index +=1;
                    self.line+=1;
                },
                TokenType::Label => {
                    match  self.verify_label(){
                        Err(e) =>return Err(e),
                        Ok(value) => {if let Some(i) = value{return Ok(Some(i))}}
                    }
                },
                TokenType::Instruction => {
                    return match self.verify_instruction(){
                        Err(e) => Err(e),
                        Ok(i) => {
                            self.label = None;
                            Ok(Some(i))
                        },
                    }
                }
                _ => return Err(throw_syntax_error(self.tokens[self.index].clone(), self.tokens[self.index - 1].clone(), self.line)),
            };
        }
        return Ok(None)
    }
}
