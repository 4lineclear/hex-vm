use std::cmp::Ordering;
use std::u16;

pub type HexSize = u16;
pub type IHexSize = i16;

pub const MEM_SIZE: usize = u16::MAX as usize;

#[derive(Debug)]
pub struct FlagSet {
    pub ord: Ordering,
    pub flo: bool,
}

#[derive(Debug)]
pub struct HexVm {
    pub flg: FlagSet,
    pub reg: RegisterSet,
    pub seq: Vec<Sequence>,
    pub mem: [HexSize; MEM_SIZE],
}

impl HexVm {
    pub fn new(seq: impl Into<Vec<Sequence>>) -> Self {
        let seq = seq.into();
        let reg = RegisterSet {
            bp: HexSize::MAX,
            sp: HexSize::MAX,
            ..Default::default()
        };
        let mem = [0; MEM_SIZE];
        let flg = FlagSet {
            ord: Ordering::Equal,
            flo: false,
        };
        Self { flg, reg, seq, mem }
    }

    pub fn run(&mut self) {
        tracing::info!("run start");
        while self.seq.len() > self.reg.ip as usize {
            self.sequence();
        }
        tracing::info!("run end");
    }

    // TODO: do overflow handling
    fn sequence(&mut self) {
        use Sequence::*;
        let old = self.reg.ip;
        let seq = self.seq[self.reg.ip as usize];
        match seq {
            Mov(add, value) => *self.address_mut(add) = self.value(value),
            Cmp(a, b) => self.flg.ord = self.value(a).cmp(&self.value(b)),
            Jmp(add) => self.reg.ip = self.address(add),
            Je(add) => self.jump_ord(add, Ordering::Equal, true),
            Jne(add) => self.jump_ord(add, Ordering::Equal, false),
            Jl(add) => self.jump_ord(add, Ordering::Less, true),
            Jle(add) => self.jump_ord(add, Ordering::Greater, false),
            Jg(add) => self.jump_ord(add, Ordering::Greater, true),
            Jge(add) => self.jump_ord(add, Ordering::Less, false),
            Push(value) => {
                self.reg.sp -= 1;
                *self.mem_mut(self.reg.sp) = self.value(value);
            }
            Pop(add) => {
                *self.address_mut(add) = self.address(register(Register::Sp, true));
                self.reg.sp += 1;
            }
            // TODO: create signed math
            Inc(add) => self.apply_math(add, 1, Op::Add),
            Dec(add) => self.apply_math(add, 1, Op::Sub),
            Add(add, value) => self.apply_math(add, self.value(value), Op::Add),
            Sub(add, value) => self.apply_math(add, self.value(value), Op::Sub),
            Mul(value) => self.reg.ax = self.math(self.reg.ax, self.value(value), Op::Mul),
            Div(value) => self.reg.ax = self.math(self.reg.ax, self.value(value), Op::Div),
            Mod(value) => self.reg.ax = self.math(self.reg.ax, self.value(value), Op::Mod),
            Pow(value) => self.reg.ax = self.math(self.reg.ax, self.value(value), Op::Pow),
            // TODO: create actual printing system
            Print(value) => {
                let [a, b] = self.value(value).to_be_bytes();
                print!("{}{}", a as char, b as char)
            }
        }
        tracing::info!("exec {}", self.reg.ip);
        tracing::info!("{seq:?}");
        tracing::info!("{:?}", self.reg);
        tracing::info!("{:?}", self.flg);
        tracing::info!("end  {}, mid-change {}", self.reg.ip, (old != self.reg.ip));
        self.reg.ip += (old == self.reg.ip) as u16;
    }

    fn jump_ord(&mut self, add: Address, ord: Ordering, eq: bool) {
        self.reg.ip = if eq == (ord == self.flg.ord) {
            self.address(add)
        } else {
            self.reg.ip
        };
    }

    fn apply_math(&mut self, add: Address, b: HexSize, op: Op) {
        *self.address_mut(add) = self.math(self.address(add), b, op);
    }

    fn math(&mut self, a: HexSize, b: HexSize, op: Op) -> HexSize {
        self.flg.ord = a.cmp(&b);
        let (v, flo) = match op {
            Op::Add => a.overflowing_add(b),
            Op::Sub => a.overflowing_sub(b),
            Op::Div => a.overflowing_div(b),
            Op::Mul => a.overflowing_mul(b),
            Op::Pow => a.overflowing_pow(u32::from(b)),
            Op::Mod => (a % b, false),
        };
        self.flg.flo = flo;
        v
    }

    #[allow(unused)]
    fn imath(&mut self, a: HexSize, b: HexSize, op: Op) -> HexSize {
        let a = a as IHexSize;
        let b = b as IHexSize;
        self.flg.ord = a.cmp(&b);
        let (v, flo) = match op {
            Op::Add => a.overflowing_add(b),
            Op::Sub => a.overflowing_sub(b),
            Op::Div => a.overflowing_div(b),
            Op::Mul => a.overflowing_mul(b),
            Op::Pow => a.overflowing_pow(u32::from(b as HexSize)),
            Op::Mod => (a % b, false),
        };
        self.flg.flo = flo;
        v as HexSize
    }

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
        }
    }

    fn address_mut(&mut self, add: Address) -> &mut HexSize {
        use Address::*;
        match add {
            Register(r, d) if d => self.mem_mut(self.reg(r)),
            Register(r, _) => self.reg_mut(r),
            Stack(add) => self.mem_mut(add),
        }
    }

    fn value(&self, value: Value) -> HexSize {
        use Value::*;
        match value {
            Address(add) => self.address(add),
            Hex(hx) => hx,
            IHex(ih) => ih as u16,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub enum Sequence {
    Mov(Address, Value),
    Cmp(Value, Value),
    Jmp(Address),
    Je(Address),
    Jne(Address),
    Jl(Address),
    Jle(Address),
    Jg(Address),
    Jge(Address),
    Push(Value),
    Pop(Address),
    Add(Address, Value),
    Sub(Address, Value),
    Inc(Address),
    Dec(Address),
    Mul(Value),
    Div(Value),
    Mod(Value),
    Pow(Value),
    Print(Value),
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

#[derive(Debug, Clone, Copy)]
pub enum Address {
    Register(Register, bool),
    Stack(HexSize),
}

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Address(Address),
    Hex(HexSize),
    IHex(IHexSize),
}

#[derive(Debug, Clone, Copy)]
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

enum Op {
    Add,
    Sub,
    Div,
    Mul,
    Pow,
    Mod,
}

pub fn stack(add: HexSize) -> Address {
    Address::Stack(add)
}

pub fn register(reg: Register, deref: bool) -> Address {
    Address::Register(reg, deref)
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
