#[allow(unused)]
use hex_vm::{mem, reg, Register::*, Sequence::*, Value::*};

// TODO: implement call
fn main() {
    setup_tracing();
    let m = 32;
    let n = 1;
    for i in n..m {
        for j in i + 1..m {
            let a = j * j - i * i;
            let b = i * j * 2;
            let c = j * j + i * i;
            if a + b + c == 1000 {
                println!("{a}² + {b}² = {c}²");
                break;
            }
        }
    }
    let mut vm = hex_vm::parse::Parser::new(include_str!("../project-euler/problem-9.asm")).parse();
    vm.run();
    println!(
        "{:#?} {:#?} {:?}",
        vm.reg,
        vm.flg,
        &vm.mem[vm.reg.sp as usize..]
    );
}

fn setup_tracing() {
    use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
}
