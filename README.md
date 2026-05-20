# sw-rv32i-isa

RV32I ISA description: opcodes, encoding, decoding, disassembly.

## Status

`0.1.0` contains the initial RV32I register model, instruction model, decode,
encode, disassembly, legality coverage, and ISA profile validation.

Assembler, emulator, target, and codegen crates should consume this shared ISA
surface rather than duplicating RISC-V bit layout rules.

## ISA profiles

The crate exposes `IsaProfile` so downstream tools can validate programs
against a target profile before assembling, emulating, or generating code. The
first supported profiles are `rv32e`, `rv32i`, and `rv32im` shape placeholders:

- `rv32e` accepts only integer registers `x0..x15`.
- `rv32i` accepts the current base integer instruction model and `x0..x31`.
- `rv32im` records the `M` extension bit so multiply/divide instructions can be
  added without changing the profile API shape.

The repository and package names should remain `sw-rv32i-*` until `rv32e`,
`rv32i`, and `rv32m` behavior are represented across the ISA, assembler, and
emulator. Rename planning should happen as a dedicated migration saga after
that point so remotes, crate names, docs, and cross-repo dependencies move
together.

## Sibling layout

Cross-crate deps assume sibling clones at `~/github/sw-langtools/`:

```
sw-langtools/
  sw-isa-core/
  sw-rv32i-isa/
  sw-rv32i-asm/
  sw-rv32i-emulator/
  sw-rv32i-target/
  sw-rv32i-codegen/
```

## License

MIT.
