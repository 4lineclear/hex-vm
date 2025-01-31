use std::cmp::Ordering;
use std::collections::HashMap;

use string_interner::{DefaultStringInterner, DefaultSymbol};

pub mod feeds;

pub type HexSize = u64;
pub type ValIndex = u64;
pub type IHexSize = i64;

pub const HEX_MEM_SIZE: HexSize = 0xEEEE;
pub const MEM_SIZE: usize = HEX_MEM_SIZE as usize;

pub mod lex;
pub mod parse;
pub mod span;

pub enum JmpKind {
    Jmp,
    Je,
    Jne,
    Jl,
    Jle,
    Jg,
    Jge,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct FlagSet {
    pub sf: bool,
    pub cf: bool,
    pub zf: bool,
    pub of: bool,
}

impl FlagSet {
    fn do_cmp(&mut self, a: HexSize, b: HexSize) {
        match a.cmp(&b) {
            Ordering::Less => {
                self.sf = true;
                self.cf = true;
                self.zf = false;
                self.of = false;
            }
            Ordering::Equal => {
                self.sf = false;
                self.cf = false;
                self.zf = true;
                self.of = false;
            }
            Ordering::Greater => {
                self.sf = false;
                self.cf = false;
                self.zf = false;
                self.of = false;
            }
        }
    }
    // fn to_ord(&self) -> Option<Ordering> {
    //     if self.zf {
    //         return Some(Ordering::Greater);
    //     }
    //     if self.cf {
    //         return Some(Ordering::Less);
    //     }
    //     Some(Ordering::Less)
    // }
}

#[derive(Debug, PartialEq, Eq)]
pub struct HexVm {
    pub si: DefaultStringInterner,
    pub labels: HashMap<DefaultSymbol, HexSize>,
    pub flg: FlagSet,
    pub reg: RegisterSet,
    pub seq: Vec<Sequence>,
    pub mem: [HexSize; MEM_SIZE],
}

impl HexVm {
    pub fn new(
        seq: impl Into<Vec<Sequence>>,
        labels: impl Into<HashMap<DefaultSymbol, HexSize>>,
    ) -> Self {
        Self {
            si: DefaultStringInterner::new(),
            labels: labels.into(),
            flg: FlagSet::default(),
            reg: RegisterSet {
                bp: HEX_MEM_SIZE,
                sp: HEX_MEM_SIZE,
                ..Default::default()
            },
            seq: seq.into(),
            mem: [0; MEM_SIZE],
        }
    }

    pub fn run(&mut self) {
        // tracing::info!("run start");
        while self.seq.len() > self.reg.ip as usize {
            self.sequence();
        }
        // tracing::info!("run end");
    }

    // TODO: do overflow handling
    fn sequence(&mut self) {
        use Sequence::*;
        let old = self.reg.ip;
        let seq = self.seq[self.reg.ip as usize];
        match seq {
            Mov(add, value) => *self.address_mut(add) = self.value(value),
            Cmp(a, b) => {
                // tracing::info!("{:?} cmp {:?}", self.value(a), self.value(b));
                self.flg.do_cmp(self.value(a), self.value(b))
            }
            Jmp(add) => self.jump_ord(add, JmpKind::Jmp),
            Je(add) => self.jump_ord(add, JmpKind::Je),
            Jne(add) => self.jump_ord(add, JmpKind::Jne),
            Jl(add) => self.jump_ord(add, JmpKind::Jl),
            Jle(add) => self.jump_ord(add, JmpKind::Jle),
            Jg(add) => self.jump_ord(add, JmpKind::Jg),
            Jge(add) => self.jump_ord(add, JmpKind::Jge),
            Call(sym) => {
                push(&mut self.reg.sp, &mut self.mem, self.reg.ip);
                self.reg.ip = self.labels.get(&sym).copied().unwrap();
            }
            Ret => {
                self.reg.ip = pop(&mut self.reg.sp, &mut self.mem);
            }
            Push(value) => {
                let word = self.value(value);
                push(&mut self.reg.sp, &mut self.mem, word);
            }
            Pop(add) => {
                *self.address_mut(add) = pop(&mut self.reg.sp, &mut self.mem);
            }
            // TODO: create signed math
            Inc(add) => self.apply_math(add, 1, Op::Add),
            Dec(add) => self.apply_math(add, 1, Op::Sub),
            Add(add, value) => self.apply_math(add, self.value(value), Op::Add),
            Sub(add, value) => self.apply_math(add, self.value(value), Op::Sub),
            Mul(value) => self.reg.ax = self.math(self.reg.ax, self.value(value), Op::Mul),
            Div(value) => self.reg.ax = self.math(self.reg.ax, self.value(value), Op::Div),
            Mod(value) => self.reg.ax = self.math(self.reg.ax, self.value(value), Op::Mod),
            // TODO: create actual printing system
            Str(s) => self
                .si
                .resolve(s)
                .unwrap()
                .as_bytes()
                .chunks(HexSize::BITS as usize / 8)
                .rev()
                .for_each(|b| {
                    let mut o = [0; HexSize::BITS as usize / 8];
                    o[..b.len()].copy_from_slice(b);
                    push(&mut self.reg.sp, &mut self.mem, HexSize::from_be_bytes(o));
                }),
            Print(add, len) => {
                let start = self.address(add);
                let s = String::from_utf8(
                    self.mem[start as usize..start as usize + (len as usize + 1) / 2]
                        .iter()
                        .flat_map(|&ch| ch.to_be_bytes())
                        .take(len as usize)
                        .collect::<Vec<_>>(),
                )
                .expect("invalid utf8");
                print!("{s}");
            }
        }
        // tracing::info!("exec {}", self.reg.ip);
        // tracing::info!("{seq:?}");
        // tracing::info!("{:?}", self.reg);
        // tracing::info!("{:?}", self.flg);
        // tracing::info!("end  {}, mid-change {}", self.reg.ip, (old != self.reg.ip));
        self.reg.ip += (old == self.reg.ip) as HexSize;
    }

