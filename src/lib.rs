//! `sw-rv32i-isa`: RV32I ISA description: opcodes, encoding, decoding,
//! disassembly.
//!
//! The concrete register model, instruction model, decoder, encoder, and
//! disassembler are added in follow-up saga steps.

#![no_std]

/// RV32I byte address.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Addr(pub u32);

impl sw_isa_core::address::AddressType for Addr {
    fn to_u64(self) -> u64 {
        self.0 as u64
    }

    fn from_u64(v: u64) -> Self {
        Addr(v as u32)
    }

    fn step(self, n: i64) -> Self {
        Addr((self.0 as i64 + n) as u32)
    }
}

/// RV32I architecture marker.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Rv32i;

/// Placeholder opcode used until the instruction set model is introduced.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Opcode {
    Invalid,
}

impl sw_isa_core::Mnemonic for Opcode {
    fn mnemonic(&self) -> &'static str {
        match self {
            Opcode::Invalid => "invalid",
        }
    }
}

/// Placeholder register used until the RV32I register model is introduced.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Reg {
    X0,
}

impl sw_isa_core::register::RegisterId for Reg {
    fn index(self) -> u32 {
        match self {
            Reg::X0 => 0,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Reg::X0 => "x0",
        }
    }
}

/// Placeholder instruction used until the RV32I instruction enum is introduced.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    Invalid,
}

/// Placeholder fixed 32-bit instruction format.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Format {
    Fixed32,
}

impl sw_isa_core::format::FormatInfo for Format {
    fn size_bytes(&self) -> usize {
        match self {
            Format::Fixed32 => 4,
        }
    }
}

impl sw_isa_core::Architecture for Rv32i {
    type Opcode = Opcode;
    type Register = Reg;
    type Instruction = Instruction;
    type Address = Addr;
    type Format = Format;

    const NAME: &'static str = "RV32I";
    const ENDIAN: sw_isa_core::endian::Endian = sw_isa_core::endian::Endian::Little;
    const ADDRESS_UNIT: sw_isa_core::address::AddressUnit = sw_isa_core::address::AddressUnit::Byte;
    const WORD_BITS: u32 = 32;
    const MAX_INSTR_BYTES: usize = 4;
    const MIN_INSTR_BYTES: usize = 4;

    fn decode(
        bytes: &[u8],
        _pc: Self::Address,
    ) -> Result<(Self::Instruction, usize), sw_isa_core::DecodeError> {
        if bytes.len() < Self::MIN_INSTR_BYTES {
            return Err(sw_isa_core::DecodeError::Truncated);
        }

        Err(sw_isa_core::DecodeError::Invalid)
    }

    fn encode(
        _insn: &Self::Instruction,
        _out: &mut [u8],
    ) -> Result<usize, sw_isa_core::EncodeError> {
        Err(sw_isa_core::EncodeError::InvalidOperands)
    }

    fn disassemble(_insn: &Self::Instruction, w: &mut dyn core::fmt::Write) -> core::fmt::Result {
        w.write_str("invalid")
    }
}
