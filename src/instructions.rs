// Reference(s):
//  https://www.pagetable.com/c64ref/6502/?tab=2

// (ﾉಥДಥ)ﾉ ︵┻━┻･/

type Byte = u8;

/* `~` = +1 if page is crossed */

////////// Load Accumulator with Memory (LDA) //////////
//
// Status Flags Affected: {N, Z}

// Addressing Mode: Immediate
// Opcode:          $A9
// Bytes:           2
// Cycles:          2
pub const LDA_IMM: Byte = 0xA9;

// Addressing Mode: Absolute
// Opcode:          $AD
// Bytes:           3
// Cycles:          4
pub const LDA_ABS: Byte = 0xAD;

// Addressing Mode: X-Indexed Absolute
// Opcode:          $BD
// Bytes:           3
// Cycles:          ~4
pub const LDA_ABSX: Byte = 0xBD;

// Addressing Mode: Y-Indexed Absolute
// Opcode:          $B9
// Bytes:           3
// Cycles:          ~4
pub const LDA_ABSY: Byte = 0xB9;

// Addressing Mode: Zero Page
// Opcode:          $A5
// Bytes:           2
// Cycles:          3
pub const LDA_ZP: Byte = 0xA5;

// Addressing Mode: X-Indexed Zero Page
// Opcode:          $B5
// Bytes:           2
// Cycles:          4
pub const LDA_ZPX: Byte = 0xB5;

// Addressing Mode: X-Indexed Zero Page Indirect
// Opcode:          $A1
// Bytes:           2
// Cycles:          6
pub const LDA_ZPX_IND: Byte = 0xA1;

// Addressing Mode: Zero Page Indirect Y-Indexed
// Opcode:          $B1
// Bytes:           2
// Cycles:          ~5
pub const LDA_ZPY_IND: Byte = 0xB1;

////////// Load Index Register X from Memory (LDX) //////////
//
// Status Flags Affected: {N, Z}

// Addressing Mode: Immediate
// Opcode:          $A2
// Bytes:           2
// Cycles:          2
pub const LDX_IMM: Byte = 0xA2;

// Addressing Mode: Absolute
// Opcode:          $AE
// Bytes:           3
// Cycles:          4
pub const LDX_ABS: Byte = 0xAE;

// Addressing Mode: Y-Indexed Absolute
// Opcode:          $BE
// Bytes:           3
// Cycles:          ~4
pub const LDX_ABSY: Byte = 0xBE;

// Addressing Mode: Zero Page
// Opcode:          $A6
// Bytes:           2
// Cycles:          3
pub const LDX_ZP: Byte = 0xA6;

// Addressing Mode: Y-Indexed Zero Page
// Opcode:          $B6
// Bytes:           2
// Cycles:          4
pub const LDX_ZPY: Byte = 0xB6;

////////// Load Index Register Y from Memory (LDY) //////////
//
// Status Flags Affected: {N, Z}

// Addressing Mode: Immediate
// Opcode:          $A0
// Bytes:           2
// Cycles:          2
pub const LDY_IMM: Byte = 0xA0;

// Addressing Mode: Absolute
// Opcode:          $AC
// Bytes:           3
// Cycles:          4
pub const LDY_ABS: Byte = 0xAC;

// Addressing Mode: X-Indexed Absolute
// Opcode:          $BC
// Bytes:           3
// Cycles:          ~4
pub const LDY_ABSX: Byte = 0xBC;

// Addressing Mode: Zero Page
// Opcode:          $A4
// Bytes:           2
// Cycles:          3
pub const LDY_ZP: Byte = 0xA4;

// Addressing Mode: X-Indexed Zero Page
// Opcode:          $B4
// Bytes:           2
// Cycles:          4
pub const LDY_ZPX: Byte = 0xB4;

////////// Store Accumulator in Memory (STA) //////////
//
// Status Flags Affected: ∅

// Addressing Mode: Absolute
// Opcode:          $8D
// Bytes:           3
// Cycles:          4
pub const STA_ABS: Byte = 0x8D;

