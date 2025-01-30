fn main() {
    #[allow(unused)]
    use hex_vm::{mem, reg, Register::*, Sequence::*, Value::*};
    tracing_subscriber::fmt().init();
    // TODO: implement call
    let mut vm = hex_vm::HexVm::new([
        ////
    ]);
    vm.run();
    println!("{:#?} {:#?}", vm.reg, vm.flg,);
}
