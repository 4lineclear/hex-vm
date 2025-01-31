use std::collections::HashMap;

use string_interner::{DefaultStringInterner, DefaultSymbol};

use crate::{
    lex::{Advance, BaseLexer, Lexeme::*, Lexer},
    mem, reg,
    span::Span,
    Address, HexSize, HexVm, Register, Sequence, Value,
};

#[cfg(test)]
pub mod test;

#[derive(Debug, Default)]
pub struct Parser<L, S> {
    pub si: DefaultStringInterner,
    pub src: S,
    pub lexer: L,
    pub seq: Vec<Sequence>,
    pub labels: HashMap<DefaultSymbol, HexSize>,
}

impl<'a> Parser<BaseLexer<'a>, &'a str> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            lexer: BaseLexer::new(src),
            ..Default::default()
        }
    }
}

impl<L: Lexer, S: AsRef<str>> Parser<L, S> {
    pub fn parse(mut self) -> HexVm {
        self.parse_inner();
        HexVm {
            si: self.si,
            ..HexVm::new(self.seq, self.labels)
        }
    }

    fn parse_inner(&mut self) {
        loop {
            let ad = self.lexer.advance();
            match ad.lex {
                Whitespace => continue,
                Ident => {
                    let Some(value) = self.parse_line(ad) else {
                        continue;
                    };
                    self.seq.push(value)
                }
                Eol(_) => (),
                Eof => break,
                _ => self.unexpected(ad),
            };
        }
    }

    fn parse_line(&mut self, first: Advance) -> Option<Sequence> {
        let second = self.peek_non_ws();
        if let Colon = second.lex {
            self.lexer.pop_peek();
            self.clear_line();
            let name = self.symbol(first.span);
            if self
                .labels
                .insert(name, self.seq.len() as HexSize)
                .is_some()
            {
                self.kill_line();
                panic!("duplicate label: {}", self.slice(first.span))
            }
            return None;
        }

        match self.slice(first.span) {
            "push" => Some(Sequence::Push(self.expect_value())),
            "jmp" => Some(Sequence::Jmp(self.expect_value_ext())),
            "je" => Some(Sequence::Je(self.expect_value_ext())),
            "jne" => Some(Sequence::Jne(self.expect_value_ext())),
            "jl" => Some(Sequence::Jl(self.expect_value_ext())),
            "jle" => Some(Sequence::Jle(self.expect_value_ext())),
            "jg" => Some(Sequence::Jg(self.expect_value_ext())),
            "jge" => Some(Sequence::Jge(self.expect_value_ext())),
            "call" => {
                let ad = self.non_ws();
                let Ident = ad.lex else {
                    self.unexpected(ad);
                };
                self.kill_line();
                Some(Sequence::Call(self.symbol(ad.span)))
            }
            "ret" => {
                self.kill_line();
                Some(Sequence::Ret)
            }
            "mul" => Some(Sequence::Mul(self.expect_value())),
            "div" => Some(Sequence::Div(self.expect_value())),
            "mod" => Some(Sequence::Mod(self.expect_value())),
            "pop" => Some(Sequence::Pop(self.expect_address())),
            "inc" => Some(Sequence::Inc(self.expect_address())),
            "dec" => Some(Sequence::Dec(self.expect_address())),
            "str" => {
                let ad = self.non_ws();
                let Str = ad.lex else {
                    self.unexpected(ad);
                };
                self.kill_line();
                Some(Sequence::Str(self.symbol(ad.span)))
            }
            "mov" => {
                let address = self.expect_address();
                self.expect_comma();
                let val = self.expect_value();
                self.kill_line();
                Some(Sequence::Mov(address, val))
            }
            "cmp" => {
                let v1 = self.expect_value();
                self.expect_comma();
                let v2 = self.expect_value();
                self.kill_line();
                Some(Sequence::Cmp(v1, v2))
            }
            "add" => {
                let address = self.expect_address();
                self.expect_comma();
                let val = self.expect_value();
                self.kill_line();
                Some(Sequence::Add(address, val))
            }
            "sub" => {
                let address = self.expect_address();
                self.expect_comma();
                let val = self.expect_value();
                self.kill_line();
                Some(Sequence::Sub(address, val))
            }
            "print" => {
                let address = self.expect_address();
                self.expect_comma();
                let hex = self.expect_hex();
                self.kill_line();
                Some(Sequence::Print(address, hex))
            }
            s => panic!("invalid instruction: {s}"),
        }
    }

