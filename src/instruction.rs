//! Semantic RV32I instruction model.

use crate::{Opcode, Reg};

/// Decoded RV32I instruction with semantic operands.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    Lui {
        rd: Reg,
        imm: i32,
    },
    Auipc {
        rd: Reg,
        imm: i32,
    },
    Jal {
        rd: Reg,
        offset: i32,
    },
    Jalr {
        rd: Reg,
        rs1: Reg,
        offset: i32,
    },
    Branch {
        cond: BranchCond,
        rs1: Reg,
        rs2: Reg,
        offset: i32,
    },
    Load {
        width: LoadWidth,
        rd: Reg,
        rs1: Reg,
        offset: i32,
    },
    Store {
        width: StoreWidth,
        rs1: Reg,
        rs2: Reg,
        offset: i32,
    },
    OpImm {
        op: ImmOp,
        rd: Reg,
        rs1: Reg,
        imm: i32,
    },
    Op {
        op: RegOp,
        rd: Reg,
        rs1: Reg,
        rs2: Reg,
    },
    Fence {
        predecessor: FenceSet,
        successor: FenceSet,
    },
    Ecall,
    Ebreak,
}

impl Instruction {
    pub fn opcode(self) -> Opcode {
        match self {
            Instruction::Lui { .. } => Opcode::Lui,
            Instruction::Auipc { .. } => Opcode::Auipc,
            Instruction::Jal { .. } => Opcode::Jal,
            Instruction::Jalr { .. } => Opcode::Jalr,
            Instruction::Branch { cond, .. } => cond.opcode(),
            Instruction::Load { width, .. } => width.opcode(),
            Instruction::Store { width, .. } => width.opcode(),
            Instruction::OpImm { op, .. } => op.opcode(),
            Instruction::Op { op, .. } => op.opcode(),
            Instruction::Fence { .. } => Opcode::Fence,
            Instruction::Ecall => Opcode::Ecall,
            Instruction::Ebreak => Opcode::Ebreak,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BranchCond {
    Eq,
    Ne,
    Lt,
    Ge,
    Ltu,
    Geu,
}

impl BranchCond {
    pub fn opcode(self) -> Opcode {
        match self {
            BranchCond::Eq => Opcode::Beq,
            BranchCond::Ne => Opcode::Bne,
            BranchCond::Lt => Opcode::Blt,
            BranchCond::Ge => Opcode::Bge,
            BranchCond::Ltu => Opcode::Bltu,
            BranchCond::Geu => Opcode::Bgeu,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LoadWidth {
    Byte,
    Half,
    Word,
    ByteUnsigned,
    HalfUnsigned,
}

impl LoadWidth {
    pub fn bytes(self) -> usize {
        match self {
            LoadWidth::Byte | LoadWidth::ByteUnsigned => 1,
            LoadWidth::Half | LoadWidth::HalfUnsigned => 2,
            LoadWidth::Word => 4,
        }
    }

    pub fn is_signed(self) -> bool {
        matches!(self, LoadWidth::Byte | LoadWidth::Half | LoadWidth::Word)
    }

    pub fn opcode(self) -> Opcode {
        match self {
            LoadWidth::Byte => Opcode::Lb,
            LoadWidth::Half => Opcode::Lh,
            LoadWidth::Word => Opcode::Lw,
            LoadWidth::ByteUnsigned => Opcode::Lbu,
            LoadWidth::HalfUnsigned => Opcode::Lhu,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StoreWidth {
    Byte,
    Half,
    Word,
}

impl StoreWidth {
    pub fn bytes(self) -> usize {
        match self {
            StoreWidth::Byte => 1,
            StoreWidth::Half => 2,
            StoreWidth::Word => 4,
        }
    }

    pub fn opcode(self) -> Opcode {
        match self {
            StoreWidth::Byte => Opcode::Sb,
            StoreWidth::Half => Opcode::Sh,
            StoreWidth::Word => Opcode::Sw,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ImmOp {
    Addi,
    Slti,
    Sltiu,
    Xori,
    Ori,
    Andi,
    Slli,
    Srli,
    Srai,
}

impl ImmOp {
    pub fn opcode(self) -> Opcode {
        match self {
            ImmOp::Addi => Opcode::Addi,
            ImmOp::Slti => Opcode::Slti,
            ImmOp::Sltiu => Opcode::Sltiu,
            ImmOp::Xori => Opcode::Xori,
            ImmOp::Ori => Opcode::Ori,
            ImmOp::Andi => Opcode::Andi,
            ImmOp::Slli => Opcode::Slli,
            ImmOp::Srli => Opcode::Srli,
            ImmOp::Srai => Opcode::Srai,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RegOp {
    Add,
    Sub,
    Sll,
    Slt,
    Sltu,
    Xor,
    Srl,
    Sra,
    Or,
    And,
}

impl RegOp {
    pub fn opcode(self) -> Opcode {
        match self {
            RegOp::Add => Opcode::Add,
            RegOp::Sub => Opcode::Sub,
            RegOp::Sll => Opcode::Sll,
            RegOp::Slt => Opcode::Slt,
            RegOp::Sltu => Opcode::Sltu,
            RegOp::Xor => Opcode::Xor,
            RegOp::Srl => Opcode::Srl,
            RegOp::Sra => Opcode::Sra,
            RegOp::Or => Opcode::Or,
            RegOp::And => Opcode::And,
        }
    }
}

/// FENCE predecessor/successor set bits in `iorw` order.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FenceSet(u8);

impl FenceSet {
    pub const NONE: Self = Self(0);
    pub const I: Self = Self(0b1000);
    pub const O: Self = Self(0b0100);
    pub const R: Self = Self(0b0010);
    pub const W: Self = Self(0b0001);
    pub const IORW: Self = Self(0b1111);

    pub const fn from_bits(bits: u8) -> Option<Self> {
        if bits <= 0b1111 {
            Some(Self(bits))
        } else {
            None
        }
    }

    pub const fn bits(self) -> u8 {
        self.0
    }

    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}
