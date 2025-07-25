//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/Interpreter-Poc
mod vm;
use vm::Vm;

fn main() {
    let code = [0x0b, 0xff, 0, 0, 0,0, 0, 0, 0, 0x12, 0x16, 0, 0, 0,0, 0, 0, 0, 0x0b, 0x02, 0, 0, 0,0, 0, 0, 0, 0x8, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x13, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0xff, 0xff, 0xff, 0xff,0xff, 0xff, 0xff, 0xff];
    unsafe{
        let mut vm: Vm = Vm::new_vm(&code, 0);
        vm.dispatcher();
    }
}
