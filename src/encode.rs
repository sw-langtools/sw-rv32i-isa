//! RV32I encoder.

use sw_isa_core::EncodeError;

use crate::{BranchCond, ImmOp, Instruction, LoadWidth, Reg, RegOp, StoreWidth, fits_signed};

pub fn encode_word(insn: Instruction) -> Result<u32, EncodeError> {
    match insn {
        Instruction::Lui { rd, imm } => encode_u(imm, rd, 0b0110111),
        Instruction::Auipc { rd, imm } => encode_u(imm, rd, 0b0010111),
        Instruction::Jal { rd, offset } => encode_j(offset, rd, 0b1101111),
        Instruction::Jalr { rd, rs1, offset } => encode_i(offset, rs1, 0b000, rd, 0b1100111),
        Instruction::Branch {
            cond,
            rs1,
            rs2,
            offset,
        } => encode_b(offset, rs1, rs2, branch_funct3(cond)),
        Instruction::Load {
            width,
            rd,
            rs1,
            offset,
        } => encode_i(offset, rs1, load_funct3(width), rd, 0b0000011),
        Instruction::Store {
            width,
            rs1,
            rs2,
            offset,
        } => encode_s(offset, rs1, rs2, store_funct3(width)),
        Instruction::OpImm { op, rd, rs1, imm } => encode_op_imm(op, rd, rs1, imm),
        Instruction::Op { op, rd, rs1, rs2 } => {
            let (funct7, funct3) = reg_op_fields(op);
            Ok(r_type(funct7, rs2, rs1, funct3, rd))
        }
        Instruction::Fence {
            predecessor,
            successor,
        } => {
            Ok(((predecessor.bits() as u32) << 24) | ((successor.bits() as u32) << 20) | 0b0001111)
        }
        Instruction::Ecall => Ok(0x0000_0073),
        Instruction::Ebreak => Ok(0x0010_0073),
    }
}

fn encode_op_imm(op: ImmOp, rd: Reg, rs1: Reg, imm: i32) -> Result<u32, EncodeError> {
    let (funct7, funct3) = match op {
        ImmOp::Addi => return encode_i(imm, rs1, 0b000, rd, 0b0010011),
        ImmOp::Slti => return encode_i(imm, rs1, 0b010, rd, 0b0010011),
        ImmOp::Sltiu => return encode_i(imm, rs1, 0b011, rd, 0b0010011),
        ImmOp::Xori => return encode_i(imm, rs1, 0b100, rd, 0b0010011),
        ImmOp::Ori => return encode_i(imm, rs1, 0b110, rd, 0b0010011),
        ImmOp::Andi => return encode_i(imm, rs1, 0b111, rd, 0b0010011),
        ImmOp::Slli => (0b0000000, 0b001),
        ImmOp::Srli => (0b0000000, 0b101),
        ImmOp::Srai => (0b0100000, 0b101),
    };
    if !(0..=31).contains(&imm) {
        return Err(EncodeError::InvalidOperands);
    }
    Ok((funct7 << 25)
        | ((imm as u32) << 20)
        | ((rs1.index_u8() as u32) << 15)
        | (funct3 << 12)
        | ((rd.index_u8() as u32) << 7)
        | 0b0010011)
}

fn encode_i(imm: i32, rs1: Reg, funct3: u32, rd: Reg, opcode: u32) -> Result<u32, EncodeError> {
    if !fits_signed(imm, 12) {
        return Err(EncodeError::InvalidOperands);
    }
    Ok(((imm as u32 & 0x0fff) << 20)
        | ((rs1.index_u8() as u32) << 15)
        | (funct3 << 12)
        | ((rd.index_u8() as u32) << 7)
        | opcode)
}

