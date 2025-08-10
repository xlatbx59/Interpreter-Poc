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

    // input.next();
    // while let Some(arg) = input.next(){
    //     match arg.as_str(){
    //         RA_SWITCH  => {
    //             println!("Return address stack {}", arg);
    //             if ra != 0 {input.next(); continue}
    //             match input.next(){
    //                 Some(idk) => ra = idk.clone().parse::<usize>().unwrap(),
    //                 None => {
    //                     eprintln!("[-]Switch \"-ra\" is missing parameters");
    //                     return;
    //                 }
    //             }
    //         },
    //         STK_SWITCH  => {
    //             println!("General stack size{}", arg);
    //             if stk != 0 {input.next(); continue}
    //             match input.next(){
    //                 Some(idk) => stk = idk.clone().parse::<usize>().unwrap(),
    //                 None => {
    //                     eprintln!("[-]Switch \"-ra\" is missing parameters");
    //                     return;
    //                 }
    //             }
    //         },
    //         _ => {
    //             flag = true;
    //             println!("Path {}", arg);
    //             bytecode_file = File::open(arg.clone()).unwrap();
    //             bytecode_file.read_to_end(&mut bytecode).unwrap();
    //         },
    //     };
    // };

    // if ra == 0 && !flag && stk == 0{
    //     eprintln!("[!]Usage: ./vm -ra 42 -stk 29 <path>");
    //     return
    // }
    // else if !flag{
    //     eprintln!("[-]No path specified");
    //     return
    // }

    bytecode_file = File::open("../interpreter-asm/vm_code.byc").unwrap();
    bytecode_file.read_to_end(&mut bytecode).unwrap();
    ra = 0x14;
    stk = 0x14;

    unsafe{
        let mut vm: Vm = Vm::new_vm(&bytecode, ra, stk);
        vm.dispatcher();
    }
    println!("[+]Vm exited successfully!");
}