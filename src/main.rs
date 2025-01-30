fn main() {
    #[allow(unused)]
    use hex_vm::{register, stack, Register::*, Sequence::*, Value::*};
    tracing_subscriber::fmt().init();
    let mut j = 0;
    for i in 0..1000 {
        if i % 5 == 0 || i % 3 == 0 {
            j += i;
        }
    }
    hex_vm::HexVm::new([
        Mov(Ax.into(), Hex(0)),
        Mov(Cx.into(), Hex(0)),
        Mov(Dx.into(), Hex(0)),
        Mov(Ax.into(), Cx.into()), // 3
        Mod(Hex(3)),
        Je(Hex(13)),
        Mov(Ax.into(), Cx.into()),
        Mod(Hex(5)),
        Je(Hex(13)),
        Inc(Cx.into()), // 9
        Cmp(Cx.into(), Hex(1000)),
        Jl(Hex(3)),
        Jmp(Hex(15)),
        Add(Dx.into(), Cx.into()), // 13
        Jmp(Hex(9)),
        // 15
        // Str("Hello, World!\n"),
        // Print(Sp.into(), 14),
        // Print(hex_vm::Value::Plain(((b'h' as u16) << 8) + b'i' as u16)),
        // Print(hex_vm::Value::Plain(((b'\n' as u16) << 8) + b'\n' as u16)),
        // Mov(Ax.into(), hex_vm::Value::Plain(u16::MAX)),
        // Add(Ax.into(), hex_vm::Value::Plain(u16::MAX)),
        // Inc(Ax.into()),
        // Cmp(Ax.into(), hex_vm::Value::Plain(100)),
        // Jl(stack(0)),
    ])
    .run();
    println!("{j}");
    // println!("{}", vm.reg.ax as i16);
    // println!("{:#?} {:#?}", vm.reg, vm.flg);
}
