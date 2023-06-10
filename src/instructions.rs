#![allow(dead_code)]

// Reference(s):
//  https://www.pagetable.com/c64ref/6502/?tab=2
//  https://www.masswerk.at/6502/6502_instruction_set.html

// (ﾉಥДಥ)ﾉ ︵┻━┻･/

type Byte = u8;

/* `~` = +1 if page is crossed */
/* `*` = +1 if branch is taken */

////////// Load Accumulator with Memory (LDA) //////////
//
// Status Flags Affected: {N, Z}

// Addressing Mode: Immediate
// Opcode:          $A9
// Bytes:           2
// Cycles:          2
// Status:          DONE
pub const LDA_IMM: Byte = 0xA9;
pub const LDA_IMM_COST: u32 = 2;

// Addressing Mode: Absolute
// Opcode:          $AD
// Bytes:           3
// Cycles:          4
pub const LDA_ABS: Byte = 0xAD;
pub const LDA_ABS_CCOST: u32 = 4;

// Addressing Mode: X-Indexed Absolute
// Opcode:          $BD
// Bytes:           3
// Cycles:          ~4
pub const LDA_ABSX: Byte = 0xBD;
pub const LDA_ABSX_CCOST: u32 = 4;

// Addressing Mode: Y-Indexed Absolute
// Opcode:          $B9
// Bytes:           3
// Cycles:          ~4
pub const LDA_ABSY: Byte = 0xB9;
pub const LDA_ABSY_CCOST: u32 = 4;

// Addressing Mode: Zero Page
// Opcode:          $A5
// Bytes:           2
// Cycles:          3
// Status:          DONE
pub const LDA_ZP: Byte = 0xA5;
pub const LDA_ZP_CCOST: u32 = 3;

// Addressing Mode: X-Indexed Zero Page
// Opcode:          $B5
// Bytes:           2
// Cycles:          4
// Status:          DONE
pub const LDA_ZPX: Byte = 0xB5;
pub const LDA_ZPX_CCOST: u32 = 4;

// Addressing Mode: X-Indexed Zero Page Indirect
// Opcode:          $A1
// Bytes:           2
// Cycles:          6
pub const LDA_ZPX_IND: Byte = 0xA1;
pub const LDA_ZPX_IND_CCOST: u32 = 6;

// Addressing Mode: Zero Page Indirect Y-Indexed
// Opcode:          $B1
// Bytes:           2
// Cycles:          ~5
pub const LDA_ZPY_IND: Byte = 0xB1;
pub const LDA_ZPY_IND_CCOST: u32 = 5;

////////// Load Index Register X from Memory (LDX) //////////
//
// Status Flags Affected: {N, Z}

// Addressing Mode: Immediate
// Opcode:          $A2
// Bytes:           2
// Cycles:          2
pub const LDX_IMM: Byte = 0xA2;
pub const LDX_IMM_CCOST: u32 = 2;

// Addressing Mode: Absolute
// Opcode:          $AE
// Bytes:           3
// Cycles:          4
pub const LDX_ABS: Byte = 0xAE;
pub const LDX_ABS_CCOST: u32 = 4;

// Addressing Mode: Y-Indexed Absolute
// Opcode:          $BE
// Bytes:           3
// Cycles:          ~4
pub const LDX_ABSY: Byte = 0xBE;
pub const LDX_ABSY_CCOST: u32 = 4;

// Addressing Mode: Zero Page
// Opcode:          $A6
// Bytes:           2
// Cycles:          3
pub const LDX_ZP: Byte = 0xA6;
pub const LDX_ZP_CCOST: u32 = 3;

// Addressing Mode: Y-Indexed Zero Page
// Opcode:          $B6
// Bytes:           2
// Cycles:          4
pub const LDX_ZPY: Byte = 0xB6;
pub const LDX_ZPY_CCOST: u32 = 4;

////////// Load Index Register Y from Memory (LDY) //////////
//
// Status Flags Affected: {N, Z}

// Addressing Mode: Immediate
// Opcode:          $A0
// Bytes:           2
// Cycles:          2
pub const LDY_IMM: Byte = 0xA0;
pub const LDY_IMM_CCOST: u32 = 2;

// Addressing Mode: Absolute
// Opcode:          $AC
// Bytes:           3
// Cycles:          4
pub const LDY_ABS: Byte = 0xAC;
pub const LDY_ABS_CCOST: u32 = 4;

// Addressing Mode: X-Indexed Absolute
// Opcode:          $BC
// Bytes:           3
// Cycles:          ~4
pub const LDY_ABSX: Byte = 0xBC;
pub const LDY_ABSX_CCOST: u32 = 4;

// Addressing Mode: Zero Page
// Opcode:          $A4
// Bytes:           2
// Cycles:          3
pub const LDY_ZP: Byte = 0xA4;
pub const LDY_ZP_CCOST: u32 = 3;

// Addressing Mode: X-Indexed Zero Page
// Opcode:          $B4
// Bytes:           2
// Cycles:          4
pub const LDY_ZPX: Byte = 0xB4;
pub const LDY_ZPX_CCOST: u32 = 4;

////////// Store Accumulator in Memory (STA) //////////
//
// Status Flags Affected: ∅

