use sw_isa_core::{Architecture, Mnemonic};
use sw_rv32i_isa::{Addr, Format, Instruction, Opcode, Reg, Rv32i};

#[test]
fn architecture_metadata_matches_rv32i() {
    assert_eq!(Rv32i::NAME, "RV32I");
    assert_eq!(Rv32i::WORD_BITS, 32);
    assert_eq!(Rv32i::MIN_INSTR_BYTES, 4);
    assert_eq!(Rv32i::MAX_INSTR_BYTES, 4);
    assert_eq!(Rv32i::ADDRESS_UNIT, sw_isa_core::address::AddressUnit::Byte);
    assert_eq!(Rv32i::ENDIAN, sw_isa_core::endian::Endian::Little);
}

#[test]
fn placeholder_types_are_wired_to_core_traits() {
    assert_eq!(Opcode::Invalid.mnemonic(), "invalid");
    assert_eq!(sw_isa_core::register::RegisterId::index(Reg::X0), 0);
    assert_eq!(sw_isa_core::register::RegisterId::name(Reg::X0), "x0");
    assert_eq!(
        sw_isa_core::format::FormatInfo::size_bytes(&Format::Fixed32),
        4
    );
}

#[test]
fn address_steps_by_bytes() {
    assert_eq!(sw_isa_core::address::AddressType::to_u64(Addr(4)), 4);
    assert_eq!(sw_isa_core::address::AddressType::step(Addr(4), 4), Addr(8));
    assert_eq!(
        sw_isa_core::address::AddressType::step(Addr(4), -4),
        Addr(0)
    );
}

#[test]
fn decode_and_encode_are_explicit_stubs() {
    assert_eq!(
        Rv32i::decode(&[0; 3], Addr(0)),
        Err(sw_isa_core::DecodeError::Truncated)
    );
    assert_eq!(
        Rv32i::decode(&[0; 4], Addr(0)),
        Err(sw_isa_core::DecodeError::Invalid)
    );

    let mut out = [0; 4];
    assert_eq!(
        Rv32i::encode(&Instruction::Invalid, &mut out),
        Err(sw_isa_core::EncodeError::InvalidOperands)
    );
}
