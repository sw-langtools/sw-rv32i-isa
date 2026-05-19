//! RV32I decoder.

use sw_isa_core::DecodeError;

use crate::{
    BranchCond, FenceSet, ImmOp, Instruction, LoadWidth, Reg, RegOp, StoreWidth, b_imm, i_imm,
    j_imm, s_imm, u_imm,
};

pub fn decode_word(word: u32) -> Result<Instruction, DecodeError> {
    match opcode(word) {
        0b0110111 => Ok(Instruction::Lui {
            rd: rd(word),
            imm: u_imm(word),
        }),
        0b0010111 => Ok(Instruction::Auipc {
            rd: rd(word),
            imm: u_imm(word),
        }),
        0b1101111 => Ok(Instruction::Jal {
            rd: rd(word),
            offset: j_imm(word),
        }),
        0b1100111 => decode_jalr(word),
        0b1100011 => decode_branch(word),
        0b0000011 => decode_load(word),
        0b0100011 => decode_store(word),
        0b0010011 => decode_op_imm(word),
        0b0110011 => decode_op(word),
        0b0001111 => decode_fence(word),
        0b1110011 => decode_system(word),
        _ => Err(DecodeError::Invalid),
    }
}

fn decode_jalr(word: u32) -> Result<Instruction, DecodeError> {
    if funct3(word) != 0 {
        return Err(DecodeError::Invalid);
    }
    Ok(Instruction::Jalr {
        rd: rd(word),
        rs1: rs1(word),
        offset: i_imm(word),
    })
}

fn decode_branch(word: u32) -> Result<Instruction, DecodeError> {
    let cond = match funct3(word) {
        0b000 => BranchCond::Eq,
        0b001 => BranchCond::Ne,
        0b100 => BranchCond::Lt,
        0b101 => BranchCond::Ge,
        0b110 => BranchCond::Ltu,
        0b111 => BranchCond::Geu,
        _ => return Err(DecodeError::Invalid),
    };
    Ok(Instruction::Branch {
        cond,
        rs1: rs1(word),
        rs2: rs2(word),
        offset: b_imm(word),
    })
}

fn decode_load(word: u32) -> Result<Instruction, DecodeError> {
    let width = match funct3(word) {
        0b000 => LoadWidth::Byte,
        0b001 => LoadWidth::Half,
        0b010 => LoadWidth::Word,
        0b100 => LoadWidth::ByteUnsigned,
        0b101 => LoadWidth::HalfUnsigned,
        _ => return Err(DecodeError::Invalid),
    };
    Ok(Instruction::Load {
        width,
        rd: rd(word),
        rs1: rs1(word),
        offset: i_imm(word),
    })
}

fn decode_store(word: u32) -> Result<Instruction, DecodeError> {
    let width = match funct3(word) {
        0b000 => StoreWidth::Byte,
        0b001 => StoreWidth::Half,
        0b010 => StoreWidth::Word,
        _ => return Err(DecodeError::Invalid),
    };
    Ok(Instruction::Store {
        width,
        rs1: rs1(word),
        rs2: rs2(word),
        offset: s_imm(word),
    })
}

fn decode_op_imm(word: u32) -> Result<Instruction, DecodeError> {
    let op = match funct3(word) {
        0b000 => ImmOp::Addi,
        0b010 => ImmOp::Slti,
        0b011 => ImmOp::Sltiu,
        0b100 => ImmOp::Xori,
        0b110 => ImmOp::Ori,
        0b111 => ImmOp::Andi,
        0b001 if funct7(word) == 0b0000000 => ImmOp::Slli,
        0b101 if funct7(word) == 0b0000000 => ImmOp::Srli,
        0b101 if funct7(word) == 0b0100000 => ImmOp::Srai,
        _ => return Err(DecodeError::Invalid),
    };
    let imm = if matches!(op, ImmOp::Slli | ImmOp::Srli | ImmOp::Srai) {
        shamt(word) as i32
    } else {
        i_imm(word)
    };
    Ok(Instruction::OpImm {
        op,
        rd: rd(word),
        rs1: rs1(word),
        imm,
    })
}

fn decode_op(word: u32) -> Result<Instruction, DecodeError> {
    let op = match (funct3(word), funct7(word)) {
        (0b000, 0b0000000) => RegOp::Add,
        (0b000, 0b0100000) => RegOp::Sub,
        (0b001, 0b0000000) => RegOp::Sll,
        (0b010, 0b0000000) => RegOp::Slt,
        (0b011, 0b0000000) => RegOp::Sltu,
        (0b100, 0b0000000) => RegOp::Xor,
        (0b101, 0b0000000) => RegOp::Srl,
        (0b101, 0b0100000) => RegOp::Sra,
        (0b110, 0b0000000) => RegOp::Or,
        (0b111, 0b0000000) => RegOp::And,
        _ => return Err(DecodeError::Invalid),
    };
    Ok(Instruction::Op {
        op,
        rd: rd(word),
        rs1: rs1(word),
        rs2: rs2(word),
    })
}

fn decode_fence(word: u32) -> Result<Instruction, DecodeError> {
    if funct3(word) != 0 || rd(word) != Reg::X0 || rs1(word) != Reg::X0 {
        return Err(DecodeError::Invalid);
    }
    let fm = ((word >> 28) & 0x0f) as u8;
    if fm != 0 {
        return Err(DecodeError::Invalid);
    }
    Ok(Instruction::Fence {
        predecessor: FenceSet::from_bits(((word >> 24) & 0x0f) as u8).unwrap(),
        successor: FenceSet::from_bits(((word >> 20) & 0x0f) as u8).unwrap(),
    })
}

fn decode_system(word: u32) -> Result<Instruction, DecodeError> {
    match word {
        0x0000_0073 => Ok(Instruction::Ecall),
        0x0010_0073 => Ok(Instruction::Ebreak),
        _ => Err(DecodeError::Invalid),
    }
}

fn opcode(word: u32) -> u32 {
    word & 0x7f
}

fn rd(word: u32) -> Reg {
    Reg::new(((word >> 7) & 0x1f) as u8).unwrap()
}

fn funct3(word: u32) -> u32 {
    (word >> 12) & 0x07
}

fn rs1(word: u32) -> Reg {
    Reg::new(((word >> 15) & 0x1f) as u8).unwrap()
}

fn rs2(word: u32) -> Reg {
    Reg::new(((word >> 20) & 0x1f) as u8).unwrap()
}

fn funct7(word: u32) -> u32 {
    (word >> 25) & 0x7f
}

fn shamt(word: u32) -> u8 {
    ((word >> 20) & 0x1f) as u8
}
