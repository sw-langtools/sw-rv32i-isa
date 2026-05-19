//! RV32I integer register identifiers and parser.

use core::fmt;
use core::str::FromStr;

/// RV32I integer register `x0..x31`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Reg(u8);

impl Reg {
    pub const X0: Self = Self(0);
    pub const X1: Self = Self(1);
    pub const X2: Self = Self(2);
    pub const X3: Self = Self(3);
    pub const X4: Self = Self(4);
    pub const X5: Self = Self(5);
    pub const X6: Self = Self(6);
    pub const X7: Self = Self(7);
    pub const X8: Self = Self(8);
    pub const X9: Self = Self(9);
    pub const X10: Self = Self(10);
    pub const X11: Self = Self(11);
    pub const X12: Self = Self(12);
    pub const X13: Self = Self(13);
    pub const X14: Self = Self(14);
    pub const X15: Self = Self(15);
    pub const X16: Self = Self(16);
    pub const X17: Self = Self(17);
    pub const X18: Self = Self(18);
    pub const X19: Self = Self(19);
    pub const X20: Self = Self(20);
    pub const X21: Self = Self(21);
    pub const X22: Self = Self(22);
    pub const X23: Self = Self(23);
    pub const X24: Self = Self(24);
    pub const X25: Self = Self(25);
    pub const X26: Self = Self(26);
    pub const X27: Self = Self(27);
    pub const X28: Self = Self(28);
    pub const X29: Self = Self(29);
    pub const X30: Self = Self(30);
    pub const X31: Self = Self(31);

    pub const fn new(index: u8) -> Option<Self> {
        if index < 32 { Some(Self(index)) } else { None }
    }

    pub const fn index_u8(self) -> u8 {
        self.0
    }

    pub fn name(self) -> &'static str {
        REGISTER_NAMES[self.0 as usize]
    }

    pub fn abi_name(self) -> &'static str {
        ABI_NAMES[self.0 as usize]
    }
}

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl FromStr for Reg {
    type Err = ParseRegisterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_register(s).ok_or(ParseRegisterError)
    }
}

impl sw_isa_core::register::RegisterId for Reg {
    fn index(self) -> u32 {
        self.0 as u32
    }

    fn name(self) -> &'static str {
        Reg::name(self)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ParseRegisterError;

const REGISTER_NAMES: [&str; 32] = [
    "x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7", "x8", "x9", "x10", "x11", "x12", "x13", "x14",
    "x15", "x16", "x17", "x18", "x19", "x20", "x21", "x22", "x23", "x24", "x25", "x26", "x27",
    "x28", "x29", "x30", "x31",
];

const ABI_NAMES: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4",
    "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
    "t5", "t6",
];

pub fn parse_register(s: &str) -> Option<Reg> {
    parse_canonical_register(s).or_else(|| parse_abi_register(s))
}

fn parse_canonical_register(s: &str) -> Option<Reg> {
    let rest = s.strip_prefix('x').or_else(|| s.strip_prefix('X'))?;
    let index = parse_decimal_u8(rest)?;
    Reg::new(index)
}

fn parse_abi_register(s: &str) -> Option<Reg> {
    ABI_NAMES
        .iter()
        .position(|name| *name == s)
        .and_then(|index| Reg::new(index as u8))
}

fn parse_decimal_u8(s: &str) -> Option<u8> {
    if s.is_empty() || (s.len() > 1 && s.as_bytes()[0] == b'0') {
        return None;
    }

    let mut value: u16 = 0;
    for b in s.bytes() {
        if !b.is_ascii_digit() {
            return None;
        }
        value = value * 10 + (b - b'0') as u16;
        if value > u8::MAX as u16 {
            return None;
        }
    }
    Some(value as u8)
}
