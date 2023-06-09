use crate::memory::Memory;
use crate::instructions;

type Byte = u8;
type Word = u16;

const STACKPTR_BEGIN: Word = 0x0100;
const PROGRAM_COUNTER_BEGIN: Word = 0xFFFC;

#[allow(dead_code)]
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
    acc: Byte,
    x: Byte,
    y: Byte,
    status: Byte,
    stackptr: Word,
    cycles: u32,
    pc: Word,
}

impl std::fmt::Display for Mos6502 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "acc: {}\nx: {}\ny: {}\nstatus: {}\nstackptr: {}\npc: {}\ncycles: {}",
               self.acc, self.x, self.y, self.status, self.stackptr, self.pc, self.cycles)
    }
}

#[allow(dead_code)]
impl Mos6502 {
    pub fn new(cycles: Option<u32>) -> Self {
        let c = match cycles { Some(k) => k, _ => 0 };
        Self {
            acc: 0x00, x: 0x00, y: 0x00,
            status: 0x00, cycles: c,
            stackptr: STACKPTR_BEGIN,
            pc: PROGRAM_COUNTER_BEGIN,
        }
    }

    pub fn exe(&mut self, mem: &mut Memory) {
        while self.cycles > 0 {
            let instr: Byte = self.fetch_byte(mem);
            match instr {
                instructions::SED_IMP => self.set_decimal_mode(),
                instructions::SEI_IMP => self.set_interrupt_disable(),
                instructions::NOP_IMP => self.nop(),
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
        todo!("update cycles");
    }

    fn fetch_byte(&mut self, mem: &mut Memory) -> Byte {
        let byte = mem.get_byte(self.pc.into());
        self.pc += 1;
        self.cycles -= 1;
        byte.clone()
    }

    fn set_decimal_mode(&mut self) {
        self.status = Mos6502Flags::D.set(self.status);
        self.cycles -= 2;
    }

    fn set_interrupt_disable(&mut self) {
        self.status = Mos6502Flags::I.set(self.status);
        self.cycles -= 2;
    }

    fn nop(&mut self) {
        self.cycles -= 2;
    }
}
