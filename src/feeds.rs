#[allow(unused)]
use super::{mem, reg, Register::*, Sequence::*, Value::*};

use crate::Sequence;

// NOTE: pe = project euler

pub fn pe1() -> impl Into<Vec<Sequence>> {
    [
        Mov(reg!(Ax), Hex(0)),
        Mov(reg!(Cx), Hex(0)),
        Mov(reg!(Dx), Hex(0)),
        Mov(reg!(Ax), reg!(Cx)), // 3
        Mod(Hex(3)),
        Je(Hex(13)),
        Mov(reg!(Ax), reg!(Cx)),
        Mod(Hex(5)),
        Je(Hex(13)),
        Inc(reg!(Cx)), // 9
        Cmp(reg!(Cx), Hex(1000)),
        Jl(Hex(3)),
        Jmp(Hex(15)),
        Add(reg!(Dx), reg!(Cx)), // 13
        Jmp(Hex(9)),
        // 15
    ]
}

pub fn pe2() -> impl Into<Vec<Sequence>> {
    [
        Mov(reg!(Dx), Hex(2)),
        Push(Hex(1)),
        Push(Hex(2)),
        Pop(reg!(Cx)), // 3
        Pop(reg!(Bx)),
        Mov(reg!(Ax), Hex(0)),
        Add(reg!(Ax), reg!(Cx)),
        Add(reg!(Ax), reg!(Bx)),
        Push(reg!(Cx)),
        Push(reg!(Ax)),
        Push(reg!(Ax)),
        Mod(Hex(2)),
        Cmp(reg!(Ax), Hex(0)),
        Pop(reg!(Ax)),
        Je(Hex(18)),
        Cmp(reg!(Ax), Hex(4_000_000)), // 15
        Jl(Hex(3)),
        Jmp(Hex(100)),
        Add(reg!(Dx), reg!(Ax)), // 18
        Jmp(Hex(15)),
        // 20
    ]
}

// NOTE: answer written in register cx
pub fn pe3() -> impl Into<Vec<Sequence>> {
    [
        Mov(reg!(Cx), Hex(600_851_475_143)),
        Mov(reg!(Bx), Hex(2)),
        Mov(reg!(Ax), reg!(Bx)),
        Mul(reg!(Ax)),
        Cmp(reg!(Ax), reg!(Cx)),
        Jge(Hex(16)),
        Mov(reg!(Ax), reg!(Cx)),
        Mod(reg!(Bx)),
        Cmp(reg!(Ax), Hex(0)),
        Jne(IHex(5)),
        Mov(reg!(Ax), reg!(Cx)),
        Div(reg!(Bx)),
        Mov(reg!(Cx), reg!(Ax)),
        Jne(IHex(2)),
        Inc(reg!(Bx)),
        Jmp(Hex(2)),
    ]
}
