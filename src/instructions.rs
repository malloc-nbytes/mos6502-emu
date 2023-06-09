// Reference(s):
//  https://www.pagetable.com/c64ref/6502/?tab=2

type Byte = u8;

////////// Load Accumulator with Memory (LDA) //////////

// Status Flags Affected: {N, Z}
// k = 1 if page is crossed

// -----------------------------
// Addressing Mode: Immediate
// Opcode:          $A9
// Bytes:           2
// Cycles:          2
// -----------------------------
pub const LDA_IMM: Byte = 0xA9;

// -----------------------------
// Addressing Mode: Absolute
// Opcode:          $AD
// Bytes:           3
// Cycles:          4
// -----------------------------
pub const LDA_ABS: Byte = 0xAD;

// -----------------------------------
// Addressing Mode: X-Indexed Absolute
// Opcode:          $BD
// Bytes:           3
// Cycles:          4 + k
pub const LDA_ABSX: Byte = 0xBD;
// -----------------------------------

// -----------------------------------
// Addressing Mode: Y-Indexed Absolute
// Opcode:          $B9
// Bytes:           3
// Cycles:          4 + k
// -----------------------------------
pub const LDA_ABSY: Byte = 0xB9;

// ------------------------------
// Addressing Mode: Zero Page
// Opcode:          $A5
// Bytes:           2
// Cycles:          3
// ------------------------------
pub const LDA_ZP: Byte = 0xA5;

// ------------------------------------
// Addressing Mode: X-Indexed Zero Page
// Opcode:          $B5
// Bytes:           2
// Cycles:          4
// ------------------------------------
pub const LDA_ZPX: Byte = 0xB5;

// ---------------------------------------------
// Addressing Mode: X-Indexed Zero Page Indirect
// Opcode:          $A1
// Bytes:           2
// Cycles:          6
// ---------------------------------------------
pub const LDA_ZPX_IND: Byte = 0xA1;

// ---------------------------------------------
// Addressing Mode: Zero Page Indirect Y-Indexed
// Opcode:          $B1
// Bytes:           2
// Cycles:          5 + k
// ---------------------------------------------
pub const LDA_ZPY_IND: Byte = 0xB1;

////////// Jump to Subroutine (JSR) //////////

// Status Flags Affected: {}

// ------------------------------
// Addressing Mode: Absolute
// Opcode:          $20
// Bytes:           3
// Cycles:          6
// ------------------------------
pub const JMP_SR: Byte = 0x20;


