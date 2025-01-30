#[allow(unused)]
use super::{register, stack, Register::*, Sequence::*, Value::*};

use crate::Sequence;

// NOTE: pe = project euler

pub fn pe1<'a>() -> impl Into<Vec<Sequence<'a>>> {
    [
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
    ]
}
