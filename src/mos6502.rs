use crate::memory::Memory;

type Byte = u8;
type Word = u16;

// LDA //
const LDA_IMM: Byte = 0xA9; // Instruction Load Accumulator Immediate Mode.
const LDA_ZP:  Byte = 0xA5; // Instruction Load Accumulator Zero Page.
const LDA_ZPX: Byte = 0xB5; // Instruction Load Accumulator Zero Page X.

// JMP //
const JMP_SR: Byte = 0x20; // Instruction Jump to Subroutine.

enum Mos6502Flags {
    C = 0, // Carry bit.
    Z = 1, // Zero.
    I = 2, // Disable interrupts.
    D = 3, // Decimal mode (unused).
    B = 4, // Break.
    U = 5, // Unused.
    V = 6, // Overflow.
    N = 7, // Negative.
}

impl Mos6502Flags {
    pub fn set(self, src: Byte) -> Byte {
        src | 1u8 << (self as Byte)
    }

    pub fn unset(self, src: Byte) -> Byte {
        src & !(1u8 << (self as Byte))
    }
}

pub struct Mos6502 {
    acc:      Byte,
    x:        Byte,
    y:        Byte,
    status:   Byte,
    stackptr: Word,
    pc:       Word,
}

impl std::fmt::Display for Mos6502 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "acc: {}\nx: {}\ny: {}\nstatus: {}\nstackptr: {}\npc: {}",
               self.acc, self.x, self.y, self.status, self.stackptr, self.pc)
    }
}

impl Mos6502 {
    pub fn new() -> Self {
        Self {
            acc: 0x00, x: 0x00,
            y: 0x00, stackptr: 0x0100,
            status: 0x00, pc: 0xFFFC,
        }
    }

    pub fn dump(&self) {
        println!("acc: {}",      self.acc);
        println!("x: {}",        self.x);
        println!("y: {}",        self.y);
        println!("status: {}",   self.status);
        println!("stackptr: {}", self.stackptr);
        println!("pc: {}",       self.pc);
    }

    pub fn exe(&mut self, mem: &mut Memory, mut cycles: u32) {
        while cycles > 0 {
            let instr: Byte = self.fetch_byte(mem, &mut cycles);
            match instr {
                LDA_IMM => todo!(),
                LDA_ZP => todo!(),
                LDA_ZPX => todo!(),
                JMP_SR => todo!(),
                _ => panic!("Unhandled instruction {}", instr),
            }
        }
    }

    fn lda_set_status(&mut self) {
        self.status = match self.acc == 0 {
            true => Mos6502Flags::Z.set(self.status),
            false => Mos6502Flags::Z.unset(self.status),
        };
        self.status = match self.acc & 0b1000_0000 != 0 {
            true => Mos6502Flags::N.set(self.status),
            false => Mos6502Flags::N.unset(self.status),
        };
    }

    fn fetch_byte(&mut self, mem: &mut Memory, cycles: &mut u32) -> Byte {
        let byte: &mut Byte = mem.get_byte(self.pc as usize);
        self.pc += 1;
        *cycles -= 1;
        byte.clone()
    }

}


