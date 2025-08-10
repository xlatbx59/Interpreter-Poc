//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/Interpreter-Poc

pub const IA_ADD: &str = "add"; pub const IA_ADDI: &str = "addi"; pub const IA_SUBI: &str = "subi"; 
pub const IA_SUB: &str = "sub"; pub const IA_LDI: &str = "ldi"; pub const IA_STI: &str = "sti";
pub const IA_RET: &str = "ret"; pub const IA_BZ: &str = "bz"; pub const IA_BNZ: &str = "bnz";
pub const IA_DUP: &str = "dup"; pub const IA_OR: &str = "or"; pub const IA_MULI: &str = "muli";
pub const IA_XOR: &str = "xor"; pub const IA_AND: &str = "and"; pub const IA_SHR: &str = "shr";
pub const IA_SHL: &str = "shl"; pub const IA_BL: &str = "bl"; pub const IA_PUSH: &str = "push";
pub const IA_NOT: &str = "not"; pub const IA_POP: &str = "pop"; pub const IA_MUL: &str = "mul";
pub const IA_NOP: &str = "nop"; pub const IA_SYSCALL: &str = "syscall"; pub const IA_VMEXIT: &str = "vmexit";

pub const IA_MNEMONIC: [&str; 23] = [
    IA_ADD, IA_ADDI, IA_SUBI, IA_SUB, IA_LDI, IA_STI, IA_RET,
    IA_BZ, IA_BNZ, IA_DUP, IA_OR, IA_MULI, IA_XOR, IA_AND,
    IA_SHR, IA_SHL, IA_BL, IA_PUSH, IA_NOT, IA_POP, IA_MUL,
    IA_NOP, IA_VMEXIT
];

pub const MNEMONIC_WITH_LABEL: [IaMnemonic; 3] = [
    IaMnemonic::IaBz, IaMnemonic::IaBnz, IaMnemonic::IaBl,
];

pub const MNEMONIC_WITH_IMM: [IaMnemonic; 6] = [
    IaMnemonic::IaAddi, IaMnemonic::IaSubi, IaMnemonic::IaMuli,
    IaMnemonic::IaPush, IaMnemonic::IaLdi, IaMnemonic::IaSti,
];

pub const MNEMONIC_WITH_OPERAND: [IaMnemonic; 9] = [
    IaMnemonic::IaBz, IaMnemonic::IaBnz, IaMnemonic::IaBl,
    IaMnemonic::IaAddi, IaMnemonic::IaSubi, IaMnemonic::IaMuli,
    IaMnemonic::IaPush, IaMnemonic::IaLdi, IaMnemonic::IaSti,
];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IaMnemonic {
    IaAdd, IaSub, IaSubi, 
    IaAnd, IaXor, IaLdi,
    IaOr, IaBl, IaShl,
    IaShr, IaSti, IaPush,
    IaPop, IaDup, IaAddi,
    IaBnz, IaBz, IaRet,
    IaNot,  IaMul, IaMuli,
    IaNop, IaVmexit, IaSyscall
}