    fn jump_ord(&mut self, value: Value, jmp: JmpKind) {
        let val = || match value {
            Value::IHex(diff) => self.reg.ip.wrapping_add_signed(diff),
            value => self.value(value),
        };
        self.reg.ip = match jmp {
            JmpKind::Jmp => val(),
            JmpKind::Je if self.flg.zf => val(),
            JmpKind::Jne if !self.flg.zf => val(),
            JmpKind::Jl if !self.flg.zf && self.flg.sf => val(),
            JmpKind::Jle if self.flg.zf || self.flg.sf => val(),
            JmpKind::Jg if !self.flg.zf && !self.flg.sf => val(),
            JmpKind::Jge if self.flg.zf || !self.flg.sf => val(),
            _ => self.reg.ip,
        };
    }

    fn apply_math(&mut self, add: Address, b: HexSize, op: Op) {
        *self.address_mut(add) = self.math(self.address(add), b, op);
    }

    fn math(&mut self, a: HexSize, b: HexSize, op: Op) -> HexSize {
        let v = match op {
            Op::Add => {
                let (v, cf) = a.overflowing_add(b);
                self.flg.cf = cf;
                self.flg.of = a < b;
                v
            }
            Op::Sub => {
                let (v, of) = a.overflowing_sub(b);
                self.flg.of = of;
                self.flg.cf = a < b;
                v
            }
            Op::Div => a / b,
            Op::Mul => a.wrapping_mul(b),
            Op::Mod => a % b,
        };
        self.flg.sf = v.leading_ones() > 0;
        // cf
        self.flg.zf = v == 0;
        // of
        v
    }

    // #[allow(unused)]
    // fn imath(&mut self, a: HexSize, b: HexSize, op: Op) -> HexSize {
    //     let a = a as IHexSize;
    //     let b = b as IHexSize;
    //     self.flg.ord = a.cmp(&b);
    //     let (v, flo) = match op {
    //         Op::Add => a.overflowing_add(b),
    //         Op::Sub => a.overflowing_sub(b),
    //         Op::Div => a.overflowing_div(b),
    //         Op::Mul => a.overflowing_mul(b),
    //         Op::Mod => (a % b, false),
    //     };
    //     self.flg.of = flo;
    //     v as HexSize
    // }

    fn reg(&self, reg: Register) -> HexSize {
        use Register::*;
        match reg {
            Ax => self.reg.ax,
            Bx => self.reg.bx,
            Cx => self.reg.cx,
            Dx => self.reg.dx,
            Si => self.reg.si,
            Di => self.reg.di,
            Sp => self.reg.sp,
            Bp => self.reg.bp,
            Ip => self.reg.ip,
        }
    }

    fn reg_mut(&mut self, reg: Register) -> &mut HexSize {
        use Register::*;
        match reg {
            Ax => &mut self.reg.ax,
            Bx => &mut self.reg.bx,
            Cx => &mut self.reg.cx,
            Dx => &mut self.reg.dx,
            Si => &mut self.reg.si,
            Di => &mut self.reg.di,
            Sp => &mut self.reg.sp,
            Bp => &mut self.reg.bp,
            Ip => &mut self.reg.ip,
        }
    }

    fn mem_at(&self, add: HexSize) -> HexSize {
        self.mem[add as usize]
    }

    fn mem_mut(&mut self, add: HexSize) -> &mut HexSize {
        &mut self.mem[add as usize]
    }

