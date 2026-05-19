//! RV32I disassembly.

use core::fmt;
use sw_isa_core::Mnemonic;

use crate::{FenceSet, Instruction};

pub fn disassemble(insn: Instruction, w: &mut dyn fmt::Write) -> fmt::Result {
    match insn {
        Instruction::Lui { rd, imm } => write!(w, "lui {rd}, {imm}"),
        Instruction::Auipc { rd, imm } => write!(w, "auipc {rd}, {imm}"),
        Instruction::Jal { rd, offset } => write!(w, "jal {rd}, {offset}"),
        Instruction::Jalr { rd, rs1, offset } => write!(w, "jalr {rd}, {offset}({rs1})"),
        Instruction::Branch {
            cond,
            rs1,
            rs2,
            offset,
        } => write!(w, "{} {rs1}, {rs2}, {offset}", cond.opcode().mnemonic()),
        Instruction::Load {
            width,
            rd,
            rs1,
            offset,
        } => write!(w, "{} {rd}, {offset}({rs1})", width.opcode().mnemonic()),
        Instruction::Store {
            width,
            rs1,
            rs2,
            offset,
        } => write!(w, "{} {rs2}, {offset}({rs1})", width.opcode().mnemonic()),
        Instruction::OpImm { op, rd, rs1, imm } => {
            write!(w, "{} {rd}, {rs1}, {imm}", op.opcode().mnemonic())
        }
        Instruction::Op { op, rd, rs1, rs2 } => {
            write!(w, "{} {rd}, {rs1}, {rs2}", op.opcode().mnemonic())
        }
        Instruction::Fence {
            predecessor,
            successor,
        } => write!(
            w,
            "fence {}, {}",
            FenceDisplay(predecessor),
            FenceDisplay(successor)
        ),
        Instruction::Ecall => w.write_str("ecall"),
        Instruction::Ebreak => w.write_str("ebreak"),
    }
}

struct FenceDisplay(FenceSet);

impl fmt::Display for FenceDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let set = self.0;
        if set == FenceSet::NONE {
            return f.write_str("0");
        }
        if set.contains(FenceSet::I) {
            f.write_str("i")?;
        }
        if set.contains(FenceSet::O) {
            f.write_str("o")?;
        }
        if set.contains(FenceSet::R) {
            f.write_str("r")?;
        }
        if set.contains(FenceSet::W) {
            f.write_str("w")?;
        }
        Ok(())
    }
}
