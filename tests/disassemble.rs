use sw_isa_core::Architecture;
use sw_rv32i_isa::{
    Addr, BranchCond, FenceSet, ImmOp, Instruction, LoadWidth, Reg, RegOp, Rv32i, StoreWidth,
    disassemble, encode_word,
};

#[test]
fn disassembles_upper_and_jump_forms() {
    assert_disasm(
        Instruction::Lui {
            rd: Reg::X1,
            imm: 0x1234_5000,
        },
        "lui x1, 305418240",
    );
    assert_disasm(
        Instruction::Auipc {
            rd: Reg::X5,
            imm: -4096,
        },
        "auipc x5, -4096",
    );
    assert_disasm(
        Instruction::Jal {
            rd: Reg::X1,
            offset: -4,
        },
        "jal x1, -4",
    );
    assert_disasm(
        Instruction::Jalr {
            rd: Reg::X1,
            rs1: Reg::X2,
            offset: -8,
        },
        "jalr x1, -8(x2)",
    );
}

#[test]
fn disassembles_branch_load_store_addressing() {
    assert_disasm(
        Instruction::Branch {
            cond: BranchCond::Geu,
            rs1: Reg::X1,
            rs2: Reg::X2,
            offset: 16,
        },
        "bgeu x1, x2, 16",
    );
    assert_disasm(
        Instruction::Load {
            width: LoadWidth::HalfUnsigned,
            rd: Reg::X3,
            rs1: Reg::X2,
            offset: 12,
        },
        "lhu x3, 12(x2)",
    );
    assert_disasm(
        Instruction::Store {
            width: StoreWidth::Word,
            rs1: Reg::X2,
            rs2: Reg::X8,
            offset: -16,
        },
        "sw x8, -16(x2)",
    );
}

#[test]
fn disassembles_arithmetic_and_system_forms() {
    assert_disasm(
        Instruction::OpImm {
            op: ImmOp::Ori,
            rd: Reg::X5,
            rs1: Reg::X6,
            imm: -1,
        },
        "ori x5, x6, -1",
    );
    assert_disasm(
        Instruction::Op {
            op: RegOp::Sub,
            rd: Reg::X10,
            rs1: Reg::X11,
            rs2: Reg::X12,
        },
        "sub x10, x11, x12",
    );
    assert_disasm(
        Instruction::Fence {
            predecessor: FenceSet::R,
            successor: FenceSet::W,
        },
        "fence r, w",
    );
    assert_disasm(
        Instruction::Fence {
            predecessor: FenceSet::IORW,
            successor: FenceSet::NONE,
        },
        "fence iorw, 0",
    );
    assert_disasm(Instruction::Ecall, "ecall");
    assert_disasm(Instruction::Ebreak, "ebreak");
}

#[test]
fn architecture_disassemble_matches_public_helper() {
    let insn = Instruction::OpImm {
        op: ImmOp::Addi,
        rd: Reg::X10,
        rs1: Reg::X0,
        imm: 42,
    };
    let mut via_trait = String::new();
    let mut via_helper = String::new();
    Rv32i::disassemble(&insn, &mut via_trait).unwrap();
    disassemble(insn, &mut via_helper).unwrap();
    assert_eq!(via_trait, via_helper);
    assert_eq!(via_trait, "addi x10, x0, 42");
}

#[test]
fn decoded_words_disassemble_stably_for_demos() {
    let insn = Instruction::Store {
        width: StoreWidth::Word,
        rs1: Reg::X0,
        rs2: Reg::X10,
        offset: 256,
    };
    let word = encode_word(insn).unwrap();
    let (decoded, len) = Rv32i::decode(&word.to_le_bytes(), Addr(0)).unwrap();
    assert_eq!(len, 4);
    assert_disasm(decoded, "sw x10, 256(x0)");
}

fn assert_disasm(insn: Instruction, expected: &str) {
    let mut out = String::new();
    disassemble(insn, &mut out).unwrap();
    assert_eq!(out, expected);
}