// Addressing Mode: Absolute
// Opcode:          $8D
// Bytes:           3
// Cycles:          4
pub const STA_ABS: Byte = 0x8D;
pub const STA_ABS_CCOST: u32 = 4;

// Addressing Mode: X-Indexed Absolute
// Opcode:          $9D
// Bytes:           3
// Cycles:          5
pub const STA_ABSX: Byte = 0x9D;
pub const STA_ABSX_CCOST: u32 = 5;

// Addressing Mode: Y-Indexed Absolute
// Opcode:          $99
// Bytes:           3
// Cycles:          5
pub const STA_ABSY: Byte = 0x99;
pub const STA_ABSY_CCOST: u32 = 5;

// Addressing Mode: Zero Page
// Opcode:          $85
// Bytes:           2
// Cycles:          3
pub const STA_ZP: Byte = 0x85;
pub const STA_ZP_CCOST: u32 = 3;

// Addressing Mode: X-Indexed Zero Page
// Opcode:          $95
// Bytes:           2
// Cycles:          4
pub const STA_ZPX: Byte = 0x95;
pub const STA_ZPX_CCOST: u32 = 4;

// Addressing Mode: X-Indexed Zero Page Indirect
// Opcode:          $81
// Bytes:           2
// Cycles:          6
pub const STA_ZPX_IND: Byte = 0x81;
pub const STA_ZPX_IND_CCOST: u32 = 6;

// Addressing Mode: Zero Page Indirect Y-Indexed
// Opcode:          $91
// Bytes:           2
// Cycles:          6
pub const STA_ZPY_IND: Byte = 0x91;
pub const STA_ZPY_IND_CCOST: u32 = 6;

////////// Store Index Register X in Memory (STX) //////////
//
// Status Flags Affected: ∅

// Addressing Mode: Absolute
// Opcode:          $91
// Bytes:           3
// Cycles:          4
pub const STX_ABS: Byte = 0x91;
pub const STX_ABS_CCOST: u32 = 4;

// Addressing Mode: Zero Page
// Opcode:          $86
// Bytes:           2
// Cycles:          3
pub const STX_ZP: Byte = 0x86;
pub const STX_ZP_CCOST: u32 = 3;

// Addressing Mode: Y-Indexed Zero Page
// Opcode:          $96
// Bytes:           2
// Cycles:          4
pub const STX_ZPY: Byte = 0x96;
pub const STX_ZPY_CCOST: u32 = 4;

////////// Store Index Register Y in Memory (STY) //////////
//
// Status Flags Affected: ∅

// Addressing Mode: Absolute
// Opcode:          $8C
// Bytes:           3
// Cycles:          4
pub const STY_ABS: Byte = 0x8C;
pub const STY_ABS_CCOST: u32 = 4;

// Addressing Mode: Zero Page
// Opcode:          $84
// Bytes:           2
// Cycles:          3
pub const STY_ZP: Byte = 0x84;
pub const STY_ZP_CCOST: u32 = 3;

// Addressing Mode: X-Indexed Zero Page
// Opcode:          $94
// Bytes:           2
// Cycles:          4
pub const STY_ZPX: Byte = 0x94;
pub const STY_ZPX_CCOST: u32 = 4;

////////// Transfer Accumulator to Index X (TAX) //////////
//
// Status Flags Affected: {N, Z}

// Addressing Mode: Implied
// Opcode:          $AA
// Bytes:           1
// Cycles:          2
pub const TAX_IMP: Byte = 0xAA;
pub const TAX_IMP_CCOST: u32 = 2;

////////// Transfer Accumulator to Index Y //////////
//
// Status Flags Affected: {N, Z}

// Addressing Mode: Implied
// Opcode:          $A8
// Bytes:           1
// Cycles:          2
pub const TAY_IMP: Byte = 0xA8;
pub const TAY_IMP_CCOST: u32 = 2;

////////// Transfer Stack Pointer to Index X (TSX) //////////
//
// Status Flags Affected: {N, Z}

// Addressing Mode: Implied
// Opcode:          $BA
// Bytes:           1
// Cycles:          2
pub const TSX_IMP: Byte = 0xBA;
pub const TSX_IMP_CCOST: u32 = 2;

////////// Transfer Index X to Accumulator (TXA) //////////
//
// Status Flags Affected: {N, Z}

// Addressing Mode: Implied
// Opcode:          $8A
// Bytes:           1
// Cycles:          2
pub const TXA_IMP: Byte = 0x8A;
pub const TXA_IMP_CCOST: u32 = 2;

////////// Transfer Index X to Stack Pointer (TXS) //////////
//
// Status Flags Affected: ∅

// Addressing Mode: Implied
// Opcode:          $9A
// Bytes:           1
// Cycles:          2
pub const TXS_IMP: Byte = 0x9A;
pub const TXS_IMP_CCOST: u32 = 2;

////////// Transfer Index Y to Accumulator (TYA) //////////
//
// Status Flags Affected: {N, Z}

// Addressing Mode: Implied
// Opcode:          $98
// Bytes:           1
// Cycles:          2
pub const TYA_IMP: Byte = 0x98;
pub const TYA_IMP_CCOST: u32 = 2;

////////// Push Accumulator on Stack (PHA) //////////
//
// Status Flags Affected: ∅

