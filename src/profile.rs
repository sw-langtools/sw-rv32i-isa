//! RISC-V ISA profile and legality helpers.

use crate::{Instruction, Reg};

/// RV32 base integer register profile.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BaseIsa {
    /// RV32E base ISA with registers x0..x15.
    Rv32e,
    /// RV32I base ISA with registers x0..x31.
    Rv32i,
}

impl BaseIsa {
    pub const fn name(self) -> &'static str {
        match self {
            BaseIsa::Rv32e => "rv32e",
            BaseIsa::Rv32i => "rv32i",
        }
    }

    pub const fn max_register_index(self) -> u8 {
        match self {
            BaseIsa::Rv32e => 15,
            BaseIsa::Rv32i => 31,
        }
    }
}

/// Optional RISC-V ISA extension.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Extension {
    /// Integer multiply/divide extension.
    M,
}

impl Extension {
    const fn bit(self) -> u32 {
        match self {
            Extension::M => 1 << 0,
        }
    }
}

/// A target ISA profile, such as `rv32i` or `rv32im`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct IsaProfile {
    base: BaseIsa,
    extensions: u32,
}

impl IsaProfile {
    pub const RV32E: Self = Self::new(BaseIsa::Rv32e, &[]);
    pub const RV32I: Self = Self::new(BaseIsa::Rv32i, &[]);
    pub const RV32IM: Self = Self::new(BaseIsa::Rv32i, &[Extension::M]);

    pub const fn new(base: BaseIsa, extensions: &[Extension]) -> Self {
        let mut bits = 0;
        let mut index = 0;
        while index < extensions.len() {
            bits |= extensions[index].bit();
            index += 1;
        }
        Self {
            base,
            extensions: bits,
        }
    }

    pub const fn base(self) -> BaseIsa {
        self.base
    }

    pub const fn has_extension(self, extension: Extension) -> bool {
        self.extensions & extension.bit() != 0
    }

    pub const fn supports_register(self, reg: Reg) -> bool {
        reg.index_u8() <= self.base.max_register_index()
    }

    pub fn validate_register(self, reg: Reg) -> Result<(), ProfileError> {
        if self.supports_register(reg) {
            Ok(())
        } else {
            Err(ProfileError::IllegalRegister { reg, profile: self })
        }
    }

    pub fn validate_instruction(self, insn: Instruction) -> Result<(), ProfileError> {
        match insn {
            Instruction::Lui { rd, .. }
            | Instruction::Auipc { rd, .. }
            | Instruction::Jal { rd, .. } => self.validate_register(rd),
            Instruction::Jalr { rd, rs1, .. } | Instruction::Load { rd, rs1, .. } => self
                .validate_register(rd)
                .and_then(|()| self.validate_register(rs1)),
            Instruction::Branch { rs1, rs2, .. } | Instruction::Store { rs1, rs2, .. } => self
                .validate_register(rs1)
                .and_then(|()| self.validate_register(rs2)),
            Instruction::OpImm { rd, rs1, .. } => self
                .validate_register(rd)
                .and_then(|()| self.validate_register(rs1)),
            Instruction::Op { rd, rs1, rs2, .. } => self
                .validate_register(rd)
                .and_then(|()| self.validate_register(rs1))
                .and_then(|()| self.validate_register(rs2)),
            Instruction::Fence { .. } | Instruction::Ecall | Instruction::Ebreak => Ok(()),
        }
    }
}

/// A profile legality failure.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ProfileError {
    IllegalRegister { reg: Reg, profile: IsaProfile },
}
