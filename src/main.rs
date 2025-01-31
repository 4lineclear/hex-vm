#[allow(unused)]
use hex_vm::{mem, reg, Register::*, Sequence::*, Value::*};

// TODO: implement call
fn main() {
    tracing_subscriber::fmt().init();
    'outer: for i in (0..1000).rev() {
        for j in (0..i).rev() {
            let mut num = i * j;
            let mut rev = 0;

            while num > 0 {
                let dig = num % 10;
                rev = rev * 10 + dig;
                num = num / 10;
            }
            if i * j == rev {
                println!("{} {i} {j}", i * j);
                break 'outer;
            }
        }
    }
    let mut vm = hex_vm::parse::Parser::new(include_str!("../project-euler/problem-4.asm")).parse();
    vm.run();
    println!("{:#?} {:#?}", vm.reg, vm.flg,);
}