// Addressing Mode: X-Indexed Absolute
// Opcode:          $9D
// Bytes:           3
// Cycles:          5
pub const STA_ABSX: Byte = 0x9D;

// Addressing Mode: Y-Indexed Absolute
// Opcode:          $99
// Bytes:           3
// Cycles:          5
pub const STA_ABSY: Byte = 0x99;

// Addressing Mode: Zero Page
// Opcode:          $85
// Bytes:           2
// Cycles:          3
pub const STA_ZP: Byte = 0x85;

// Addressing Mode: X-Indexed Zero Page
// Opcode:          $95
// Bytes:           2
// Cycles:          4
pub const STA_ZPX: Byte = 0x95;

// Addressing Mode: X-Indexed Zero Page Indirect
// Opcode:          $81
// Bytes:           2
// Cycles:          6
pub const STA_ZPX_IND: Byte = 0x81;

// Addressing Mode: Zero Page Indirect Y-Indexed
// Opcode:          $91
// Bytes:           2
// Cycles:          6
pub const STA_ZPY_IND: Byte = 0x91;

////////// Store Index Register X in Memory (STX) //////////
//
// Status Flags Affected: ∅

// Addressing Mode: Absolute
// Opcode:          $91
// Bytes:           3
// Cycles:          4
pub const STX_ABS: Byte = 0x91;

// Addressing Mode: Zero Page
// Opcode:          $86
// Bytes:           2
// Cycles:          3
pub const STX_ZP: Byte = 0x86;

// Addressing Mode: Y-Indexed Zero Page
// Opcode:          $96
// Bytes:           2
// Cycles:          4
pub const STX_ZPY: Byte = 0x96;

////////// Store Index Register Y in Memory (STY) //////////
//
// Status Flags Affected: ∅

// Addressing Mode: Absolute
// Opcode:          $8C
// Bytes:           3
// Cycles:          4
pub const STY_ABS: Byte = 0x8C;

// Addressing Mode: Zero Page
// Opcode:          $84
// Bytes:           2
// Cycles:          3
pub const STY_ZP: Byte = 0x84;

// Addressing Mode: X-Indexed Zero Page
// Opcode:          $94
// Bytes:           2
// Cycles:          4
pub const STY_ZPX: Byte = 0x94;

////////// Transfer Accumulator to Index X (TAX) //////////
//
// Status Flags Affected: {N, Z}

// Addressing Mode: Implied
// Opcode:          $AA
// Bytes:           1
// Cycles:          2
pub const TAX_IMP: Byte = 0xAA;

////////// Transfer Accumulator to Index Y //////////
//
// Status Flags Affected: {N, Z}

// Addressing Mode: Implied
// Opcode:          $A8
// Bytes:           1
// Cycles:          2
pub const TAY_IMP: Byte = 0xA8;

////////// Transfer Stack Pointer to Index X (TSX) //////////
//
// Status Flags Affected: {N, Z}

// Addressing Mode: Implied
// Opcode:          $BA
// Bytes:           1
// Cycles:          2
pub const TSX_IMP: Byte = 0xBA;

////////// Transfer Index X to Accumulator (TXA) //////////
//
// Status Flags Affected: {N, Z}

// Addressing Mode: Implied
// Opcode:          $8A
// Bytes:           1
// Cycles:          2
pub const TXA_IMP: Byte = 0x8A;

////////// Transfer Index X to Stack Pointer (TXS) //////////
//
// Status Flags Affected: ∅

// Addressing Mode: Implied
// Opcode:          $9A
// Bytes:           1
// Cycles:          2
pub const TXS_IMP: Byte = 0x9A;

////////// Transfer Index Y to Accumulator (TYA) //////////
//
// Status Flags Affected: {N, Z}

// Addressing Mode: Implied
// Opcode:          $98
// Bytes:           1
// Cycles:          2
pub const TYA_IMP: Byte = 0x98;

////////// Push Accumulator on Stack (PHA) //////////
//
// Status Flags Affected: ∅

