#[allow(unused)]
use hex_vm::{mem, reg, Register::*, Sequence::*, Value::*};

// TODO: implement call
fn main() {
    let mut count = 0;
    for i in 0.. {
        let mut j = 2;
        let prime = loop {
            if j * j > i {
                break true;
            }
            if i % j == 0 {
                break false;
            }
            j += 1
        };
        if prime && count > 10_001 {
            println!("{i}");
            break;
        }
        count += prime as u32;
    }
    setup_tracing();
    let mut vm = hex_vm::parse::Parser::new(include_str!("../project-euler/problem-7.asm")).parse();
    vm.run();
    println!("{:#?} {:#?}", vm.reg, vm.flg,);
}

fn setup_tracing() {
    use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
}
