//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/Interpreter-Poc

use super::*;
use std::alloc::{dealloc, Layout};

impl Vm{
    pub(super) unsafe fn syscall(&mut self){
        unimplemented!()
    }
    pub(super) unsafe fn vm_exit(&mut self){
        let layout = Layout::array::<u64>( self.memory_size as usize). unwrap();
        self.exit_flag = true;
        dealloc(self.memory as *mut u8, layout);
    }
    //Stack stuff
    pub(super) unsafe fn dup(&mut self) -> (){
        self.stack_push(*self.memory.offset(self.sp));
    }
    pub(super) unsafe fn pop(&mut self) -> (){
        self.stack_pop();
    }
    pub(super) unsafe fn push(&mut self) -> (){        //Good
        self.stack_push(self.imm);
    }
    pub(super) unsafe fn store(&mut self) -> (){
        self.memory.offset(self.bp + self.imm as isize).write(self.stack_pop());
    }
    pub(super) unsafe fn load(&mut self) -> (){
        self.stack_push(*self.memory.offset(self.bp + self.imm as isize));
    }

    //add sub
    pub(super) unsafe fn addi(&mut self) -> (){
        let x: u64 = self.stack_pop();
        let result = x.wrapping_add(self.imm);

        self.stack_push(result);
    }
    pub(super) unsafe fn nop(&mut self) -> (){
        let temp = self.stack_pop();
        self.stack_push(temp)
    }
    pub(super) unsafe fn subi(&mut self) -> (){
        let x: u64 = self.stack_pop();
        let result = x.wrapping_sub(self.imm);

        self.stack_push(result);
    }
    pub(super) unsafe fn add(&mut self) -> (){
        let x: u64 = self.stack_pop();
        let result = x.wrapping_add(self.stack_pop());

        self.stack_push(result);
    }
    pub(super) unsafe fn sub(&mut self) -> (){
        let x: u64 = self.stack_pop();
        let result = x.wrapping_sub(self.stack_pop());

        self.stack_push(result);
    }
    pub(super) unsafe fn mul(&mut self) -> (){
        let x: u64 = self.stack_pop();
        let result = x.wrapping_mul(self.stack_pop());

        self.stack_push(result);
    }
    pub(super) unsafe fn muli(&mut self) -> (){
        let x: u64 = self.stack_pop();
        let result = x.wrapping_mul(self.imm);

        self.stack_push(result);
    }

    //xor and or
    pub(super) unsafe fn xor(&mut self) -> (){
        let x: u64 = self.stack_pop();
        let result = x ^ self.stack_pop();

        self.stack_push(result);
    }
    pub(super) unsafe fn and(&mut self) -> (){
        let x: u64 = self.stack_pop();
        let result = x & self.stack_pop();

        self.stack_push(result);
    }
    pub(super) unsafe fn or(&mut self) -> (){
        let x: u64 = self.stack_pop();
        let result = x | self.stack_pop();

        self.stack_push(result);
    }
    pub(super) unsafe fn not(&mut self) -> (){
        let result: u64 = !self.stack_pop();

        self.stack_push(result);
    }

    //shl shr
    pub(super) unsafe fn shl(&mut self) -> (){
        let x = self.stack_pop();
        let result = x << self.stack_pop();

        self.stack_push( result );
    }
    pub(super) unsafe fn shr(&mut self) -> (){
        let x = self.stack_pop();
        let result = x >> self.stack_pop();

        self.stack_push( result );
    }

    //ret bl bz bnz
    pub(super) unsafe fn ret(&mut self) ->(){
        self.pc = self.stack_pop_return();
    }
    pub(super) unsafe fn bl(&mut self) ->(){
        self.stack_push_return(self.pc);
        self.pc += self.imm as isize;
    }
    pub(super) unsafe fn bz(&mut self) ->(){
        if self.stack_pop() - self.stack_pop() == 0{
            self.pc += self.imm as isize;
        }
    }
    pub(super) unsafe fn bnz(&mut self) ->(){
        if self.stack_pop() - self.stack_pop() != 0{
            self.pc += self.imm as isize;
        }
    }
}