#[allow(unused)]
use hex_vm::{mem, reg, Register::*, Sequence::*, Value::*};

// TODO: implement call
fn main() {
    setup_tracing();
    let mut vm = hex_vm::parse::Parser::new(include_str!("../project-euler/problem-8.asm")).parse();
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
