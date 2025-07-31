//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/Interpreter-Poc

#![warn(unsafe_op_in_unsafe_fn)]

mod interpreter;

use core::alloc;
use std::alloc::{alloc, dealloc, Layout};

const PAGE_SIZE: usize = 4096;
const INSTTRUCTION_SIZE: u8 = 9;

fn align(to_align: usize, align_on_x_bytes: usize) -> usize{
    if to_align == 0{
        return align_on_x_bytes
    }
    align_on_x_bytes.wrapping_mul(align_on_x_bytes.wrapping_div(to_align))
}

pub struct Vm{
    memory: *mut u64,
    ra: isize,  //Return address stack pointer
    bp: isize,  //General stack base
    sp: isize,  //General stack pointer
    pc: isize,  //Program counter
    imm: u64,   //Operand
    memory_size: u64,   //Self explanatory
}

//TODO: Push and Pop are supposed to return Error incase of stack overflow or underflow
impl Vm{
    unsafe fn handler_table(&mut self, opcode: usize) -> (){
        const HANDLERS: [unsafe fn(&mut Vm) -> (); 22] = [
            Vm::add, Vm::sub, Vm::subi, Vm::and, Vm::xor, Vm::load, Vm::or, Vm::bl,
            Vm::shl, Vm::shr, Vm::store, Vm::push, Vm::pop, Vm::dup, Vm::addi, Vm::bnz,
            Vm::bz, Vm::ret, Vm::not, Vm::mul, Vm::muli, Vm::nop];
        HANDLERS[opcode](self)
    }
    pub unsafe fn dispatcher(&mut self) -> (){
        loop{
            let opcode: usize = *(self.memory as *mut u8).byte_offset(self.pc) as usize;
            self.pc +=1;
            self.imm = self.memory.byte_offset(self.pc).read();
            self.pc += 8;
            self.handler_table(opcode);
        }
    }
    pub unsafe fn new_vm(code:&[u8], ra_stack_size: usize, stack_size: usize) -> Vm{
        let code_size = align(code.len(), 8);
        let ra_stack = align(ra_stack_size, 128);
        let stack_size= align(stack_size, 256);
        let total_size = code_size + ra_stack + stack_size;
        let layout = Layout::array::<u64>( total_size ).unwrap();
        let memory = alloc(layout);
        let mut index: isize = 0;

        for bytes in code{
            *memory.offset((stack_size + ra_stack) as isize + index) = *bytes;
            index += 1;
        }

        Vm{memory: memory as *mut u64, ra: 0, bp: ra_stack as isize, sp: ra_stack as isize, pc: (stack_size + ra_stack) as isize, imm: 0, memory_size: total_size as u64}
    }

    //System stuff I guess
    unsafe fn stack_push_return(&mut self, value: isize) -> (){
        self.ra +=1;
        *self.memory.offset(self.ra) = value as u64;
    }
    unsafe fn stack_pop_return(&mut self) -> isize{
        self.ra -=1;
        *self.memory.offset(self.ra + 1) as isize
    }
    unsafe fn stack_push(&mut self, value: u64) -> (){  //Good
        self.sp +=1;
        *self.memory.offset(self.sp) = value;
    }
    unsafe fn stack_pop(&mut self) -> u64{
        self.sp -=1;
        *self.memory.offset(self.sp + 1)
    }

}
