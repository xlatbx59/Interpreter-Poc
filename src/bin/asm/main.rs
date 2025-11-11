//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/Interpreter-Poc

pub mod mnemonics;
pub mod error;
pub mod lexapro;
pub mod parser;
mod symbol_entry;

use std::env;
use std::io::Read;
use std::{fs::File, io::Write};
use std::cmp::Ordering;
use crate::{parser::Operand, symbol_entry::SymbolEntry};

const INSTRUCTION_SIZE: usize = 9;

#[derive(Debug)]
///How an instruction is represented inside of memory, very simple/naive way for the moment
struct ByteCode{
    opcode: u8,
    imm: u64
}

impl ByteCode{
    pub fn to_bytes(&self) -> [u8; 9]{
        let op: [u8; 8] = self.imm.to_le_bytes();
        [self.opcode, op[0], op[1], op[2], op[3], op[4], op[5], op[6], op[7]]
    }
}

//Lexapro == lexer
fn main(){
    let mut pc: usize = 0;
    let mut sourcefile: File;
    let mut parse: parser::Parser;
    let mut code: Vec<ByteCode> = Vec::new();
    let mut ref_table: Vec<symbol_entry::SymbolEntry> = Vec::new();
    let mut symbol_table: Vec<symbol_entry::SymbolEntry> = Vec::new();
    let mut cmd_line = env::args();
    let mut source_line: String = String::new();
    cmd_line.next().unwrap();
    let path = cmd_line.next().unwrap();
    sourcefile = File::open(path).unwrap();
    sourcefile.read_to_string(&mut source_line).unwrap();

    let tokens: Vec<lexapro::Token> = match lexapro::lexapro_doesnt_work(&source_line){
        Ok(yes) => yes,
        Err(e) => {
            eprintln!("{}", e);
            return;
        },
    };
    println!("{:?}", tokens);
    parse = parser::Parser::new_parser(tokens);
    //First pass, assembles but doesn't do anything for the symbols
    //While else statement would be usefull for this kind of mess
    loop{
        match parse.parsing(){
            Ok(Some(i)) => {
                //Add the address of where the label has been defined
                if let Some(lab) = &i.label{
                    symbol_table.push(SymbolEntry{symbol: lab.clone(), pc})
                }
                //Assembling the instruction
                code.push(ByteCode{
                    opcode: i.mnemonic as u8,
                    //Convert the string operand into an immediate operand
                    imm: if let Some(op) = &i.operand{
                        match op{
                            //Label operands will be dealth with in the second pass
                            Operand::Lab(lab) => {ref_table.push(SymbolEntry{symbol: lab.clone(), pc}); 0},
                            Operand::Imm(imm) => *imm,
                        }
                    }else{
                        0x757775dead  //Instruction with no operand still have an operand field so we don't care about the value
                    },
                });
                //End assembling
                println!("{:?}", i);
                pc += 1;
            },
            Err(e) => {
                eprintln!("{}", e);
                return;
            },
            Ok(None) => break,
        }
    }
    println!("{:?}", code);
    //Second pass, fixes the offset for symbols
    for xref in ref_table{
        for symbol in &symbol_table{
            if xref.symbol.cmp(&symbol.symbol) == Ordering::Equal{
                code[xref.pc].imm = ((symbol.pc as i64 - (xref.pc + 1) as i64) * INSTRUCTION_SIZE as i64) as u64;
                break;
            }
        }
    }

    let mut bytecode_file= File::create_new("vm_code.byc").unwrap();
    for inst in code{
        let bytecode = inst.to_bytes();
        bytecode_file.write(&bytecode).unwrap();
    }

    return;
}