use crate::memory::Memory;
use crate::instructions;

type Byte = u8;
type Word = u16;

#[allow(dead_code)]
enum Mos6502Flags {
    C = 0, // Carry
    Z = 1, // Zero
    I = 2, // Disable interrupts
    D = 3, // Decimal mode (unused)
    B = 4, // Break
    U = 5, // Unused
    V = 6, // Overflow
    N = 7, // Negative
}

impl Mos6502Flags {
    pub fn set(self, status: &mut Byte) {
        *status |= 1u8 << (self as Byte);
    }

    pub fn clear(self, status: &mut Byte) {
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
        write!(f, "acc: {:x}\nx: {:x}\ny: {:x}\nstatus: {:x}\nstackptr: {:x}\npc: {:x}\ncycles: {:x}",
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

    pub fn reset(&mut self, cycles: Option<u32>, mem: &mut Memory) {
        (self.acc, self.x, self.y, self.status) = (0x00, 0x00, 0x00, 0x00);
        self.cycles = match cycles { Some(c) => c, _ => 0 };
        self.stackptr = STACKPTR_BEGIN;
        self.pc = PROGRAM_COUNTER_BEGIN;
        mem.clear();
    }

    pub fn exe(&mut self, mem: &mut Memory) {
        while self.cycles > 0 {
            let instr: Byte = self.fetch_byte(mem);
            match instr {
                instructions::LDA_IMM => self.lda_imm(mem, instructions::LDA_IMM_COST),
                instructions::LDA_ZP => self.lda_zp(mem, instructions::LDA_ZP_CCOST),
                instructions::LDA_ZPX => self.lda_zpx(mem, instructions::LDA_ZPX_CCOST),
                instructions::CLC_IMP => self.clc_imp(instructions::CLC_IMP_CCOST),
                instructions::CLD_IMP => self.cld_imp(instructions::CLD_IMP_CCOST),
                instructions::CLI_IMP => self.cli_imp(instructions::CLI_IMP_CCOST),
                instructions::CLV_IMP => self.clv_imp(instructions::CLV_IMP_CCOST),
                instructions::SEC_IMP => self.sec_imp(instructions::SEC_IMP_CCOST),
                instructions::SED_IMP => self.sed_imp(instructions::SED_IMP_CCOST),
                instructions::SEI_IMP => self.sei_imp(instructions::SEI_IMP_CCOST),
                instructions::NOP_IMP => self.nop_imp(instructions::NOP_IMP_CCOST),
                _ => println!("Unhandled instruction {instr}"),
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

    fn read_byte(&mut self, addr: Byte, mem: &Memory) -> Byte {
        // NOTE: This function is needed to retrieve a byte
        // and not increment the program counter.
        // NOTE: it is not needed to use up a cycle here,
        // as the function that simulates the instruction
        // will cover it.
        mem.get_byte(addr as usize)
    }

    fn fetch_byte(&mut self, mem: &Memory) -> Byte {
        // NOTE: it is not needed to use up a cycle here,
        // as the function that simulates the instruction
        // will cover it.
        let b: Byte = mem.get_byte(self.pc as usize);
        self.increment_program_counter(1);
        b
    }

    fn fetch_word(&mut self, mem: &mut Memory) -> Word {
        // NOTE: it is not needed to use up a cycle here,
        // as the function that simulates the instruction
        // will cover it.
        let w1 = u16::from(mem.get_byte(self.pc as usize));
        let w2 = u16::from(mem.get_byte((self.pc + 1) as usize));
        self.increment_program_counter(2);
        (w1 << 8) | w2
    }

    ////////// SET STATUS FUNCTIONS //////////

    fn lda_set_status(&mut self) {
        // NOTE: it is not needed to use up a cycle here,
        // as the function that simulates the instruction
        // will cover it.
        if self.acc == 0 {
            Mos6502Flags::Z.set(&mut self.status);
        } else {
            Mos6502Flags::Z.clear(&mut self.status);
        }
        if self.acc & 0b1000_0000 == 0 {
            Mos6502Flags::N.clear(&mut self.status);
        } else {
            Mos6502Flags::N.set(&mut self.status);
        }
    }

    ////////// CPU FUNCTIONS //////////

    fn lda_imm(&mut self, mem: &Memory, cycle_cost: u32) {
        self.acc = self.fetch_byte(mem);
        self.lda_set_status();
        self.use_cycles(cycle_cost);
    }

    fn lda_zp(&mut self, mem: &Memory, cycle_cost: u32) {
        let zpaddr = self.fetch_byte(mem);
        self.acc = self.read_byte(zpaddr, mem);
        self.lda_set_status();
        self.use_cycles(cycle_cost);
    }

    fn lda_zpx(&mut self, mem: &Memory, cycle_cost: u32) {
        let zpaddr = self.fetch_byte(mem) + self.x;
        self.acc = self.read_byte(zpaddr, mem);
        self.lda_set_status();
        self.use_cycles(cycle_cost);
    }

    fn clc_imp(&mut self, cycle_cost: u32) {
        Mos6502Flags::C.clear(&mut self.status);
        self.use_cycles(cycle_cost);
    }

    fn cld_imp(&mut self, cycle_cost: u32) {
        Mos6502Flags::D.clear(&mut self.status);
        self.use_cycles(cycle_cost);
    }

    fn cli_imp(&mut self, cycle_cost: u32) {
        Mos6502Flags::I.clear(&mut self.status);
        self.use_cycles(cycle_cost);
    }

    fn clv_imp(&mut self, cycle_cost: u32) {
        Mos6502Flags::V.clear(&mut self.status);
        self.use_cycles(cycle_cost);
    }

    fn sec_imp(&mut self, cycle_cost: u32) {
        Mos6502Flags::C.set(&mut self.status);
        self.use_cycles(cycle_cost);
    }

    fn sed_imp(&mut self, cycle_cost: u32) {
        Mos6502Flags::D.set(&mut self.status);
        self.use_cycles(cycle_cost);
    }

    fn sei_imp(&mut self, cycle_cost: u32) {
        Mos6502Flags::I.set(&mut self.status);
        self.use_cycles(cycle_cost);
    }

    fn nop_imp(&mut self, cycle_cost: u32) {
        self.use_cycles(cycle_cost);
    }
}