// Addressing Mode: Implied
// Opcode:          $48
// Bytes:           1
// Cycles:          3
pub const PHA_IMP: Byte = 0x48;
pub const PHA_IMP_CCOST: u32 = 3;

////////// Push Processor Status on Stack (PHP) //////////
//
// Status Flags Affected: ∅

// Addressing Mode: Implied
// Opcode:          $08
// Bytes:           1
// Cycles:          3
pub const PHP_IMP: Byte = 0x08;
pub const PHP_IMP_CCOST: u32 = 3;

////////// Pull Accumulator from Stack (PLA) //////////
//
// Status Flags Affected: {N, Z}

// Addressing Mode: Implied
// Opcode:          $68
// Bytes:           1
// Cycles:          4
pub const PLA_IMP: Byte = 0x68;
pub const PLA_IMP_CCOST: u32 = 4;

////////// Pull Processor Status from Stack (PLP) //////////
//
// Status Flags Affected: {N, V, D, I, Z, C}

// Addressing Mode: Implied
// Opcode:          $28
// Bytes:           1
// Cycles:          4
pub const PLP_IMP: Byte = 0x28;
pub const PLP_IMP_CCOST: u32 = 4;

////////// Arithmetic Shift Left (ASL) //////////
//
// Status Flags Affected: {N, Z, C}

// Addressing Mode: Accumulator
// Opcode:          $0A
// Bytes:           1
// Cycles:          2
pub const ASL_ACC: Byte = 0x0A;
pub const ASL_ACC_CCOST: u32 = 2;

// Addressing Mode: Absolute
// Opcode:          $0E
// Bytes:           3
// Cycles:          6
pub const ASL_ABS: Byte = 0x0E;
pub const ASL_ABS_CCOST: u32 = 6;

// Addressing Mode: X-Indexed Absolute
// Opcode:          $1E
// Bytes:           3
// Cycles:          7
pub const ASL_ABSX: Byte = 0x1E;
pub const ASL_ABSX_CCOST: u32 = 7;

// Addressing Mode: Zero Page
// Opcode:          $06
// Bytes:           2
// Cycles:          5
pub const ASL_ZP: Byte = 0x06;
pub const ASL_ZP_CCOST: u32 = 5;

// Addressing Mode: X-Indexed Zero Page
// Opcode:          $16
// Bytes:           2
// Cycles:          6
pub const ASL_ZPX: Byte = 0x16;
pub const ASL_ZPX_CCOST: u32 = 6;

////////// Logical Shift Right (LSR) //////////
//
// Status Flags Affected: {(N: 0), Z, C}

// Addressing Mode: Accumulator
// Opcode:          $4A
// Bytes:           1
// Cycles:          2
pub const LSR_ACC: Byte = 0x4A;
pub const LSR_ACC_CCOST: u32 = 2;

// Addressing Mode: Absolute
// Opcode:          $4E
// Bytes:           3
// Cycles:          6
pub const LSR_ABS: Byte = 0x4E;
pub const LSR_ABS_CCOST: u32 = 6;

// Addressing Mode: X-Indexed Absolute
// Opcode:          $5E
// Bytes:           3
// Cycles:          7
pub const LSR_ABSX: Byte = 0x5E;
pub const LSR_ABSX_CCOST: u32 = 7;

// Addressing Mode: Zero Page
// Opcode:          $46
// Bytes:           2
// Cycles:          5
pub const LSR_ZP: Byte = 0x46;
pub const LSR_ZP_CCOST: u32 = 5;

// Addressing Mode: X-Indexed Zero Page
// Opcode:          $56
// Bytes:           2
// Cycles:          6
pub const LSR_ZPX: Byte = 0x56;
pub const LSR_ZPX_CCOST: u32 = 6;

////////// Rotate Left (ROL) //////////
//
// Status Flags Affected: {N, Z, C}

// Addressing Mode: Accumulator
// Opcode:          $2A
// Bytes:           1
// Cycles:          2
pub const ROL_ACC: Byte = 0x2A;
pub const ROL_ACC_CCOST: u32 = 2;

// Addressing Mode: Absolute
// Opcode:          $2E
// Bytes:           3
// Cycles:          6
pub const ROL_ABS: Byte = 0x2E;
pub const ROL_ABS_CCOST: u32 = 6;

// Addressing Mode: X-Indexed Absolute
// Opcode:          $3E
// Bytes:           3
// Cycles:          7
pub const ROL_ABSX: Byte = 0x3E;
pub const ROL_ABSX_CCOST: u32 = 7;

// Addressing Mode: Zero Page
// Opcode:          $26
// Bytes:           2
// Cycles:          5
pub const ROL_ZP: Byte = 0x26;
pub const ROL_ZP_CCOST: u32 = 5;

// Addressing Mode: X-Indexed Zero Page
// Opcode:          $36
// Bytes:           2
// Cycles:          6
pub const ROL_ZPX: Byte = 0x36;
pub const ROL_ZPX_CCOST: u32 = 6;

////////// Rotate Right (ROR) //////////
//
// Status Flags Affected: {N, Z, C}

