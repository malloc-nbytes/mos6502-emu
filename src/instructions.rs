// Reference(s):
//  https://www.pagetable.com/c64ref/6502/?tab=2

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

////////// Jump to Subroutine (JSR) //////////
//
// Status Flags Affected: ∅

// Addressing Mode: Absolute
// Opcode:          $20
// Bytes:           3
// Cycles:          6
pub const JMP_SR: Byte = 0x20;
