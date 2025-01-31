use std::collections::HashMap;

use expect_test::{expect, Expect};
use string_interner::DefaultSymbol;

use crate::{feeds, HexSize, Sequence};

fn check(s: &str, seq: Option<(Vec<Sequence>, HashMap<DefaultSymbol, HexSize>)>, e: Expect) {
    let mut h_ac = super::Parser::new(s).parse();
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
    if let Some((seq_ex, label_ex)) = seq {
        assert_eq!(seq_ex, h_ac.seq);
        let mut h_ex = crate::HexVm::new(seq_ex, label_ex);
        h_ac.run();
        h_ex.run();
        assert_eq!(h_ac, h_ex);
    }
}

#[test]
fn pe1() {
    let src = r"
mov ax, 0
mov cx, 0
mov dx, 0
mov ax, cx
mod 3
je 13
mov ax, cx
mod 5
je 13
inc cx
cmp cx, 1000
jl 3
jmp 15
add dx, cx
jmp 9
";
    check(
        src,
        Some((feeds::pe1().into(), [].into())),
        expect![[r#"
            []
            Mov(Register(Ax, false), Hex(0))
            Mov(Register(Cx, false), Hex(0))
            Mov(Register(Dx, false), Hex(0))
            Mov(Register(Ax, false), Address(Register(Cx, false)))
            Mod(Hex(3))
            Je(Hex(13))
            Mov(Register(Ax, false), Address(Register(Cx, false)))
            Mod(Hex(5))
            Je(Hex(13))
            Inc(Register(Cx, false))
            Cmp(Address(Register(Cx, false)), Hex(1000))
            Jl(Hex(3))
            Jmp(Hex(15))
            Add(Register(Dx, false), Address(Register(Cx, false)))
            Jmp(Hex(9))"#]],
    );
}

#[test]
fn pe1_label() {
    let src = r"
    mov ax, 0
    mov cx, 0
    mov dx, 0
main_loop:
    mov ax, cx
    mod 3
    je do_add
    mov ax, cx
    mod 5
    je do_add
inc_loop:
    inc cx
    cmp cx, 1000
    jl main_loop
    jmp end
do_add:
    add dx, cx
    jmp inc_loop
end:
";
    check(
        src,
        None,
        expect![[r#"
            [
                "do_add: 13",
                "end: 15",
                "inc_loop: 9",
                "main_loop: 3",
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
