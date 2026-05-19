//! RV32I instruction formats and immediate helpers.

/// RV32I base instruction formats.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Format {
    R,
    I,
    S,
    B,
    U,
    J,
    System,
}

impl sw_isa_core::format::FormatInfo for Format {
    fn size_bytes(&self) -> usize {
        4
    }
}

pub const fn sign_extend(value: u32, bits: u8) -> i32 {
    debug_assert!(bits > 0 && bits <= 32);
    let shift = 32 - bits;
    ((value << shift) as i32) >> shift
}

pub const fn fits_signed(value: i32, bits: u8) -> bool {
    debug_assert!(bits > 0 && bits <= 32);
    if bits == 32 {
        return true;
    }
    let min = -(1i32 << (bits - 1));
    let max = (1i32 << (bits - 1)) - 1;
    value >= min && value <= max
}

pub const fn i_imm(word: u32) -> i32 {
    sign_extend(word >> 20, 12)
}

pub const fn s_imm(word: u32) -> i32 {
    let imm = ((word >> 20) & 0xfe0) | ((word >> 7) & 0x1f);
    sign_extend(imm, 12)
}

pub const fn b_imm(word: u32) -> i32 {
    let imm = ((word >> 19) & 0x1000)
        | ((word << 4) & 0x0800)
        | ((word >> 20) & 0x07e0)
        | ((word >> 7) & 0x001e);
    sign_extend(imm, 13)
}

pub const fn u_imm(word: u32) -> i32 {
    (word & 0xffff_f000) as i32
}

pub const fn j_imm(word: u32) -> i32 {
    let imm = ((word >> 11) & 0x100000)
        | (word & 0x000f_f000)
        | ((word >> 9) & 0x0000_0800)
        | ((word >> 20) & 0x0000_07fe);
    sign_extend(imm, 21)
}
