#[allow(unused)]
use hex_vm::{mem, reg, Register::*, Sequence::*, Value::*};

// TODO: implement call
fn main() {
    tracing_subscriber::fmt().init();
    let mut vm = hex_vm::parse::Parser::new(include_str!("../project-euler/problem-4.asm")).parse();
    // let mut vm = hex_vm::HexVm::new(hex_vm::feeds::pe2(), []);
    vm.run();
    println!("{:#?} {:#?}", vm.reg, vm.flg,);
}
