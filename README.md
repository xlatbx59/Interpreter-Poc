# Interpreter POC

This is a poc of an stack based interpreter a bit like java with a custom. The assembler can take source files as input and will output a byc file as it's output.

## Description

This is a 64bit stack machine, still figuring out if it'll be Risc or Cisc.

## Assembler

The syntax to write for this assembler is fairly simple(lazy), whitespaces are ignore, comments too:

```
;Don't try executing this code, I'm just showing off the syntax
start: 		;Labels on single lines like this have to finish with a colon

	push 0x59	;supports hexadecimal
    push 932	;supports decimal
    add
    bz end		
    dup
    bnz start
end ret		;Labels preceding an instruction this don't need a colon
```

## Virtual Machine

### Usage
- "-ra" switch is to define the size of the return address stack(as to be decimal)
- "-stk" switch is to define the size of the general stack(as to be decimal)
Example: "./vm -ra 38 -stk 40 <path to bytecode>"

### Code

Self explanatory, executable bytecode resides there

### Stacks

- Data stack: this stack is used for data manipulation
- Return address stack: this stack saves the return addresses

### Instructions

9 bytes instructions. Only 23 supported for the moment ('i' means there's an immediate):

#### Other
- Syscall: currently unimplemented

#### Stack operations
- Push: push an immediate on the stack
- Pop: pops a value off the stack
- Ldi: loads i variable on the top of the stack
- Sti: stores value on the top of the stack to i variable
- Dup: duplicates the value on top of the stack

#### Arithmetic
- Add: adds two values on top of the stack
- Sub: subs two values on top of the stack
- Addi: adds the value on top of the stack with the operand
- Subi: subs the value on top of the stack with the operand
- Mul: mul two values on top of the stack
- Muli: multiply the value on top of the stack with the operand

#### Logical
- Xor: xors two values on top of the stack
- Or: ors two values on top of the stack
- And: ands two values on top of the stack
- Not: nots the value on top of the stack
- Shl: shifts the value on top of the stack by the second value on top of the stack to the left
- Shr: shifts the value on top of the stack by the second value on top of the stack to the right

#### Branch
- Ret: return to the return address
- BL: branch and links
- Bz: subs the two values on top of the stack and branch if zero branch, the offset as to be specified
- Bnz: subs the two values on top of the stack and branch if not zero, the offset as to be specified

### Registers

There are a few registers, they're not general purpose they can not be used for calculation or whatever

- Sp: the stack pointer that keeps track of the top of the stack
- Bp: the base register that keeps track of the base of a stack frame for each functions
- Pc: program counter, it points to the executed instruction
- Ra: just like the Sp but for the stack for the return addresses

> ## Issues
> No Vmentry/Vmexit

## Advantages
- :3

## Disadvantages
- Size: please do not talk to me about the size of the bytecode