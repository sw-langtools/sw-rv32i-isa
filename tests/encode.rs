use sw_isa_core::Architecture;
use sw_rv32i_isa::{
    Addr, BranchCond, FenceSet, ImmOp, Instruction, LoadWidth, Reg, RegOp, Rv32i, StoreWidth,
    decode_word, encode_word,
};

#[test]
fn architecture_encode_writes_little_endian_words() {
    let mut out = [0xaa; 8];
    let len = Rv32i::encode(
        &Instruction::Lui {
            rd: Reg::X1,
            imm: 0x1234_5000,
        },
        &mut out,
    );
    assert_eq!(len, Ok(4));
    assert_eq!(&out[..4], &0x1234_50b7u32.to_le_bytes());
    assert_eq!(&out[4..], &[0xaa; 4]);

    let mut short = [0; 3];
    assert_eq!(
        Rv32i::encode(&Instruction::Ebreak, &mut short),
        Err(sw_isa_core::EncodeError::BufferTooSmall)
    );
}

#[test]
fn encodes_u_and_j_formats() {
    assert_round_trip(Instruction::Auipc {
        rd: Reg::X5,
        imm: -4096,
    });
    assert_round_trip(Instruction::Jal {
        rd: Reg::X1,
        offset: -4,
    });
}

#[test]
fn encodes_branches_with_signed_offsets() {
    assert_round_trip(Instruction::Branch {
        cond: BranchCond::Ge,
        rs1: Reg::X1,
        rs2: Reg::X2,
        offset: -4,
    });
    assert_round_trip(Instruction::Branch {
        cond: BranchCond::Ltu,
        rs1: Reg::X3,
        rs2: Reg::X4,
        offset: 16,
    });
}

#[test]
fn encodes_jalr_loads_and_stores() {
    assert_round_trip(Instruction::Jalr {
        rd: Reg::X1,
        rs1: Reg::X2,
        offset: -8,
    });
    assert_round_trip(Instruction::Load {
        width: LoadWidth::HalfUnsigned,
        rd: Reg::X3,
        rs1: Reg::X2,
        offset: 12,
    });
    assert_round_trip(Instruction::Store {
        width: StoreWidth::Word,
        rs1: Reg::X2,
        rs2: Reg::X8,
        offset: -16,
    });
}

#[test]
fn encodes_immediate_and_register_arithmetic() {
    assert_round_trip(Instruction::OpImm {
        op: ImmOp::Ori,
        rd: Reg::X5,
        rs1: Reg::X6,
        imm: -1,
    });
    assert_round_trip(Instruction::OpImm {
        op: ImmOp::Srai,
        rd: Reg::X9,
        rs1: Reg::X8,
        imm: 7,
    });
    assert_round_trip(Instruction::Op {
        op: RegOp::Sub,
        rd: Reg::X10,
        rs1: Reg::X11,
        rs2: Reg::X12,
    });
}

#[test]
fn encodes_fence_and_system() {
    assert_round_trip(Instruction::Fence {
        predecessor: FenceSet::R,
        successor: FenceSet::W,
    });
    assert_eq!(encode_word(Instruction::Ecall), Ok(0x0000_0073));
    assert_eq!(encode_word(Instruction::Ebreak), Ok(0x0010_0073));
}

#[test]
fn rejects_invalid_immediates() {
    for insn in [
        addi_like(2048),
        addi_like(-2049),
        Instruction::Branch {
            cond: BranchCond::Eq,
            rs1: Reg::X1,
            rs2: Reg::X2,
            offset: 3,
        },
        Instruction::Branch {
            cond: BranchCond::Eq,
            rs1: Reg::X1,
            rs2: Reg::X2,
            offset: 4096,
        },
        Instruction::Jal {
            rd: Reg::X1,
            offset: 1,
        },
        Instruction::Jal {
            rd: Reg::X1,
            offset: 1_048_576,
        },
        Instruction::Lui {
            rd: Reg::X1,
            imm: 1,
        },
        Instruction::OpImm {
            op: ImmOp::Slli,
            rd: Reg::X1,
            rs1: Reg::X2,
            imm: 32,
        },
    ] {
        assert_eq!(
            encode_word(insn),
            Err(sw_isa_core::EncodeError::InvalidOperands)
        );
    }
}

fn assert_round_trip(insn: Instruction) {
    let word = encode_word(insn).expect("encodes");
    assert_eq!(decode_word(word), Ok(insn));
    assert_eq!(Rv32i::decode(&word.to_le_bytes(), Addr(0)), Ok((insn, 4)));
}

fn addi_like(imm: i32) -> Instruction {
    Instruction::OpImm {
        op: ImmOp::Addi,
        rd: Reg::X1,
        rs1: Reg::X2,
        imm,
    }
}