fn encode_s(imm: i32, rs1: Reg, rs2: Reg, funct3: u32) -> Result<u32, EncodeError> {
    if !fits_signed(imm, 12) {
        return Err(EncodeError::InvalidOperands);
    }
    let imm = imm as u32 & 0x0fff;
    Ok(((imm >> 5) << 25)
        | ((rs2.index_u8() as u32) << 20)
        | ((rs1.index_u8() as u32) << 15)
        | (funct3 << 12)
        | ((imm & 0x1f) << 7)
        | 0b0100011)
}

fn encode_b(imm: i32, rs1: Reg, rs2: Reg, funct3: u32) -> Result<u32, EncodeError> {
    if !fits_signed(imm, 13) || imm & 0x1 != 0 {
        return Err(EncodeError::InvalidOperands);
    }
    let imm = imm as u32 & 0x1fff;
    Ok(((imm >> 12) << 31)
        | (((imm >> 5) & 0x3f) << 25)
        | ((rs2.index_u8() as u32) << 20)
        | ((rs1.index_u8() as u32) << 15)
        | (funct3 << 12)
        | (((imm >> 1) & 0x0f) << 8)
        | (((imm >> 11) & 0x01) << 7)
        | 0b1100011)
}

fn encode_u(imm: i32, rd: Reg, opcode: u32) -> Result<u32, EncodeError> {
    if imm & 0x0fff != 0 {
        return Err(EncodeError::InvalidOperands);
    }
    Ok((imm as u32 & 0xffff_f000) | ((rd.index_u8() as u32) << 7) | opcode)
}

fn encode_j(imm: i32, rd: Reg, opcode: u32) -> Result<u32, EncodeError> {
    if !fits_signed(imm, 21) || imm & 0x1 != 0 {
        return Err(EncodeError::InvalidOperands);
    }
    let imm = imm as u32 & 0x1f_ffff;
    Ok(((imm >> 20) << 31)
        | (((imm >> 1) & 0x03ff) << 21)
        | (((imm >> 11) & 0x01) << 20)
        | (((imm >> 12) & 0x0ff) << 12)
        | ((rd.index_u8() as u32) << 7)
        | opcode)
}

fn r_type(funct7: u32, rs2: Reg, rs1: Reg, funct3: u32, rd: Reg) -> u32 {
    (funct7 << 25)
        | ((rs2.index_u8() as u32) << 20)
        | ((rs1.index_u8() as u32) << 15)
        | (funct3 << 12)
        | ((rd.index_u8() as u32) << 7)
        | 0b0110011
}

fn branch_funct3(cond: BranchCond) -> u32 {
    match cond {
        BranchCond::Eq => 0b000,
        BranchCond::Ne => 0b001,
        BranchCond::Lt => 0b100,
        BranchCond::Ge => 0b101,
        BranchCond::Ltu => 0b110,
        BranchCond::Geu => 0b111,
    }
}

fn load_funct3(width: LoadWidth) -> u32 {
    match width {
        LoadWidth::Byte => 0b000,
        LoadWidth::Half => 0b001,
        LoadWidth::Word => 0b010,
        LoadWidth::ByteUnsigned => 0b100,
        LoadWidth::HalfUnsigned => 0b101,
    }
}

fn store_funct3(width: StoreWidth) -> u32 {
    match width {
        StoreWidth::Byte => 0b000,
        StoreWidth::Half => 0b001,
        StoreWidth::Word => 0b010,
    }
}

fn reg_op_fields(op: RegOp) -> (u32, u32) {
    match op {
        RegOp::Add => (0b0000000, 0b000),
        RegOp::Sub => (0b0100000, 0b000),
        RegOp::Sll => (0b0000000, 0b001),
        RegOp::Slt => (0b0000000, 0b010),
        RegOp::Sltu => (0b0000000, 0b011),
        RegOp::Xor => (0b0000000, 0b100),
        RegOp::Srl => (0b0000000, 0b101),
        RegOp::Sra => (0b0100000, 0b101),
        RegOp::Or => (0b0000000, 0b110),
        RegOp::And => (0b0000000, 0b111),
    }
}