// Addressing Mode: Accumulator
// Opcode:          $6A
// Bytes:           1
// Cycles:          2
pub const ROR_ACC: Byte = 0x6A;
pub const ROR_ACC_CCOST: u32 = 2;

// Addressing Mode: Absolute
// Opcode:          $6E
// Bytes:           3
// Cycles:          6
pub const ROR_ABS: Byte = 0x6E;
pub const ROR_ABS_CCOST: u32 = 6;

// Addressing Mode: X-Indexed Absolute
// Opcode:          $7E
// Bytes:           3
// Cycles:          7
pub const ROR_ABSX: Byte = 0x7E;
pub const ROR_ABSX_CCOST: u32 = 7;

// Addressing Mode: Zero Page
// Opcode:          $66
// Bytes:           2
// Cycles:          5
pub const ROR_ZP: Byte = 0x66;
pub const ROR_ZP_CCOST: u32 = 5;

// Addressing Mode: X-Indexed Zero Page
// Opcode:          $76
// Bytes:           2
// Cycles:          6
pub const ROR_ZPX: Byte = 0x76;
pub const ROR_ZPX_CCOST: u32 = 6;

////////// AND Memory with Accumulator (AND) //////////
//
// Status Flags Affected: {N, Z}

// Addressing Mode: Immediate
// Opcode:          $29
// Bytes:           2
// Cycles:          2
pub const AND_IMM: Byte = 0x29;
pub const AND_IMM_CCOST: u32 = 2;

// Addressing Mode: Absolute
// Opcode:          $2D
// Bytes:           3
// Cycles:          4
pub const AND_ABS: Byte = 0x2D;
pub const AND_ABS_CCOST: u32 = 4;

// Addressing Mode: X-Indexed Absolute
// Opcode:          $3D
// Bytes:           3
// Cycles:          ~4
pub const AND_ABSX: Byte = 0x3D;
pub const AND_ABSX_CCOST: u32 = 4;

// Addressing Mode: Y-Indexed Absolute
// Opcode:          $39
// Bytes:           3
// Cycles:          ~4
pub const AND_ABSY: Byte = 0x39;
pub const AND_ABSY_CCOST: u32 = 4;

// Addressing Mode: Zero Page
// Opcode:          $25
// Bytes:           2
// Cycles:          3
pub const AND_ZP: Byte = 0x25;
pub const AND_ZP_CCOST: u32 = 3;

// Addressing Mode: X-Indexed Zero Page
// Opcode:          $35
// Bytes:           2
// Cycles:          4
pub const AND_ZPX: Byte = 0x35;
pub const AND_ZPX_CCOST: u32 = 4;

// Addressing Mode: X-Indexed Zero Page Indirect
// Opcode:          $21
// Bytes:           2
// Cycles:          6
pub const AND_ZPX_IND: Byte = 0x21;
pub const AND_ZPX_IND_CCOST: u32 = 6;

// Addressing Mode: Zero Page Indirect Y-Index
// Opcode:          $31
// Bytes:           2
// Cycles:          ~5
pub const AND_ZPY_IND: Byte = 0x31;
pub const AND_ZPY_IND_CCOST: u32 = 5;

////////// Test Bits in Memory with Accumulator (BIT) //////////
//
// Status Flags Affected: {N, V, Z}

// Addressing Mode: Absolute
// Opcode:          $2C
// Bytes:           3
// Cycles:          4
pub const BIT_ABS: Byte = 0x2C;
pub const BIT_ABS_CCOST: u32 = 4;

// Addressing Mode: Zero Page
// Opcode:          $24
// Bytes:           2
// Cycles:          3
pub const BIT_ZP: Byte = 0x24;
pub const BIT_ZP_CCOST: u32 = 3;

////////// Exclusive OR Memory with Accumulator (EOR) //////////

// Addressing Mode: Immediate
// Opcode:          $49
// Bytes:           2
// Cycles:          2
pub const EOR_IMM: Byte = 0x49;
pub const EOR_IMM_CCOST: u32 = 2;

// Addressing Mode: Absolute
// Opcode:          $4D
// Bytes:           3
// Cycles:          4
pub const EOR_ABS: Byte = 0x4D;
pub const EOR_ABS_CCOST: u32 = 4;

// Addressing Mode: X-Indexed Absolute
// Opcode:          $5D
// Bytes:           3
// Cycles:          ~4
pub const EOR_ABSX: Byte = 0x5D;
pub const EOR_ABSX_CCOST: u32 = 4;

// Addressing Mode: Y-Indexed Absolute
// Opcode:          $59
// Bytes:           3
// Cycles:          ~4
pub const EOR_ABSY: Byte = 0x59;
pub const EOR_ABSY_CCOST: u32 = 4;

// Addressing Mode: Zero Page
// Opcode:          $45
// Bytes:           2
// Cycles:          3
pub const EOR_ZP: Byte = 0x45;
pub const EOR_ZP_CCOST: u32 = 3;

// Addressing Mode: X-Indexed Zero Page
// Opcode:          $55
// Bytes:           2
// Cycles:          4
pub const EOR_ZPX: Byte = 0x55;
pub const EOR_ZPX_CCOST: u32 = 4;

// Addressing Mode: X-Indexed Zero Page Indirect
// Opcode:          $41
// Bytes:           2
// Cycles:          6
pub const EOR_ZPX_IND: Byte = 0x41;
pub const EOR_ZPX_IND_CCOST: u32 = 6;

