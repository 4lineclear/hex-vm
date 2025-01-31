use expect_test::{expect, Expect};

fn check(s: &str, e: Expect) {
    let h_ac = super::Parser::new(s).parse();
    let mut label = h_ac
        .labels
        .iter()
        .map(|(s, i)| format!("{}: {i}", h_ac.si.resolve(*s).unwrap()))
        .collect::<Vec<_>>();
    label.sort();
    let seq_str = h_ac
        .seq
        .iter()
        .map(|s| format!("{s:?}"))
        .collect::<Vec<_>>()
        .join("\n");
    e.assert_eq(&format!("{label:#?}\n{seq_str}",));
}

#[test]
fn pe1() {
    check(
        include_str!("../../project-euler/problem-1.asm"),
        expect![[r#"
            [
                "end: 15",
                "loop: 3",
                "to_add: 13",
                "to_inc: 9",
            ]
            Mov(Register(Ax, false), Hex(0))
            Mov(Register(Cx, false), Hex(0))
            Mov(Register(Dx, false), Hex(0))
            Mov(Register(Ax, false), Address(Register(Cx, false)))
            Mod(Hex(3))
            Je(Address(Ident(SymbolU32 { value: 2 })))
            Mov(Register(Ax, false), Address(Register(Cx, false)))
            Mod(Hex(5))
            Je(Address(Ident(SymbolU32 { value: 2 })))
            Inc(Register(Cx, false))
            Cmp(Address(Register(Cx, false)), Hex(1000))
            Jl(Address(Ident(SymbolU32 { value: 1 })))
            Jmp(Address(Ident(SymbolU32 { value: 4 })))
            Add(Register(Dx, false), Address(Register(Cx, false)))
            Jmp(Address(Ident(SymbolU32 { value: 3 })))"#]],
    );
}

#[test]
fn pe2() {
    check(
        include_str!("../../project-euler/problem-2.asm"),
        expect![[r#"
            [
                "be: 18",
                "check: 15",
                "end: 20",
                "loop: 3",
            ]
            Mov(Register(Dx, false), Hex(2))
            Push(Hex(1))
            Push(Hex(2))
            Pop(Register(Cx, false))
            Pop(Register(Bx, false))
            Mov(Register(Ax, false), Hex(0))
            Add(Register(Ax, false), Address(Register(Cx, false)))
            Add(Register(Ax, false), Address(Register(Bx, false)))
            Push(Address(Register(Cx, false)))
            Push(Address(Register(Ax, false)))
            Push(Address(Register(Ax, false)))
            Mod(Hex(2))
            Cmp(Address(Register(Ax, false)), Hex(0))
            Pop(Register(Ax, false))
            Je(Address(Ident(SymbolU32 { value: 2 })))
            Cmp(Address(Register(Ax, false)), Hex(4000000))
            Jl(Address(Ident(SymbolU32 { value: 1 })))
            Jmp(Hex(100))
            Add(Register(Dx, false), Address(Register(Ax, false)))
            Jmp(Address(Ident(SymbolU32 { value: 3 })))"#]],
    );
}

#[test]
fn pe3() {
    check(
        include_str!("../../project-euler/problem-3.asm"),
        expect![[r#"
            [
                "be: 14",
                "end: 16",
                "loop: 2",
            ]
            Mov(Register(Cx, false), Hex(600851475143))
            Mov(Register(Bx, false), Hex(2))
            Mov(Register(Ax, false), Address(Register(Bx, false)))
            Mul(Address(Register(Ax, false)))
            Cmp(Address(Register(Ax, false)), Address(Register(Cx, false)))
            Jge(Address(Ident(SymbolU32 { value: 2 })))
            Mov(Register(Ax, false), Address(Register(Cx, false)))
            Mod(Address(Register(Bx, false)))
            Cmp(Address(Register(Ax, false)), Hex(0))
            Jne(Address(Ident(SymbolU32 { value: 3 })))
            Mov(Register(Ax, false), Address(Register(Cx, false)))
            Div(Address(Register(Bx, false)))
            Mov(Register(Cx, false), Address(Register(Ax, false)))
            Jne(Address(Ident(SymbolU32 { value: 1 })))
            Inc(Register(Bx, false))
            Jmp(Address(Ident(SymbolU32 { value: 1 })))"#]],
    );
}

#[test]
fn pe4() {
    check(
        include_str!("../../project-euler/problem-4.asm"),
        expect![[r#"
            [
                "check_end: 31",
                "check_loop: 15",
                "do_check: 9",
                "end: 34",
                "greater: 26",
                "inner: 6",
                "outer: 2",
                "start: 0",
            ]
            Mov(Register(Di, false), Hex(0))
            Mov(Register(Bx, false), Hex(1000))
            Mov(Register(Cx, false), Address(Register(Bx, false)))
            Dec(Register(Bx, false))
            Cmp(Address(Register(Bx, false)), Hex(0))
            Je(Address(Ident(SymbolU32 { value: 3 })))
            Dec(Register(Cx, false))
            Cmp(Address(Register(Cx, false)), Hex(0))
            Je(Address(Ident(SymbolU32 { value: 2 })))
            Mov(Register(Ax, false), Address(Register(Cx, false)))
            Mul(Address(Register(Bx, false)))
            Mov(Register(Si, false), Address(Register(Ax, false)))
            Push(Address(Register(Bx, false)))
            Push(Address(Register(Cx, false)))
            Mov(Register(Bx, false), Hex(0))
            Mov(Register(Cx, false), Address(Register(Ax, false)))
            Mod(Hex(10))
            Mov(Register(Dx, false), Address(Register(Ax, false)))
            Mov(Register(Ax, false), Address(Register(Bx, false)))
            Mul(Hex(10))
            Add(Register(Ax, false), Address(Register(Dx, false)))
            Mov(Register(Bx, false), Address(Register(Ax, false)))
            Mov(Register(Ax, false), Address(Register(Cx, false)))
            Div(Hex(10))
            Cmp(Address(Register(Ax, false)), Hex(0))
            Jg(Address(Ident(SymbolU32 { value: 6 })))
            Cmp(Address(Register(Bx, false)), Address(Register(Si, false)))
            Jne(Address(Ident(SymbolU32 { value: 8 })))
            Cmp(Address(Register(Si, false)), Address(Register(Di, false)))
            Jl(Address(Ident(SymbolU32 { value: 8 })))
            Mov(Register(Di, false), Address(Register(Si, false)))
            Pop(Register(Cx, false))
            Pop(Register(Bx, false))
            Jmp(Address(Ident(SymbolU32 { value: 4 })))"#]],
    );
}
