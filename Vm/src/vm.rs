//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/Interpreter-Poc

#![warn(unsafe_op_in_unsafe_fn)]

use core::alloc;
use std::alloc::{alloc, dealloc, Layout};

const PAGE_SIZE: usize = 4096;
const INSTTRUCTION_SIZE: u8 = 9;

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
    pub unsafe fn new_vm(code:&[u8], pc:isize) -> Vm{
        let layout = Layout::array::<u64>( PAGE_SIZE ).unwrap();
        let memory = alloc(layout);
        let mut temp = pc;

        for bytes in code{
            *memory.offset(temp) = *bytes;
            temp += 1;
        }

        Vm{memory: memory as *mut u64, ra: 64, bp: 66, sp: 66, pc, imm: 0, memory_size: PAGE_SIZE as u64}
    }

    //Setting flags
    // fn set_signflag(&mut self, x: u64) -> (){
        // if x & 0x8000000000000000 == 0x8000000000000000 { self.status_flag |= SET_SF; }
        // else { self.status_flag &= !SET_SF; }
    // }
    // fn set_carryflag(&mut self, x: u64, y: u64) -> (){
        // if y & 0x8000000000000000 == x & 0x8000000000000000{
            // if x == 0 { self.status_flag |= SET_CF; }
            // else { self.status_flag &= !SET_CF }
        // }
    // }
    // fn set_overflowflag(&mut self, x: u64, y: u64) -> (){
        // if x < 0x8000000000000000 && y >= 0x8000000000000000
        // || x >= 0x8000000000000000 && y < 0x8000000000000000 {
            // self.status_flag |= SET_OF;
        // }
        // else { self.status_flag &= !SET_OF; }
    // }
    // fn set_paritylag(&mut self, x: u64) -> (){
        // if x & 1 == 1 { self.status_flag |= SET_PF; }
        // else { self.status_flag &= !SET_PF; }
    // }

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

    unsafe fn syscall(&mut self){
        unimplemented!()
    }
    //Stack stuff
    unsafe fn dup(&mut self) -> (){
        self.stack_push(*self.memory.offset(self.sp));
    }
    unsafe fn pop(&mut self) -> (){
        self.stack_pop();
    }
    unsafe fn push(&mut self) -> (){        //Good
        self.stack_push(self.imm);
    }
    unsafe fn store(&mut self) -> (){
        self.memory.offset(self.bp + self.imm as isize).write(self.stack_pop());
    }
    unsafe fn load(&mut self) -> (){
        self.stack_push(*self.memory.offset(self.bp + self.imm as isize));
    }

    //add sub
    unsafe fn addi(&mut self) -> (){
        let x: u64 = self.stack_pop();
        let result = x.wrapping_add(self.imm);

        self.stack_push(result);
    }
    unsafe fn nop(&mut self) -> (){
        let temp = self.stack_pop();
        self.stack_push(temp)
    }
    unsafe fn subi(&mut self) -> (){
        let x: u64 = self.stack_pop();
        let result = x.wrapping_sub(self.imm);

        self.stack_push(result);
    }
    unsafe fn add(&mut self) -> (){
        let x: u64 = self.stack_pop();
        let result = x.wrapping_add(self.stack_pop());

        self.stack_push(result);
    }
    unsafe fn sub(&mut self) -> (){
        let x: u64 = self.stack_pop();
        let result = x.wrapping_sub(self.stack_pop());

        self.stack_push(result);
    }
    unsafe fn mul(&mut self) -> (){
        let x: u64 = self.stack_pop();
        let result = x.wrapping_mul(self.stack_pop());

        self.stack_push(result);
    }
    unsafe fn muli(&mut self) -> (){
        let x: u64 = self.stack_pop();
        let result = x.wrapping_mul(self.imm);

        self.stack_push(result);
    }

    //xor and or
    unsafe fn xor(&mut self) -> (){
        let x: u64 = self.stack_pop();
        let result = x ^ self.stack_pop();

        self.stack_push(result);
    }
    unsafe fn and(&mut self) -> (){
        let x: u64 = self.stack_pop();
        let result = x & self.stack_pop();

        self.stack_push(result);
    }
    unsafe fn or(&mut self) -> (){
        let x: u64 = self.stack_pop();
        let result = x | self.stack_pop();

        self.stack_push(result);
    }
    unsafe fn not(&mut self) -> (){
        let result: u64 = !self.stack_pop();

        self.stack_push(result);
    }

    //shl shr
    unsafe fn shl(&mut self) -> (){
        let x = self.stack_pop();
        let result = x << self.stack_pop();

        self.stack_push( result );
    }
    unsafe fn shr(&mut self) -> (){
        let x = self.stack_pop();
        let result = x >> self.stack_pop();

        self.stack_push( result );
    }

    //ret bl bz bnz
    unsafe fn ret(&mut self) ->(){
        self.pc = self.stack_pop_return();
    }
    unsafe fn bl(&mut self) ->(){
        self.stack_push_return(self.pc);
        self.pc += self.imm as isize;
    }
    unsafe fn bz(&mut self) ->(){
        if self.stack_pop() - self.stack_pop() == 0{
            self.pc += self.imm as isize;
        }
    }
    unsafe fn bnz(&mut self) ->(){
        if self.stack_pop() - self.stack_pop() != 0{
            self.pc += self.imm as isize;
        }
    }
}
