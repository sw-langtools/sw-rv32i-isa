use sw_rv32i_isa::{
    BaseIsa, BranchCond, Extension, FenceSet, ImmOp, Instruction, IsaProfile, LoadWidth,
    ProfileError, Reg, RegOp, StoreWidth,
};

#[test]
fn profile_metadata_models_base_and_extension_sets() {
    assert_eq!(IsaProfile::RV32E.base(), BaseIsa::Rv32e);
    assert_eq!(IsaProfile::RV32I.base(), BaseIsa::Rv32i);
    assert!(!IsaProfile::RV32I.has_extension(Extension::M));
    assert!(IsaProfile::RV32IM.has_extension(Extension::M));

    let custom = IsaProfile::new(BaseIsa::Rv32e, &[Extension::M]);
    assert_eq!(custom.base(), BaseIsa::Rv32e);
    assert!(custom.has_extension(Extension::M));
}

#[test]
fn rv32e_rejects_registers_x16_through_x31() {
    assert!(IsaProfile::RV32E.supports_register(Reg::X15));
    assert!(!IsaProfile::RV32E.supports_register(Reg::X16));
    assert_eq!(
        IsaProfile::RV32E.validate_register(Reg::X31),
        Err(ProfileError::IllegalRegister {
            reg: Reg::X31,
            profile: IsaProfile::RV32E,
        })
    );
}

#[test]
fn rv32i_accepts_all_integer_registers() {
    assert!(IsaProfile::RV32I.supports_register(Reg::X0));
    assert!(IsaProfile::RV32I.supports_register(Reg::X31));
    assert_eq!(IsaProfile::RV32I.validate_register(Reg::X31), Ok(()));
}

#[test]
fn rv32i_accepts_current_base_instruction_model() {
    for insn in rv32i_base_examples() {
        assert_eq!(IsaProfile::RV32I.validate_instruction(insn), Ok(()));
    }
}

#[test]
fn rv32e_rejects_high_registers_in_any_operand_position() {
    for insn in [
        Instruction::Lui {
            rd: Reg::X16,
            imm: 0,
        },
        Instruction::Jalr {
            rd: Reg::X1,
            rs1: Reg::X17,
            offset: 0,
        },
        Instruction::Branch {
            cond: BranchCond::Eq,
            rs1: Reg::X1,
            rs2: Reg::X18,
            offset: 0,
        },
        Instruction::Store {
            width: StoreWidth::Word,
            rs1: Reg::X2,
            rs2: Reg::X31,
            offset: 0,
        },
        Instruction::Op {
            op: RegOp::Add,
            rd: Reg::X1,
            rs1: Reg::X2,
            rs2: Reg::X28,
        },
    ] {
        assert!(matches!(
            IsaProfile::RV32E.validate_instruction(insn),
            Err(ProfileError::IllegalRegister { .. })
        ));
    }
}

#[test]
fn rv32e_accepts_base_instructions_using_low_registers() {
    for insn in rv32e_base_examples() {
        assert_eq!(IsaProfile::RV32E.validate_instruction(insn), Ok(()));
    }
}

fn rv32i_base_examples() -> [Instruction; 13] {
    [
        Instruction::Lui {
            rd: Reg::X31,
            imm: 0,
        },
        Instruction::Auipc {
            rd: Reg::X30,
            imm: 0,
        },
        Instruction::Jal {
            rd: Reg::X29,
            offset: 0,
        },
        Instruction::Jalr {
            rd: Reg::X28,
            rs1: Reg::X27,
            offset: 0,
        },
        Instruction::Branch {
            cond: BranchCond::Geu,
            rs1: Reg::X26,
            rs2: Reg::X25,
            offset: 0,
        },
        Instruction::Load {
            width: LoadWidth::Word,
            rd: Reg::X24,
            rs1: Reg::X23,
            offset: 0,
        },
        Instruction::Store {
            width: StoreWidth::Word,
            rs1: Reg::X22,
            rs2: Reg::X21,
            offset: 0,
        },
        Instruction::OpImm {
            op: ImmOp::Addi,
            rd: Reg::X20,
            rs1: Reg::X19,
            imm: 0,
        },
        Instruction::Op {
            op: RegOp::Add,
            rd: Reg::X18,
            rs1: Reg::X17,
            rs2: Reg::X16,
        },
        Instruction::Fence {
            predecessor: FenceSet::R,
            successor: FenceSet::W,
        },
        Instruction::Ecall,
        Instruction::Ebreak,
        Instruction::OpImm {
            op: ImmOp::Srai,
            rd: Reg::X15,
            rs1: Reg::X14,
            imm: 31,
        },
    ]
}

fn rv32e_base_examples() -> [Instruction; 13] {
    [
        Instruction::Lui {
            rd: Reg::X15,
            imm: 0,
        },
        Instruction::Auipc {
            rd: Reg::X14,
            imm: 0,
        },
        Instruction::Jal {
            rd: Reg::X13,
            offset: 0,
        },
        Instruction::Jalr {
            rd: Reg::X12,
            rs1: Reg::X11,
            offset: 0,
        },
        Instruction::Branch {
            cond: BranchCond::Geu,
            rs1: Reg::X10,
            rs2: Reg::X9,
            offset: 0,
        },
        Instruction::Load {
            width: LoadWidth::Word,
            rd: Reg::X8,
            rs1: Reg::X7,
            offset: 0,
        },
        Instruction::Store {
            width: StoreWidth::Word,
            rs1: Reg::X6,
            rs2: Reg::X5,
            offset: 0,
        },
        Instruction::OpImm {
            op: ImmOp::Addi,
            rd: Reg::X4,
            rs1: Reg::X3,
            imm: 0,
        },
        Instruction::Op {
            op: RegOp::Add,
            rd: Reg::X2,
            rs1: Reg::X1,
            rs2: Reg::X0,
        },
        Instruction::Fence {
            predecessor: FenceSet::R,
            successor: FenceSet::W,
        },
        Instruction::Ecall,
        Instruction::Ebreak,
        Instruction::OpImm {
            op: ImmOp::Srai,
            rd: Reg::X15,
            rs1: Reg::X14,
            imm: 31,
        },
    ]
}