// Addressing Mode: Zero Page Indirect Y-Indexed
// Opcode:          $51
// Bytes:           2
// Cycles:          ~5
pub const EOR_ZPY_IND: Byte = 0x51;
pub const EOR_ZPY_IND_CCOST: u32 = 5;

////////// OR Memory with Accumulator (ORA) //////////
//
// Status Flags Affected: {N, Z}

// Addressing Mode: Immediate
// Opcode:          $09
// Bytes:           2
// Cycles:          2
pub const ORA_IMM: Byte = 0x09;
pub const ORA_IMM_CCOST: u32 = 2;

// Addressing Mode: Absolute
// Opcode:          $0D
// Bytes:           3
// Cycles:          4
pub const ORA_ABS: Byte = 0x0D;
pub const ORA_ABS_CCOST: u32 = 4;

// Addressing Mode: X-Indexed Absolute
// Opcode:          $1D
// Bytes:           3
// Cycles:          ~4
pub const ORA_ABSX: Byte = 0x1D;
pub const ORA_ABSX_CCOST: u32 = 4;

// Addressing Mode: Y-Indexed Absolute
// Opcode:          $19
// Bytes:           3
// Cycles:          ~4
pub const ORA_ABSY: Byte = 0x19;
pub const ORA_ABSY_CCOST: u32 = 4;

// Addressing Mode: Zero Page
// Opcode:          $05
// Bytes:           2
// Cycles:          3
pub const ORA_ZP: Byte = 0x05;
pub const ORA_ZP_CCOST: u32 = 3;

// Addressing Mode: X-Indexed Zero Page
// Opcode:          $15
// Bytes:           2
// Cycles:          4
pub const ORA_ZPX: Byte = 0x15;
pub const ORA_ZPX_CCOST: u32 = 4;

// Addressing Mode: X-Indexed Zero Page Indirect
// Opcode:          $01
// Bytes:           2
// Cycles:          6
pub const ORA_ZPX_IND: Byte = 0x01;
pub const ORA_ZPX_IND_CCOST: u32 = 6;

// Addressing Mode: Zero Page Indirect Y-Indexed
// Opcode:          $11
// Bytes:           2
// Cycles:          ~5
pub const ORA_ZPY_IND: Byte = 0x11;
pub const ORA_ZPY_IND_CCOST: u32 = 5;

////////// Add Memory to Accumulator with Carry (ADC) //////////
//
// Status Flags Affected: {N, V, Z, C}
//
// Note: bug with the 6502

// Addressing Mode: Immediate
// Opcode:          $69
// Bytes:           2
// Cycles:          2
pub const ADC_IMM: Byte = 0x69;
pub const ADC_IMM_CCOST: u32 = 2;

// Addressing Mode: Absolute
// Opcode:          $6D
// Bytes:           3
// Cycles:          4
pub const ADC_ABS: Byte = 0x6D;
pub const ADC_ABS_CCOST: u32 = 4;

// Addressing Mode: X-Indexed Absolute
// Opcode:          $7D
// Bytes:           3
// Cycles:          ~4
pub const ADC_ABSX: Byte = 0x7D;
pub const ADC_ABSX_CCOST: u32 = 4;

// Addressing Mode: Y-Indexed Absolute
// Opcode:          $79
// Bytes:           3
// Cycles:          ~4
pub const ADC_ABSY: Byte = 0x79;
pub const ADC_ABSY_CCOST: u32 = 4;

// Addressing Mode: Zero Page
// Opcode:          $65
// Bytes:           2
// Cycles:          3
pub const ADC_ZP: Byte = 0x65;
pub const ADC_ZP_CCOST: u32 = 3;

// Addressing Mode: X-Indexed Zero Page
// Opcode:          $75
// Bytes:           2
// Cycles:          4
pub const ADC_ZPX: Byte = 0x75;
pub const ADC_ZPX_CCOST: u32 = 4;

// Addressing Mode: X-Indexed Zero Page Indirect
// Opcode:          $61
// Bytes:           2
// Cycles:          6
pub const ADC_ZPX_IND: Byte = 0x61;
pub const ADC_ZPX_IND_CCOST: u32 = 6;

// Addressing Mode: Zero Page Indirect Y-Indexed
// Opcode:          $71
// Bytes:           2
// Cycles:          ~5
pub const ADC_ZPY_IND: Byte = 0x71;
pub const ADC_ZPY_IND_CCOST: u32 = 5;

////////// Compare Memory and Accumulator (CMP) //////////
//
// Status Flags Affected: {N, Z, C}

// Addressing Mode: Immediate
// Opcode:          $C9
// Bytes:           2
// Cycles:          2
pub const CMP_IMM: Byte = 0xC9;
pub const CMP_IMM_CCOST: u32 = 2;

// Addressing Mode: Absolute
// Opcode:          $CD
// Bytes:           2
// Cycles:          4
pub const CMP_ABS: Byte = 0xCD;
pub const CMP_ABS_CCOST: u32 = 4;

