//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/Interpreter-Poc
mod vm;
use vm::Vm;
use std::env::*;
use std::fs::File;
use std::io::Read;

fn main() {
    // let code = [0x0b, 0xff, 0, 0, 0,0, 0, 0, 0, 0x12, 0x16, 0, 0, 0,0, 0, 0, 0, 0x0b, 0x02, 0, 0, 0,0, 0, 0, 0, 0x8, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x13, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0xff, 0xff, 0xff, 0xff,0xff, 0xff, 0xff, 0xff];
    let mut flag = false;
    const STK_SWITCH: &str = "-stk";
    const RA_SWITCH: &str = "-ra";
    let mut input = args();
    let mut stk: usize = 0;
    let mut ra: usize = 0;
    let mut bytecode_file: File;
    let mut bytecode: Vec<u8> = Vec::new();

    input.next();
    for arg in input.next(){
        match arg.as_str(){
            RA_SWITCH  => {
                if ra != 0 {input.next(); continue}
                match input.next(){
                    Some(idk) =>ra = idk.clone().parse::<usize>().unwrap(),
                    None => {
                        eprintln!("[-]Switch \"-ra\" is missing parameters");
                        return;
                    }
                }
            },
            STK_SWITCH  => {
                if stk != 0 {input.next(); continue}
                match input.next(){
                    Some(idk) => stk = idk.clone().parse::<usize>().unwrap(),
                    None => {
                        eprintln!("[-]Switch \"-ra\" is missing parameters");
                        return;
                    }
                }
            },
            _ => {
                flag = true;
                bytecode_file = File::open(arg.clone()).unwrap();
                bytecode_file.read_to_end(&mut bytecode).unwrap();
            },
        };
    };

    if !flag{
        eprintln!("[-]No path specified");
        return
    }

    unsafe{
        let mut vm: Vm = Vm::new_vm(&bytecode, ra, stk);
        vm.dispatcher();
    }
}
