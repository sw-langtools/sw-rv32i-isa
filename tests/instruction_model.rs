use sw_isa_core::{Architecture, Mnemonic};
use sw_rv32i_isa::{
    BranchCond, FenceSet, Format, ImmOp, Instruction, LoadWidth, Opcode, Reg, RegOp, StoreWidth,
    b_imm, fits_signed, i_imm, j_imm, s_imm, sign_extend, u_imm,
};

#[test]
fn instruction_opcode_reflects_semantic_variant() {
    assert_eq!(
        Instruction::Branch {
            cond: BranchCond::Geu,
            rs1: Reg::X1,
            rs2: Reg::X2,
            offset: -4,
        }
        .opcode(),
        Opcode::Bgeu
    );
    assert_eq!(
        Instruction::Load {
            width: LoadWidth::HalfUnsigned,
            rd: Reg::X3,
            rs1: Reg::X4,
            offset: 8,
        }
        .opcode(),
        Opcode::Lhu
    );
    assert_eq!(
        Instruction::Op {
            op: RegOp::Sub,
            rd: Reg::X5,
            rs1: Reg::X6,
            rs2: Reg::X7,
        }
        .opcode(),
        Opcode::Sub
    );
    assert_eq!(Instruction::Ebreak.opcode(), Opcode::Ebreak);
}

#[test]
fn opcode_mnemonics_are_canonical_lowercase() {
    assert_eq!(Opcode::Lui.mnemonic(), "lui");
    assert_eq!(Opcode::Auipc.mnemonic(), "auipc");
    assert_eq!(Opcode::Bltu.mnemonic(), "bltu");
    assert_eq!(Opcode::Lbu.mnemonic(), "lbu");
    assert_eq!(Opcode::Srai.mnemonic(), "srai");
    assert_eq!(Opcode::Ebreak.mnemonic(), "ebreak");
}

#[test]
fn width_helpers_report_byte_counts_and_sign_behavior() {
    assert_eq!(LoadWidth::Byte.bytes(), 1);
    assert_eq!(LoadWidth::Half.bytes(), 2);
    assert_eq!(LoadWidth::Word.bytes(), 4);
    assert_eq!(LoadWidth::ByteUnsigned.bytes(), 1);
    assert!(LoadWidth::Byte.is_signed());
    assert!(LoadWidth::Word.is_signed());
    assert!(!LoadWidth::HalfUnsigned.is_signed());

    assert_eq!(StoreWidth::Byte.bytes(), 1);
    assert_eq!(StoreWidth::Half.bytes(), 2);
    assert_eq!(StoreWidth::Word.bytes(), 4);
}

#[test]
fn fence_set_helpers_preserve_iorw_bits() {
    let io = FenceSet::from_bits(FenceSet::I.bits() | FenceSet::O.bits()).unwrap();
    assert_eq!(FenceSet::from_bits(0b1_0000), None);
    assert!(FenceSet::IORW.contains(FenceSet::R));
    assert!(FenceSet::IORW.contains(FenceSet::W));
    assert!(io.contains(FenceSet::I));
    assert!(!io.contains(FenceSet::R));
}

#[test]
fn format_is_fixed_32_bits_for_all_base_forms() {
    for format in [
        Format::R,
        Format::I,
        Format::S,
        Format::B,
        Format::U,
        Format::J,
        Format::System,
    ] {
        assert_eq!(sw_isa_core::format::FormatInfo::size_bytes(&format), 4);
    }
    assert_eq!(sw_rv32i_isa::Rv32i::MIN_INSTR_BYTES, 4);
    assert_eq!(sw_rv32i_isa::Rv32i::MAX_INSTR_BYTES, 4);
}

#[test]
fn sign_extension_and_fit_helpers_cover_boundaries() {
    assert_eq!(sign_extend(0x7ff, 12), 2047);
    assert_eq!(sign_extend(0x800, 12), -2048);
    assert_eq!(sign_extend(0xfff, 12), -1);

    assert!(fits_signed(2047, 12));
    assert!(fits_signed(-2048, 12));
    assert!(!fits_signed(2048, 12));
    assert!(!fits_signed(-2049, 12));
}

#[test]
fn immediate_extractors_follow_rv32i_bit_placement() {
    assert_eq!(i_imm(0xfff0_0093), -1);
    assert_eq!(s_imm(0xfe20_2e23), -4);
    assert_eq!(b_imm(0xfe20_8ee3), -4);
    assert_eq!(u_imm(0x1234_5037), 0x1234_5000);
    assert_eq!(j_imm(0xffdf_f0ef), -4);
}

#[test]
fn operation_enums_map_to_expected_opcodes() {
    assert_eq!(BranchCond::Lt.opcode(), Opcode::Blt);
    assert_eq!(LoadWidth::ByteUnsigned.opcode(), Opcode::Lbu);
    assert_eq!(StoreWidth::Word.opcode(), Opcode::Sw);
    assert_eq!(ImmOp::Srai.opcode(), Opcode::Srai);
    assert_eq!(RegOp::And.opcode(), Opcode::And);
}
