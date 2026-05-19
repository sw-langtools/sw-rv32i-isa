use sw_isa_core::Architecture;
use sw_rv32i_isa::{
    Addr, BranchCond, FenceSet, ImmOp, Instruction, LoadWidth, Reg, RegOp, Rv32i, StoreWidth,
    decode_word,
};

#[test]
fn architecture_decode_reads_little_endian_words() {
    assert_eq!(
        Rv32i::decode(&0x1234_50b7u32.to_le_bytes(), Addr(0)),
        Ok((
            Instruction::Lui {
                rd: Reg::X1,
                imm: 0x1234_5000,
            },
            4
        ))
    );
    assert_eq!(
        Rv32i::decode(&[0; 3], Addr(0)),
        Err(sw_isa_core::DecodeError::Truncated)
    );
}

#[test]
fn decodes_u_and_j_formats() {
    assert_eq!(
        decode_word(0xffff_f297),
        Ok(Instruction::Auipc {
            rd: Reg::X5,
            imm: -4096,
        })
    );
    assert_eq!(
        decode_word(j_type(-4, 1, 0b1101111)),
        Ok(Instruction::Jal {
            rd: Reg::X1,
            offset: -4,
        })
    );
}

#[test]
fn decodes_branches_with_signed_offsets() {
    assert_eq!(
        decode_word(b_type(-4, Reg::X1, Reg::X2, 0b101)),
        Ok(Instruction::Branch {
            cond: BranchCond::Ge,
            rs1: Reg::X1,
            rs2: Reg::X2,
            offset: -4,
        })
    );
    assert_eq!(
        decode_word(b_type(16, Reg::X3, Reg::X4, 0b110)),
        Ok(Instruction::Branch {
            cond: BranchCond::Ltu,
            rs1: Reg::X3,
            rs2: Reg::X4,
            offset: 16,
        })
    );
}

#[test]
fn decodes_jalr_loads_and_stores() {
    assert_eq!(
        decode_word(i_type(-8, Reg::X2, 0b000, Reg::X1, 0b1100111)),
        Ok(Instruction::Jalr {
            rd: Reg::X1,
            rs1: Reg::X2,
            offset: -8,
        })
    );
    assert_eq!(
        decode_word(i_type(12, Reg::X2, 0b101, Reg::X3, 0b0000011)),
        Ok(Instruction::Load {
            width: LoadWidth::HalfUnsigned,
            rd: Reg::X3,
            rs1: Reg::X2,
            offset: 12,
        })
    );
    assert_eq!(
        decode_word(s_type(-16, Reg::X2, Reg::X8, 0b010)),
        Ok(Instruction::Store {
            width: StoreWidth::Word,
            rs1: Reg::X2,
            rs2: Reg::X8,
            offset: -16,
        })
    );
}

#[test]
fn decodes_immediate_and_register_arithmetic() {
    assert_eq!(
        decode_word(i_type(-1, Reg::X6, 0b110, Reg::X5, 0b0010011)),
        Ok(Instruction::OpImm {
            op: ImmOp::Ori,
            rd: Reg::X5,
            rs1: Reg::X6,
            imm: -1,
        })
    );
    assert_eq!(
        decode_word(shift_imm(0b0100000, 7, Reg::X8, 0b101, Reg::X9)),
        Ok(Instruction::OpImm {
            op: ImmOp::Srai,
            rd: Reg::X9,
            rs1: Reg::X8,
            imm: 7,
        })
    );
    assert_eq!(
        decode_word(r_type(0b0100000, Reg::X12, Reg::X11, 0b000, Reg::X10)),
        Ok(Instruction::Op {
            op: RegOp::Sub,
            rd: Reg::X10,
            rs1: Reg::X11,
            rs2: Reg::X12,
        })
    );
}

#[test]
fn decodes_fence_and_system() {
    let fence = (FenceSet::R.bits() as u32) << 24 | (FenceSet::W.bits() as u32) << 20 | 0b0001111;
    assert_eq!(
        decode_word(fence),
        Ok(Instruction::Fence {
            predecessor: FenceSet::R,
            successor: FenceSet::W,
        })
    );
    assert_eq!(decode_word(0x0000_0073), Ok(Instruction::Ecall));
    assert_eq!(decode_word(0x0010_0073), Ok(Instruction::Ebreak));
}

#[test]
fn rejects_reserved_or_malformed_encodings() {
    for word in [
        0,
        i_type(0, Reg::X1, 0b010, Reg::X2, 0b1100111),
        b_type(4, Reg::X1, Reg::X2, 0b010),
        i_type(0, Reg::X1, 0b011, Reg::X2, 0b0000011),
        s_type(0, Reg::X1, Reg::X2, 0b011),
        shift_imm(0b0000001, 0, Reg::X1, 0b001, Reg::X2),
        r_type(0b0000001, Reg::X1, Reg::X2, 0b000, Reg::X3),
        0x0000_1073,
        0x0020_0073,
    ] {
        assert_eq!(decode_word(word), Err(sw_isa_core::DecodeError::Invalid));
    }
}

#[test]
fn rejects_invalid_base_opcodes() {
    for word in [
        0x0000_0000,
        0x0000_0001,
        0x0000_0002,
        0x0000_000b,
        0x0000_002b,
        0x0000_005b,
        0xffff_ffff,
    ] {
        assert_eq!(decode_word(word), Err(sw_isa_core::DecodeError::Invalid));
    }
}

