use core::str::FromStr;

use sw_isa_core::register::RegisterId;
use sw_rv32i_isa::{Reg, parse_register};

#[test]
fn canonical_registers_cover_x0_through_x31() {
    for index in 0..32 {
        let reg = Reg::new(index).expect("valid RV32I register");
        assert_eq!(reg.index_u8(), index);
        assert_eq!(reg.index(), index as u32);
        assert_eq!(reg.name(), format_register_name(index));
        assert_eq!(reg.to_string(), format_register_name(index));
        assert_eq!(parse_register(format_register_name(index)), Some(reg));
    }
}

#[test]
fn rejects_out_of_range_and_malformed_canonical_registers() {
    assert_eq!(Reg::new(32), None);

    for text in ["", "x", "x-1", "x01", "x32", "x255", "r0", "X32"] {
        assert_eq!(parse_register(text), None, "{text}");
    }
}

#[test]
fn parses_uppercase_canonical_prefix() {
    assert_eq!(parse_register("X0"), Some(Reg::X0));
    assert_eq!(parse_register("X31"), Some(Reg::X31));
}

#[test]
fn abi_aliases_map_to_standard_rv32i_registers() {
    let aliases = [
        ("zero", Reg::X0),
        ("ra", Reg::X1),
        ("sp", Reg::X2),
        ("gp", Reg::X3),
        ("tp", Reg::X4),
        ("t0", Reg::X5),
        ("t1", Reg::X6),
        ("t2", Reg::X7),
        ("s0", Reg::X8),
        ("s1", Reg::X9),
        ("a0", Reg::X10),
        ("a1", Reg::X11),
        ("a2", Reg::X12),
        ("a3", Reg::X13),
        ("a4", Reg::X14),
        ("a5", Reg::X15),
        ("a6", Reg::X16),
        ("a7", Reg::X17),
        ("s2", Reg::X18),
        ("s3", Reg::X19),
        ("s4", Reg::X20),
        ("s5", Reg::X21),
        ("s6", Reg::X22),
        ("s7", Reg::X23),
        ("s8", Reg::X24),
        ("s9", Reg::X25),
        ("s10", Reg::X26),
        ("s11", Reg::X27),
        ("t3", Reg::X28),
        ("t4", Reg::X29),
        ("t5", Reg::X30),
        ("t6", Reg::X31),
    ];

    for (alias, reg) in aliases {
        assert_eq!(parse_register(alias), Some(reg), "{alias}");
        assert_eq!(reg.abi_name(), alias);
    }
}

#[test]
fn abi_aliases_are_lowercase_only() {
    for text in ["ZERO", "Ra", "SP", "A0", "T6"] {
        assert_eq!(parse_register(text), None, "{text}");
    }
}

#[test]
fn from_str_uses_the_shared_parser() {
    assert_eq!(Reg::from_str("x10"), Ok(Reg::X10));
    assert_eq!(Reg::from_str("a0"), Ok(Reg::X10));
    assert!(Reg::from_str("pc").is_err());
}

fn format_register_name(index: u8) -> &'static str {
    const NAMES: [&str; 32] = [
        "x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7", "x8", "x9", "x10", "x11", "x12", "x13",
        "x14", "x15", "x16", "x17", "x18", "x19", "x20", "x21", "x22", "x23", "x24", "x25", "x26",
        "x27", "x28", "x29", "x30", "x31",
    ];
    NAMES[index as usize]
}
