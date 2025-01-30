fn main() {
    use hex_vm::stack;
    use hex_vm::Register::*;
    use hex_vm::Sequence::*;
    tracing_subscriber::fmt().init();
    hex_vm::HexVm::new([
        // Print(hex_vm::Value::Plain(((b'h' as u16) << 8) + b'i' as u16)),
        // Print(hex_vm::Value::Plain(((b'\n' as u16) << 8) + b'\n' as u16)),
        // Mov(Ax.into(), hex_vm::Value::Plain(u16::MAX)),
        // Add(Ax.into(), hex_vm::Value::Plain(u16::MAX)),
        // Inc(Ax.into()),
        // Cmp(Ax.into(), hex_vm::Value::Plain(100)),
        // Jl(stack(0)),
    ])
    .run();
    // println!("{}", vm.reg.ax as i16);
    // println!("{:#?} {:#?}", vm.reg, vm.flg);
}