// Addressing Mode: X-Indexed Absolute
// Opcode:          $DD
// Bytes:           3
// Cycles:          ~4
pub const CMP_ABSX: Byte = 0xDD;
pub const CMP_ABSX_CCOST: u32 = 4;

// Addressing Mode: Y-Indexed Absolute
// Opcode:          $D9
// Bytes:           3
// Cycles:          ~4
pub const CMP_ABSY: Byte = 0xD9;
pub const CMP_ABSY_CCOST: u32 = 4;

// Addressing Mode: Zero Page
// Opcode:          $C5
// Bytes:           2
// Cycles:          3
pub const CMP_ZP: Byte = 0xC5;
pub const CMP_ZP_CCOST: u32 = 3;

// Addressing Mode: X-Indexed Zero Page
// Opcode:          $D5
// Bytes:           2
// Cycles:          4
pub const CMP_ZPX: Byte = 0xD5;
pub const CMP_ZPX_CCOST: u32 = 4;

// Addressing Mode: X-Indexed Zero Page Indirect
// Opcode:          $C1
// Bytes:           2
// Cycles:          6
pub const CMP_ZPX_IND: Byte = 0xC1;
pub const CMP_ZPX_IND_CCOST: u32 = 6;

// Addressing Mode: Zero Page Indirect Y-Indexed
// Opcode:          $D1
// Bytes:           2
// Cycles:          ~5
pub const CMP_ZPY_IND: Byte = 0xD1;
pub const CMP_ZPY_IND_CCOST: u32 = 5;

////////// Compare Index Register X to Memory (CPX) //////////
//
// Status Flags Affected: {N, Z, C}

// Addressing Mode: Immediate
// Opcode:          $E0
// Bytes:           2
// Cycles:          2
pub const CPX_IMM: Byte = 0xE0;
pub const CPX_IMM_CCOST: u32 = 2;

// Addressing Mode: Absolute
// Opcode:          $EC
// Bytes:           3
// Cycles:          4
pub const CPX_ABS: Byte = 0xEC;
pub const CPX_ABS_CCOST: u32 = 4;

// Addressing Mode: Zero Page
// Opcode:          $E4
// Bytes:           2
// Cycles:          3
pub const CPX_ZP: Byte = 0xE4;
pub const CPX_ZP_CCOST: u32 = 3;

////////// Compare Index Register Y to Memory (CPY) //////////
//
// Status Flags Affected: {N, Z, C}

// Addressing Mode: Immediate
// Opcode:          $C0
// Bytes:           2
// Cycles:          2
pub const CPY_IMM: Byte = 0xC0;
pub const CPY_IMM_CCOST: u32 = 2;

// Addressing Mode: Absolute
// Opcode:          $CC
// Bytes:           3
// Cycles:          4
pub const CPY_ABS: Byte = 0xCC;
pub const CPY_ABS_CCOST: u32 = 4;

// Addressing Mode: Zero Page
// Opcode:          $C4
// Bytes:           2
// Cycles:          3
pub const CPY_ZP: Byte = 0xC4;
pub const CPY_ZP_CCOST: u32 = 3;

////////// Subtract Memory from Accumulator with Borrow (SBC) //////////
//
// Status Flags Affected: {N, V, Z, C}
//
// Note: bug with the 6502

// Addressing Mode: Immediate
// Opcode:          $E9
// Bytes:           2
// Cycles:          2
pub const SBC_IMM: Byte = 0xE9;
pub const SBC_IMM_CCOST: u32 = 2;

// Addressing Mode: Absolute
// Opcode:          $ED
// Bytes:           3
// Cycles:          4
pub const SBC_ABS: Byte = 0xED;
pub const SBC_ABS_CCOST: u32 = 4;

// Addressing Mode: X-Indexed Absolute
// Opcode:          $FD
// Bytes:           3
// Cycles:          ~4
pub const SBC_ABSX: Byte = 0xFD;
pub const SBC_ABSX_CCOST: u32 = 4;

// Addressing Mode: Y-Indexed Absolute
// Opcode:          $F9
// Bytes:           3
// Cycles:          ~4
pub const SBC_ABSY: Byte = 0xF9;
pub const SBC_ABSY_CCOST: u32 = 4;

// Addressing Mode: Zero Page
// Opcode:          $E5
// Bytes:           2
// Cycles:          3
pub const SBC_ZP: Byte = 0xE5;
pub const SBC_ZP_CCOST: u32 = 3;

// Addressing Mode: X-Indexed Zero Page
// Opcode:          $F5
// Bytes:           2
// Cycles:          4
pub const SBC_ZPX: Byte = 0xF5;
pub const SBC_ZPX_CCOST: u32 = 4;

// Addressing Mode: X-Indexed Zero Page Indirect
// Opcode:          $E1
// Bytes:           2
// Cycles:          6
pub const SBC_ZPX_IND: Byte = 0xE1;
pub const SBC_ZPX_IND_CCOST: u32 = 6;

// Addressing Mode: Zero Page Indirect Y-Indexed
// Opcode:          $F1
// Bytes:           2
// Cycles:          ~5
pub const SBC_ZPY_IND: Byte = 0xF1;
pub const SBC_ZPY_IND_CCOST: u32 = 5;

////////// Decrement Memory by One (DEC) //////////
//
// Status Flags Affected: {N, Z}

