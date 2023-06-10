use crate::memory::Memory;
use crate::instructions;

type Byte = u8;
type Word = u16;

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
    pub fn set(self, status: &mut Byte) {
        *status |= 1u8 << (self as Byte);
    }

    pub fn unset(self, status: &mut Byte) {
        *status &= !(1u8 << (self as Byte));
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

const STACKPTR_BEGIN: Word = 0x0100;
const PROGRAM_COUNTER_BEGIN: Word = 0xFFFC;

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
                instructions::CLC_IMP => self.clear_carry_flag(instructions::CLC_IMP_CCOST),
                instructions::CLD_IMP => self.clear_decimal_mode(instructions::CLD_IMP_CCOST),
                instructions::CLI_IMP => self.clear_interrupt_disable(instructions::CLI_IMP_CCOST),
                instructions::CLV_IMP => self.clear_overflow_flag(instructions::CLV_IMP_CCOST),
                instructions::SEC_IMP => self.set_carry_flag(instructions::SEC_IMP_CCOST),
                instructions::SED_IMP => self.set_decimal_mode(instructions::SED_IMP_CCOST),
                instructions::SEI_IMP => self.set_interrupt_disable(instructions::SEI_IMP_CCOST),
                instructions::NOP_IMP => self.nop(instructions::NOP_IMP_CCOST),
                _ => panic!("Unhandled instruction {instr}"),
            }
        }
    }

    ////////// HELPER FUNCTIONS //////////

    fn increment_program_counter(&mut self, increment: Word) {
        self.pc += increment;
    }

    fn use_cycles(&mut self, c: u32) {
        assert!(self.cycles > 0);
        self.cycles -= c;
    }

    fn fetch_byte(&mut self, mem: &Memory) -> Byte {
        let b: Byte = mem.get_byte(self.pc as usize);
        self.increment_program_counter(1);
        self.use_cycles(1);
        b
    }

    fn fetch_word(&mut self, mem: &mut Memory) -> Word {
        let w1 = u16::from(mem.get_byte(self.pc as usize));
        let w2 = u16::from(mem.get_byte((self.pc + 1) as usize));
        self.increment_program_counter(2);
        self.use_cycles(2);
        (w1 << 8) | w2
    }

    ////////// SET STATUS FUNCTIONS //////////

    fn lda_set_status(&mut self) {
        if self.acc == 0 {
            Mos6502Flags::Z.set(&mut self.status);
        } else {
            Mos6502Flags::Z.unset(&mut self.status);
        }
        if self.acc & 0b1000_0000 == 0 {
            Mos6502Flags::N.unset(&mut self.status);
        } else {
            Mos6502Flags::N.set(&mut self.status);
        }
        todo!("update cycles");
    }

    ////////// CPU FUNCTIONS //////////

    fn clear_carry_flag(&mut self, cycle_cost: u32) {
        Mos6502Flags::C.unset(&mut self.status);
        self.use_cycles(cycle_cost);
    }

    fn clear_decimal_mode(&mut self, cycle_cost: u32) {
        Mos6502Flags::D.unset(&mut self.status);
        self.use_cycles(cycle_cost);
    }

    fn clear_interrupt_disable(&mut self, cycle_cost: u32) {
        Mos6502Flags::I.unset(&mut self.status);
        self.use_cycles(cycle_cost);
    }

    fn clear_overflow_flag(&mut self, cycle_cost: u32) {
        Mos6502Flags::V.unset(&mut self.status);
        self.use_cycles(cycle_cost);
    }

    fn set_carry_flag(&mut self, cycle_cost: u32) {
        Mos6502Flags::C.set(&mut self.status);
        self.use_cycles(cycle_cost);
    }

    fn set_decimal_mode(&mut self, cycle_cost: u32) {
        Mos6502Flags::D.set(&mut self.status);
        self.use_cycles(cycle_cost);
    }

    fn set_interrupt_disable(&mut self, cycle_cost: u32) {
        Mos6502Flags::I.set(&mut self.status);
        self.use_cycles(cycle_cost);
    }

    fn nop(&mut self, cycle_cost: u32) {
        self.use_cycles(cycle_cost);
    }
}
