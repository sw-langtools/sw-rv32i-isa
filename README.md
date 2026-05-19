# sw-rv32i-isa

RV32I ISA description: opcodes, encoding, decoding, disassembly.

## Status

`0.1.0` is a scaffold. Follow-up saga steps will add the register model,
instruction model, decode, encode, disassembly, and illegal encoding coverage.

Assembler, emulator, target, and codegen crates should consume this shared ISA
surface rather than duplicating RISC-V bit layout rules.

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