// Addressing Mode: Absolute
// Opcode:          $CE
// Bytes:           3
// Cycles:          6
pub const DEC_ABS: Byte = 0xCE;
pub const DEC_ABS_CCOST: u32 = 6;

// Addressing Mode: X-Indexed Absolute
// Opcode:          $DE
// Bytes:           3
// Cycles:          7
pub const DEC_ABSX: Byte = 0xDE;
pub const DEC_ABSX_CCOST: u32 = 7;

// Addressing Mode: Zero Page
// Opcode:          $C6
// Bytes:           2
// Cycles:          5
pub const DEC_ZP: Byte = 0xC6;
pub const DEC_ZP_CCOST: u32 = 5;

// Addressing Mode: X-Indexed Zero Page
// Opcode:          $D6
// Bytes:           2
// Cycles:          6
pub const DEC_ZPX: Byte = 0xD6;
pub const DEC_ZPX_CCOST: u32 = 6;

////////// Decrement Index Register X by One (DEX) //////////
//
// Status Flags Affected: {N, Z}

// Addressing Mode: Implied
// Opcode:          $CA
// Bytes:           1
// Cycles:          2
pub const DEX_IMP: Byte = 0xCA;
pub const DEX_IMP_CCOST: u32 = 2;

////////// Decrement Index Register Y by One (DEY) //////////
//
// Status Flags Affected: {N, Z}

// Addressing Mode: Implied
// Opcode:          $88
// Bytes:           1
// Cycles:          2
pub const DEY_IMP: Byte = 0x88;
pub const DEY_IMP_CCOST: u32 = 2;

////////// Increment Memory by One (INC) //////////
//
// Status Flags Affected: {N, Z}

// Addressing Mode: Absolute
// Opcode:          $EE
// Bytes:           3
// Cycles:          6
pub const INC_ABS: Byte = 0xEE;
pub const INC_ABS_CCOST: u32 = 6;

// Addressing Mode: X-Indexed Absolute
// Opcode:          $FE
// Bytes:           3
// Cycles:          7
pub const INC_ABSX: Byte = 0xFE;
pub const INC_ABSX_CCOST: u32 = 7;

// Addressing Mode: Zero Page
// Opcode:          $E6
// Bytes:           2
// Cycles:          5
pub const INC_ZP: Byte = 0xE6;
pub const INC_ZP_CCOST: u32 = 5;

// Addressing Mode: X-Indexed Zero Page
// Opcode:          $F6
// Bytes:           2
// Cycles:          6
pub const INC_ZPX: Byte = 0xF6;
pub const INC_ZPX_CCOST: u32 = 6;

////////// Increment Index Register X by One (INX) //////////
//
// Status Flags Affected: {N, Z}

// Addressing Mode: Implied
// Opcode:          $E8
// Bytes:           1
// Cycles:          2
pub const INX_IMP: Byte = 0xE8;
pub const INX_IMP_CCOST: u32 = 2;

////////// Increment Index Register Y by One (INY) //////////
//
// Status Flags Affected: {N, Z}

// Addressing Mode: Implied
// Opcode:          $C8
// Bytes:           1
// Cycles:          2
pub const INY_IMP: Byte = 0xC8;
pub const INY_IMP_CCOST: u32 = 2;

////////// Break Command (BRK) //////////
//
// Status Flags Affected: {I}
//
// Note: If an IRQ happens at the same time as BRK, the BRK will be ignored.

// Addressing Mode: Implied
// Opcode:          $00
// Bytes:           1
// Cycles:          7
pub const BRK_IMP: Byte = 0x00;
pub const BRK_IMP_CCOST: u32 = 7;

////////// JMP Indirect (JMP) //////////
//
// Status Flags Affected: ∅

// Addressing Mode: Absolute
// Opcode:          $4C
// Bytes:           3
// Cycles:          3
pub const JMP_ABS: Byte = 0x4C;
pub const JMP_ABS_CCOST: u32 = 3;

// Addressing Mode: Absolute Indirect
// Opcode:          $6C
// Bytes:           3
// Cycles:          5
pub const JMP_ABS_IND: Byte = 0x6C;
pub const JMP_ABS_IND_CCOST: u32 = 5;

////////// Jump to Subroutine (JSR) //////////
//
// Status Flags Affected: ∅

// Addressing Mode: Absolute
// Opcode:          $20
// Bytes:           3
// Cycles:          6
pub const JMP_SR_ABS: Byte = 0x20;
pub const JMP_SR_ABS_CCOST: u32 = 6;

////////// Return From Interrupt (RTI) //////////
//
// Status Flags Affected: {N, V, D, I, Z, C}

// Addressing Mode: Implied
// Opcode:          $40
// Bytes:           1
// Cycles:          6
pub const RTI_IMP: Byte = 0x40;
pub const RTI_IMP_CCOST: u32 = 6;

////////// Return From Subroutine (RTS) //////////
//
// Status Flags Affected: ∅

// Addressing Mode: Implied
// Opcode:          $60
// Bytes:           1
// Cycles:          6
pub const RTS_IMP: Byte = 0x60;
pub const RTS_IMP_CCOST: u32 = 6;

////////// Branch on Carry Clear (BCC) //////////
//
// Status Flags Affected: ∅