#[test]
fn rejects_unsupported_funct3_combinations() {
    for word in [
        i_type(0, Reg::X1, 0b001, Reg::X2, 0b1100111),
        i_type(0, Reg::X1, 0b111, Reg::X2, 0b1100111),
        b_type(4, Reg::X1, Reg::X2, 0b010),
        b_type(4, Reg::X1, Reg::X2, 0b011),
        i_type(0, Reg::X1, 0b011, Reg::X2, 0b0000011),
        i_type(0, Reg::X1, 0b110, Reg::X2, 0b0000011),
        s_type(0, Reg::X1, Reg::X2, 0b011),
        s_type(0, Reg::X1, Reg::X2, 0b111),
    ] {
        assert_eq!(decode_word(word), Err(sw_isa_core::DecodeError::Invalid));
    }
}

#[test]
fn rejects_malformed_shift_and_register_op_funct7_values() {
    for word in [
        shift_imm(0b0000001, 0, Reg::X1, 0b001, Reg::X2),
        shift_imm(0b0100000, 0, Reg::X1, 0b001, Reg::X2),
        shift_imm(0b0000001, 0, Reg::X1, 0b101, Reg::X2),
        shift_imm(0b1111111, 31, Reg::X1, 0b101, Reg::X2),
        r_type(0b0000001, Reg::X1, Reg::X2, 0b000, Reg::X3),
        r_type(0b0100000, Reg::X1, Reg::X2, 0b001, Reg::X3),
        r_type(0b1111111, Reg::X1, Reg::X2, 0b101, Reg::X3),
    ] {
        assert_eq!(decode_word(word), Err(sw_isa_core::DecodeError::Invalid));
    }
}

#[test]
fn rejects_reserved_fence_and_system_forms() {
    for word in [
        (1 << 12) | 0b0001111,
        (Reg::X1.index_u8() as u32) << 7 | 0b0001111,
        (Reg::X1.index_u8() as u32) << 15 | 0b0001111,
        (1 << 28) | 0b0001111,
        0x0000_1073,
        0x0020_0073,
        0x3050_2073,
        0xffff_f073,
    ] {
        assert_eq!(decode_word(word), Err(sw_isa_core::DecodeError::Invalid));
    }
}

#[test]
fn branch_and_jump_decode_clear_unencoded_low_offset_bits() {
    assert_eq!(
        decode_word(b_type(6, Reg::X1, Reg::X2, 0b000)),
        Ok(Instruction::Branch {
            cond: BranchCond::Eq,
            rs1: Reg::X1,
            rs2: Reg::X2,
            offset: 6,
        })
    );
    assert_eq!(
        decode_word(j_type(6, 1, 0b1101111)),
        Ok(Instruction::Jal {
            rd: Reg::X1,
            offset: 6,
        })
    );
}

fn i_type(imm: i32, rs1: Reg, funct3: u32, rd: Reg, opcode: u32) -> u32 {
    ((imm as u32 & 0x0fff) << 20)
        | ((rs1.index_u8() as u32) << 15)
        | (funct3 << 12)
        | ((rd.index_u8() as u32) << 7)
        | opcode
}

fn shift_imm(funct7: u32, shamt: u32, rs1: Reg, funct3: u32, rd: Reg) -> u32 {
    (funct7 << 25)
        | (shamt << 20)
        | ((rs1.index_u8() as u32) << 15)
        | (funct3 << 12)
        | ((rd.index_u8() as u32) << 7)
        | 0b0010011
}

fn s_type(imm: i32, rs1: Reg, rs2: Reg, funct3: u32) -> u32 {
    let imm = imm as u32 & 0x0fff;
    ((imm >> 5) << 25)
        | ((rs2.index_u8() as u32) << 20)
        | ((rs1.index_u8() as u32) << 15)
        | (funct3 << 12)
        | ((imm & 0x1f) << 7)
        | 0b0100011
}

fn b_type(imm: i32, rs1: Reg, rs2: Reg, funct3: u32) -> u32 {
    let imm = imm as u32 & 0x1fff;
    ((imm >> 12) << 31)
        | (((imm >> 5) & 0x3f) << 25)
        | ((rs2.index_u8() as u32) << 20)
        | ((rs1.index_u8() as u32) << 15)
        | (funct3 << 12)
        | (((imm >> 1) & 0x0f) << 8)
        | (((imm >> 11) & 0x01) << 7)
        | 0b1100011
}

fn j_type(imm: i32, rd: u32, opcode: u32) -> u32 {
    let imm = imm as u32 & 0x1f_ffff;
    ((imm >> 20) << 31)
        | (((imm >> 1) & 0x03ff) << 21)
        | (((imm >> 11) & 0x01) << 20)
        | (((imm >> 12) & 0x0ff) << 12)
        | (rd << 7)
        | opcode
}

fn r_type(funct7: u32, rs2: Reg, rs1: Reg, funct3: u32, rd: Reg) -> u32 {
    (funct7 << 25)
        | ((rs2.index_u8() as u32) << 20)
        | ((rs1.index_u8() as u32) << 15)
        | (funct3 << 12)
        | ((rd.index_u8() as u32) << 7)
        | 0b0110011
}