// Addressing Mode: Implied
// Opcode:          $48
// Bytes:           1
// Cycles:          3
pub const PHA_IMP: Byte = 0x48;

////////// Push Processor Status on Stack (PHP) //////////
//
// Status Flags Affected: ∅

// Addressing Mode: Implied
// Opcode:          $08
// Bytes:           1
// Cycles:          3
pub const PHP_IMP: Byte = 0x08;

////////// Pull Accumulator from Stack (PLA) //////////
//
// Status Flags Affected: {N, Z}

// Addressing Mode: Implied
// Opcode:          $68
// Bytes:           1
// Cycles:          4
pub const PLA_IMP: Byte = 0x68;

////////// Pull Processor Status from Stack (PLP) //////////
//
// Status Flags Affected: {N, V, D, I, Z, C}

// Addressing Mode: Implied
// Opcode:          $28
// Bytes:           1
// Cycles:          4
pub const PLP_IMP: Byte = 0x28;

////////// Arithmetic Shift Left (ASL) //////////
//
// Status Flags Affected: {N, Z, C}

// Addressing Mode: Accumulator
// Opcode:          $0A
// Bytes:           1
// Cycles:          2
pub const ASL_ACC: Byte = 0x0A;

// Addressing Mode: Absolute
// Opcode:          $0E
// Bytes:           3
// Cycles:          6
pub const ASL_ABS: Byte = 0x0E;

// Addressing Mode: X-Indexed Absolute
// Opcode:          $1E
// Bytes:           3
// Cycles:          7
pub const ASL_ABSX: Byte = 0x1E;

// Addressing Mode: Zero Page
// Opcode:          $06
// Bytes:           2
// Cycles:          5
pub const ASL_ZP: Byte = 0x06;

// Addressing Mode: X-Indexed Zero Page
// Opcode:          $16
// Bytes:           2
// Cycles:          6
pub const ASL_ZPX: Byte = 0x16;

////////// Logical Shift Right (LSR) //////////
//
// Status Flags Affected: {(N: 0), Z, C}

// Addressing Mode: Accumulator
// Opcode:          $4A
// Bytes:           1
// Cycles:          2
pub const LSR_ACC: Byte = 0x4A;

// Addressing Mode: Absolute
// Opcode:          $4E
// Bytes:           3
// Cycles:          6
pub const LSR_ABS: Byte = 0x4E;

// Addressing Mode: X-Indexed Absolute
// Opcode:          $5E
// Bytes:           3
// Cycles:          7
pub const LSR_ABSX: Byte = 0x5E;

// Addressing Mode: Zero Page
// Opcode:          $46
// Bytes:           2
// Cycles:          5
pub const LSR_ZP: Byte = 0x46;

// Addressing Mode: X-Indexed Zero Page
// Opcode:          $56
// Bytes:           2
// Cycles:          6
pub const LSR_ZPX: Byte = 0x56;

////////// Rotate Left (ROL) //////////
//
// Status Flags Affected: {N, Z, C}

// Addressing Mode: Accumulator
// Opcode:          $2A
// Bytes:           1
// Cycles:          2
pub const ROL_ACC: Byte = 0x2A;

// Addressing Mode: Absolute
// Opcode:          $2E
// Bytes:           3
// Cycles:          6
pub const ROL_ABS: Byte = 0x2E;

// Addressing Mode: X-Indexed Absolute
// Opcode:          $3E
// Bytes:           3
// Cycles:          7
pub const ROL_ABSX: Byte = 0x3E;

// Addressing Mode: Zero Page
// Opcode:          $26
// Bytes:           2
// Cycles:          5
pub const ROL_ZP: Byte = 0x26;

// Addressing Mode: X-Indexed Zero Page
// Opcode:          $36
// Bytes:           2
// Cycles:          6
pub const ROL_ZPX: Byte = 0x36;

////////// Jump to Subroutine (JSR) //////////
//
// Status Flags Affected: ∅

// Addressing Mode: Absolute
// Opcode:          $20
// Bytes:           3
// Cycles:          6
pub const JMP_SR: Byte = 0x20;