// Addressing Mode: Relative
// Opcode:          $90
// Bytes:           2
// Cycles:          ~*2
pub const BCC_REL_CCOST: u32 = 2;

////////// Branch on Carry Set (BCS) //////////
//
// Status Flags Affected: ∅

// Addressing Mode: Relative
// Opcode:          $B0
// Bytes:           2
// Cycles:          ~*2
pub const BCS_REL: Byte = 0xB0;
pub const BCS_REL_CCOST: u32 = 2;

////////// Branch on Result Zero (BEQ) //////////
//
// Status Flags Affected: ∅

// Addressing Mode: Relative
// Opcode:          $F0
// Bytes:           2
// Cycles:          ~*2
pub const BEQ_REL: Byte = 0xF0;
pub const BEQ_REL_CCOST: u32 = 2;

////////// Branch on Result Minus (BMI) //////////
//
// Status Flags Affected: ∅

// Addressing Mode: Relative
// Opcode:          $30
// Bytes:           2
// Cycles:          ~*2
pub const BMI_REL: Byte = 0x30;
pub const BMI_REL_CCOST: u32 = 2;

////////// Branch on Result Not Zero (BNE) //////////
//
// Status Flags Affected: ∅

// Addressing Mode: Relative
// Opcode:          $D0
// Bytes:           2
// Cycles:          ~*2
pub const BNE_REL: Byte = 0xD0;
pub const BNE_REL_CCOST: u32 = 2;

////////// Branch on Result Plus (BPL) //////////
//
// Status Flags Affected: ∅

// Addressing Mode: Relative
// Opcode:          $10
// Bytes:           2
// Cycles:          ~*2
pub const BPL_REL: Byte = 0x10;
pub const BPL_REL_CCOST: u32 = 2;

////////// Branch on Overflow Clear (BVC) //////////
//
// Status Flags Affected: ∅

// Addressing Mode: Relative
// Opcode:          $50
// Bytes:           2
// Cycles:          ~*2
pub const BVC_REL: Byte = 0x50;
pub const BVC_REL_CCOST: u32 = 2;

////////// Branch on Overflow Set (BVS) //////////
//
// Status Flags Affected: ∅

// Addressing Mode: Relative
// Opcode:          $70
// Bytes:           2
// Cycles:          ~*2
pub const BVS_REL: Byte = 0x70;
pub const BVS_REL_CCOST: u32 = 2;

////////// Clear Carry Flag (CLC) //////////
//
// Status Flags Affected: {(C: 0)}

// Addressing Mode: Implied
// Opcode:          $18
// Bytes:           1
// Cycles:          2
// Status:          DONE
pub const CLC_IMP: Byte = 0x18;
pub const CLC_IMP_CCOST: u32 = 2;

////////// Clear Decimal Mode (CLD) //////////
//
// Status Flags Affected: {(D: 0)}
//
// Note: The value of the decimal mode flag is indeterminate after a RESET.

// Addressing Mode: Implied
// Opcode:          $D8
// Bytes:           1
// Cycles:          2
// Status:          DONE
pub const CLD_IMP: Byte = 0xD8;
pub const CLD_IMP_CCOST: u32 = 2;

////////// Clear Interrupt Disable (CLI) //////////
//
// Status Flags Affected: {(I: 0)}

// Addressing Mode: Implied
// Opcode:          $58
// Bytes:           1
// Cycles:          2
// Status:          DONE
pub const CLI_IMP: Byte = 0x58;
pub const CLI_IMP_CCOST: u32 = 2;

////////// Clear Overflow Flag (CLV) //////////
//
// Status Flags Affected: {(V: 0)}

// Addressing Mode: Implied
// Opcode:          $B8
// Bytes:           1
// Cycles:          2
// Status:          DONE
pub const CLV_IMP: Byte = 0xB8;
pub const CLV_IMP_CCOST: u32 = 2;

////////// Set Carry Flag (SEC) //////////
//
// Status Flags Affected: {(C: 1)}

// Addressing Mode: Implied
// Opcode:          $38
// Bytes:           1
// Cycles:          2
// Status:          DONE
pub const SEC_IMP: Byte = 0x38;
pub const SEC_IMP_CCOST: u32 = 2;

////////// Set Decimal Mode (SED) //////////
//
// Status Flags Affected: {(D: 1)}

// Addressing Mode: Implied
// Opcode:          $F8
// Bytes:           1
// Cycles:          2
// Status:          DONE
pub const SED_IMP: Byte = 0xF8;
pub const SED_IMP_CCOST: u32 = 2;

////////// Set Interrupt Disable Status (SEI) //////////
//
// Status Flags Affected: {(I: 1)}

// Addressing Mode: Implied
// Opcode:          $78
// Bytes:           1
// Cycles:          2
// Status:          DONE
pub const SEI_IMP: Byte = 0x78;
pub const SEI_IMP_CCOST: u32 = 2;

////////// No Operation (NOP) //////////
//
// Status Flags Affected: ∅

// Addressing Mode: Implied
// Opcode:          $EA
// Bytes:           1
// Cycles:          2
// Status:          DONE
pub const NOP_IMP: Byte = 0xEA;
pub const NOP_IMP_CCOST: u32 = 2;