    fn address(&self, add: Address) -> HexSize {
        use Address::*;
        match add {
            Register(r, d) if d => self.mem_at(self.reg(r)),
            Register(r, _) => self.reg(r),
            Stack(add) => self.mem_at(add),
            Ident(sym) => self.labels.get(&sym).copied().unwrap(),
        }
    }

    fn address_mut(&mut self, add: Address) -> &mut HexSize {
        use Address::*;
        match add {
            Register(r, d) if d => self.mem_mut(self.reg(r)),
            Register(r, _) => self.reg_mut(r),
            Stack(add) => self.mem_mut(add),
            Ident(sym) => self.labels.get_mut(&sym).unwrap(),
        }
    }

    fn value(&self, value: Value) -> HexSize {
        use Value::*;
        match value {
            Address(add) => self.address(add),
            Hex(hx) => hx,
            IHex(ih) => ih as HexSize,
            // Expr(l, op, r) => {
            //     use Op::*;
            //     let l = self.value(l);
            //     let r = self.value(r);
            //     match op {
            //         Add => l + r,
            //         Sub => l - r,
            //         Div => l / r,
            //         Mul => l * r,
            //         Pow => l.pow(r as HexSize),
            //         Mod => l % r,
            //     }
            // }
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct RegisterSet {
    pub ax: HexSize,
    pub bx: HexSize,
    pub cx: HexSize,
    pub dx: HexSize,
    pub si: HexSize,
    pub di: HexSize,
    pub sp: HexSize,
    pub bp: HexSize,
    pub ip: HexSize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sequence {
    Mov(Address, Value),
    Cmp(Value, Value),
    Jmp(Value),
    Je(Value),
    Jne(Value),
    Jl(Value),
    Jle(Value),
    Jg(Value),
    Jge(Value),
    Call(DefaultSymbol),
    Ret,
    Push(Value),
    Pop(Address),
    // TODO: add bit operators: xor, and, or
    Add(Address, Value),
    Sub(Address, Value),
    Inc(Address),
    Dec(Address),
    Mul(Value),
    Div(Value),
    Mod(Value),
    // Pow(Value),
    Str(DefaultSymbol),
    Print(Address, HexSize),
    // Dyn      = allocate  dynamic
    // Down     = delete    dynamic
}

impl Sequence {
    pub fn is_jump(&self) -> bool {
        use Sequence::*;
        matches!(
            self,
            Jmp(_) | Je(_) | Jne(_) | Jl(_) | Jle(_) | Jg(_) | Jge(_)
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Address {
    Register(Register, bool),
    Stack(HexSize),
    Ident(DefaultSymbol),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    Address(Address),
    Hex(HexSize),
    IHex(IHexSize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    /// accumulator, volatile, return value
    Ax,
    /// base, stable, storage
    Bx,
    /// counter, volatile, arg 4
    Cx,
    /// data, volatile, arg 3
    Dx,
    /// source, volatile, arg 2
    Si,
    /// destination, volatile, arg 1
    Di,
    /// stack pointer, stable, stack top
    Sp,
    /// base pointer, stable, stack bot
    Bp,
    /// instruction pointer, stable, next instruction
    Ip,
}

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Add,
    Sub,
    Div,
    Mul,
    Mod,
}

impl From<Register> for Address {
    fn from(value: Register) -> Self {
        Self::Register(value, false)
    }
}

impl From<Register> for Value {
    fn from(value: Register) -> Self {
        Value::Address(value.into())
    }
}

impl From<Address> for Value {
    fn from(value: Address) -> Self {
        Value::Address(value)
    }
}

fn push(sp: &mut HexSize, mem: &mut [HexSize], word: HexSize) {
    if *sp == 0 {
        panic!("used entire available memory; underflow.")
    }
    *sp -= 1;
    mem[*sp as usize] = word;
}

fn pop(sp: &mut HexSize, mem: &mut [HexSize]) -> HexSize {
    if *sp >= HEX_MEM_SIZE {
        panic!("used entire available memory; overflow.")
    }
    let val = mem[*sp as usize];
    *sp += 1;
    val
}

#[allow(unused)]
fn copy_words(i: HexSize, mem: &mut [HexSize], words: &[HexSize]) {
    let i = i as usize;
    mem[i..i + words.len()].copy_from_slice(words);
}

pub fn mem(add: HexSize) -> Address {
    Address::Stack(add)
}

#[macro_export]
macro_rules! reg {
    ($reg:expr) => {
        reg!($reg, false)
    };
    ($reg:expr, $drf:expr) => {
        $crate::Address::Register($reg, $drf).into()
    };
}

// pub(crate) use reg;
// pub fn reg(reg: Register, deref: bool) -> Address {
//     Address::Register(reg, deref)
// }