    fn expect_value_ext(&mut self) -> Value {
        self.value(true).expect("expected value")
    }

    fn expect_value(&mut self) -> Value {
        self.value(false).expect("expected value")
    }

    fn expect_address(&mut self) -> Address {
        match self.expect_value() {
            Value::Address(add) => add,
            val => panic!("invalid val found: {val:?}"),
        }
    }

    fn expect_hex(&mut self) -> HexSize {
        match self.expect_value() {
            Value::Hex(h) => h,
            val => panic!("invalid val found: {val:?}"),
        }
    }

    fn expect_comma(&mut self) {
        let ad = self.non_ws();
        if Comma != ad.lex {
            self.unexpected(ad);
        }
    }

    fn value(&mut self, label: bool) -> Option<Value> {
        let ad = self.non_ws();
        match ad.lex {
            Eol(_) | Eof => None,
            Ident if label => match self.try_reg(ad.span) {
                Ok(reg) => Some(Value::Address(reg.into())),
                Err(s) => Some(Value::Address(Address::Ident(s))),
            },
            Ident => Some(Value::Address(reg!(self.reg(ad.span)))),
            Digit(base) => {
                let n = HexSize::from_str_radix(self.slice(ad.span), base as u32)
                    .expect("invalid integer");
                Some(Value::Hex(n))
            }
            OpenBracket => {
                let (span, db) = self.after_bracket();
                db.map_or_else(
                    || Some(Value::Address(reg!(self.reg(span), true))),
                    |n| Some(Value::Address(mem(n))),
                )
            }
            _ => self.unexpected(ad),
        }
    }

    fn after_bracket(&mut self) -> (Span, Option<HexSize>) {
        let first = self.non_ws();
        let db = match first.lex {
            Digit(b) => Some(
                HexSize::from_str_radix(self.slice(first.span), b as u32).expect("invalid integer"),
            ),
            Ident => None,
            _ => self.unexpected(first),
        };
        if !matches!(first.lex, Ident | Digit(_)) {
            self.unexpected(first);
        }
        let close = self.non_ws();
        if CloseBracket != close.lex {
            self.unexpected(close);
        }
        (first.span, db)
    }

    fn unexpected(&mut self, ad: Advance) -> ! {
        self.mkill_line(ad);
        panic!("unexpected lexeme: {ad:?}")
    }

    fn non_ws(&mut self) -> Advance {
        while let Whitespace = self.lexer.peek().lex {
            self.lexer.pop_peek();
        }
        self.lexer.advance()
    }
    fn peek_non_ws(&mut self) -> Advance {
        while let Whitespace = self.lexer.peek().lex {
            self.lexer.pop_peek();
        }
        self.lexer.peek()
    }
    fn clear_line(&mut self) {
        let ad = self.non_ws();
        let (Eol(_) | Eof) = ad.lex else {
            self.unexpected(ad);
        };
    }

    fn mkill_line(&mut self, mut ad: Advance) {
        while !matches!(ad.lex, Eol(_) | Eof) {
            ad = self.lexer.advance();
        }
    }

    fn kill_line(&mut self) {
        while !matches!(self.lexer.advance().lex, Eol(_) | Eof) {}
    }

    fn reg(&mut self, span: impl Into<Span>) -> Register {
        match self.try_reg(span) {
            Ok(reg) => reg,
            Err(s) => panic!(
                "invalid ident used: {}",
                self.si.resolve(s).expect("invalid symbol")
            ),
        }
    }

    fn try_reg(&mut self, span: impl Into<Span>) -> Result<Register, DefaultSymbol> {
        Ok(match span.into().slice(self.src.as_ref()) {
            "ax" => Register::Ax,
            "bx" => Register::Bx,
            "cx" => Register::Cx,
            "dx" => Register::Dx,
            "si" => Register::Si,
            "di" => Register::Di,
            "sp" => Register::Sp,
            "bp" => Register::Bp,
            "ip" => Register::Ip,
            s => return Err(self.si.get_or_intern(s)),
        })
    }

    fn symbol(&mut self, span: impl Into<Span>) -> DefaultSymbol {
        self.si.get_or_intern(span.into().slice(self.src.as_ref()))
    }

    fn slice(&self, span: impl Into<Span>) -> &str {
        span.into().slice(self.src.as_ref())
    }
}
