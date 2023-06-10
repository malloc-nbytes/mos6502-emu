#![allow(dead_code)]

use crate::memory::Memory;
use crate::instructions;

type Byte = u8;
type Word = u16;

enum Mos6502Flags {
    C = 1 << 0, // Carry
    Z = 1 << 1, // Zero
    I = 1 << 2, // Disable interrupts
    D = 1 << 3, // Decimal mode (unused)
    B = 1 << 4, // Break
    U = 1 << 5, // Unused
    V = 1 << 6, // Overflow
    N = 1 << 7, // Negative
}

impl Mos6502Flags {
    pub fn set(self, status: &mut Byte) {
        *status |= self as Byte
    }

    pub fn clear(self, status: &mut Byte) {
        *status &= !(self as Byte)
    }
}

const STACKPTR_BEGIN: Byte = 0xFF;
const PROGRAM_COUNTER_BEGIN: Word = 0xFFFC;

pub struct Mos6502 {
    acc: Byte,
    x: Byte,
    y: Byte,
    status: Byte,
    stackptr: Byte,
    cycles: u32,
    pc: Word,
    mem: Memory,
}

impl std::fmt::Display for Mos6502 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "acc: {:x}\nx: {:x}\ny: {:x}\nstatus: {:x}\nstackptr: {:x}\npc: {:x}\ncycles: {:x}",
               self.acc, self.x, self.y, self.status, self.stackptr, self.pc, self.cycles)
    }
}

const LOOKUP_TBL_SIZE: usize = 121;

const LOOKUP: [Option<fn(&mut Mos6502)>; LOOKUP_TBL_SIZE] = [
    Some(Mos6502::brk_imp),
    Some(Mos6502::ora_zpx_ind),
    None,
    None,
    None,
    Some(Mos6502::ora_zp),
    Some(Mos6502::asl_zp),
    None,
    Some(Mos6502::php_imp),
    Some(Mos6502::ora_imm),
    Some(Mos6502::asl_acc),
    None,
    None,
    Some(Mos6502::ora_abs),
    Some(Mos6502::asl_abs),
    None,
    Some(Mos6502::bpl_rel),
    Some(Mos6502::ora_zpy_ind),
    None,
    None,
    None,
    Some(Mos6502::ora_zpx),
    Some(Mos6502::asl_zpx),
    None,
    Some(Mos6502::clc_imp),
    Some(Mos6502::ora_absy),
    None,
    None,
    None,
    Some(Mos6502::ora_absx),
    Some(Mos6502::asl_absx),
    None,
    Some(Mos6502::jmp_sr_abs),
    Some(Mos6502::and_zpx_ind),
    None,
    None,
    Some(Mos6502::bit_zp),
    Some(Mos6502::and_zp),
    Some(Mos6502::rol_zp),
    None,
    Some(Mos6502::plp_imp),
    Some(Mos6502::and_imm),
    Some(Mos6502::rol_acc),
    None,
    Some(Mos6502::bit_abs),
    Some(Mos6502::and_abs),
    Some(Mos6502::rol_abs),
    None,
    Some(Mos6502::bmi_rel),
    Some(Mos6502::and_zpy_ind),
    None,
    None,
    None,
    Some(Mos6502::and_zpx),
    Some(Mos6502::rol_zpx),
    None,
    Some(Mos6502::sec_imp),
    Some(Mos6502::and_absy),
    None,
    None,
    None,
    Some(Mos6502::and_absx),
    Some(Mos6502::rol_absx),
    None,
    Some(Mos6502::rti_imp),
    Some(Mos6502::eor_zpx_ind),
    None,
    None,
    None,
    Some(Mos6502::eor_zp),
    Some(Mos6502::lsr_zp),
    None,
    Some(Mos6502::pha_imp),
    Some(Mos6502::eor_imm),
    Some(Mos6502::lsr_acc),
    None,
    Some(Mos6502::jmp_abs),
    Some(Mos6502::eor_abs),
    Some(Mos6502::lsr_abs),
    None,
    Some(Mos6502::bvc_rel),
    Some(Mos6502::eor_zpy_ind),
    None,
    None,
    None,
    Some(Mos6502::eor_zpx),
    Some(Mos6502::lsr_zpx),
    None,
    Some(Mos6502::cli_imp),
    Some(Mos6502::eor_absy),
    None,
    None,
    None,
    Some(Mos6502::eor_absx),
    Some(Mos6502::lsr_absx),
    None,
    Some(Mos6502::rts_imp),
    Some(Mos6502::adc_zpx_ind),
    None,
    None,
    None,
    Some(Mos6502::adc_zp),
    Some(Mos6502::ror_zp),
    None,
    Some(Mos6502::pla_imp),
    Some(Mos6502::adc_imm),
    Some(Mos6502::ror_acc),
    None,
    Some(Mos6502::jmp_abs_ind),
    Some(Mos6502::adc_abs),
    Some(Mos6502::ror_abs),
    None,
    Some(Mos6502::bvs_rel),
    Some(Mos6502::adc_zpy_ind),
    None,
    None,
    None,
    Some(Mos6502::adc_zpx),
    Some(Mos6502::ror_zpx),
    None,
    Some(Mos6502::sei_imp),
];

impl Mos6502 {
    pub fn new(cycles: Option<u32>, mem: Memory) -> Self {
        let c = match cycles { Some(k) => k, _ => 0 };
        Self {
            acc: 0x00, x: 0x00, y: 0x00,
            status: 0x00, cycles: c,
            stackptr: STACKPTR_BEGIN,
            pc: PROGRAM_COUNTER_BEGIN,
            mem,
        }
    }

    pub fn reset(&mut self, cycles: Option<u32>) {
        (self.acc, self.x, self.y, self.status) = (0x00, 0x00, 0x00, 0x00);
        self.cycles = match cycles { Some(c) => c, _ => 0 };
        self.stackptr = STACKPTR_BEGIN;
        self.pc = PROGRAM_COUNTER_BEGIN;
        self.mem.clear();
    }

    pub fn exe(&mut self) {
        while self.cycles > 0 {
            let _instr: Byte = self.fetch_byte();
            todo!()
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

    fn read_byte(&mut self, addr: Byte) -> Byte {
        // NOTE: This function is needed to retrieve a byte
        // and not increment the program counter.
        // NOTE: it is not needed to use up a cycle here,
        // as the function that simulates the instruction
        // will cover it.
        self.mem.get_byte(addr as usize)
    }

    fn fetch_byte(&mut self) -> Byte {
        // NOTE: it is not needed to use up a cycle here,
        // as the function that simulates the instruction
        // will cover it.
        let b: Byte = self.mem.get_byte(self.pc as usize);
        self.increment_program_counter(1);
        b
    }

    fn fetch_word(&mut self) -> Word {
        // NOTE: it is not needed to use up a cycle here,
        // as the function that simulates the instruction
        // will cover it.
        let w1 = u16::from(self.mem.get_byte(self.pc as usize));
        let w2 = u16::from(self.mem.get_byte((self.pc + 1) as usize));
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
        if self.acc & (1u8 << 7) == 0 {
            Mos6502Flags::N.clear(&mut self.status);
        } else {
            Mos6502Flags::N.set(&mut self.status);
        }
    }

    ////////// CPU INSTRUCTION FUNCTIONS //////////

    fn brk_imp(&mut self) {
        todo!()
    }

    fn ora_zpx_ind(&mut self) {
        todo!()
    }

    fn ora_zp(&mut self) {
        todo!()
    }

    fn asl_zp(&mut self) {
        todo!()
    }

    fn php_imp(&mut self) {
        todo!()
    }

    fn ora_imm(&mut self) {
        todo!()
    }

    fn asl_acc(&mut self) {
        todo!()
    }

    fn ora_abs(&mut self) {
        todo!()
    }

    fn asl_abs(&mut self) {
        todo!()
    }

    fn bpl_rel(&mut self) {
        todo!()
    }

    fn ora_zpy_ind(&mut self) {
        todo!()
    }

    fn ora_zpx(&mut self) {
        todo!()
    }

    fn asl_zpx(&mut self) {
        todo!()
    }

    fn clc_imp(&mut self) {
        Mos6502Flags::C.clear(&mut self.status);
        self.use_cycles(instructions::CLC_IMP_CCOST);
    }

    fn ora_absy(&mut self) {
        todo!()
    }

    fn ora_absx(&mut self) {
        todo!()
    }

    fn asl_absx(&mut self) {
        todo!()
    }

    fn jmp_sr_abs(&mut self) {
        todo!()
    }

    fn and_zpx_ind(&mut self) {
        todo!()
    }

    fn bit_zp(&mut self) {
        todo!()
    }

    fn and_zp(&mut self) {
        todo!()
    }

    fn rol_zp(&mut self) {
        todo!()
    }

    fn plp_imp(&mut self) {
        todo!()
    }

    fn and_imm(&mut self) {
        todo!()
    }

    fn rol_acc(&mut self) {
        todo!()
    }

    fn bit_abs(&mut self) {
        todo!()
    }

    fn and_abs(&mut self) {
        todo!()
    }

    fn rol_abs(&mut self) {
        todo!()
    }

    fn bmi_rel(&mut self) {
        todo!()
    }

    fn and_zpy_ind(&mut self) {
        todo!()
    }

    fn and_zpx(&mut self) {
        todo!()
    }

    fn rol_zpx(&mut self) {
        todo!()
    }

    fn sec_imp(&mut self) {
        Mos6502Flags::C.set(&mut self.status);
        self.use_cycles(instructions::SEC_IMP_CCOST);
    }

    fn and_absy(&mut self) {
        todo!()
    }

    fn and_absx(&mut self) {
        todo!()
    }

    fn rol_absx(&mut self) {
        todo!()
    }

    fn rti_imp(&mut self) {
        todo!()
    }

    fn eor_zpx_ind(&mut self) {
        todo!()
    }

    fn eor_zp(&mut self) {
        todo!()
    }

    fn lsr_zp(&mut self) {
        todo!()
    }

    fn pha_imp(&mut self) {
        todo!()
    }

    fn eor_imm(&mut self) {
        todo!()
    }

    fn lsr_acc(&mut self) {
        todo!()
    }

    fn jmp_abs(&mut self) {
        todo!()
    }

    fn eor_abs(&mut self) {
        todo!()
    }

    fn lsr_abs(&mut self) {
        todo!()
    }

    fn bvc_rel(&mut self) {
        todo!()
    }

    fn eor_zpy_ind(&mut self) {
        todo!()
    }

    fn eor_zpx(&mut self) {
        todo!()
    }

    fn lsr_zpx(&mut self) {
        todo!()
    }

    fn cli_imp(&mut self) {
        Mos6502Flags::I.clear(&mut self.status);
        self.use_cycles(instructions::CLI_IMP_CCOST);
    }

    fn eor_absy(&mut self) {
        todo!()
    }

    fn eor_absx(&mut self) {
        todo!()
    }

    fn lsr_absx(&mut self) {
        todo!()
    }

    fn rts_imp(&mut self) {
        todo!()
    }

    fn adc_zpx_ind(&mut self) {
        todo!()
    }

    fn adc_zpx_in(&mut self) {
        todo!()
    }

    fn adc_zp(&mut self) {
        todo!()
    }

    fn ror_zp(&mut self) {
        todo!()
    }

    fn pla_imp(&mut self) {
        todo!()
    }

    fn adc_imm(&mut self) {
        todo!()
    }

    fn ror_acc(&mut self) {
        todo!()
    }

    fn jmp_abs_ind(&mut self) {
        todo!()
    }

    fn adc_abs(&mut self) {
        todo!()
    }

    fn ror_abs(&mut self) {
        todo!()
    }

    fn bvs_rel(&mut self) {
        todo!()
    }

    fn adc_zpy_ind(&mut self) {
        todo!()
    }

    fn adc_zpx(&mut self) {
        todo!()
    }

    fn ror_zpx(&mut self) {
        todo!()
    }

    fn sei_imp(&mut self) {
        Mos6502Flags::I.set(&mut self.status);
        self.use_cycles(instructions::SEI_IMP_CCOST);
    }

    fn lda_imm(&mut self) {
        self.acc = self.fetch_byte();
        self.lda_set_status();
        self.use_cycles(instructions::LDA_IMM_COST);
    }

    fn lda_abs(&mut self) {
        todo!()
    }

    fn lda_zp(&mut self) {
        // TODO: address overflow.
        let zpaddr: Byte = self.fetch_byte();
        self.acc = self.read_byte(zpaddr);
        self.lda_set_status();
        self.use_cycles(instructions::LDA_ZP_CCOST);
    }

    fn lda_zpx(&mut self) {
        // TODO: address overflow.
        let zpaddr: Byte = self.fetch_byte() + self.x;
        self.acc = self.read_byte(zpaddr);
        self.lda_set_status();
        self.use_cycles(instructions::LDA_ZPX_CCOST);
    }

    fn cld_imp(&mut self) {
        Mos6502Flags::D.clear(&mut self.status);
        self.use_cycles(instructions::CLD_IMP_CCOST);
    }

    fn clv_imp(&mut self) {
        Mos6502Flags::V.clear(&mut self.status);
        self.use_cycles(instructions::CLV_IMP_CCOST);
    }

    fn sed_imp(&mut self) {
        Mos6502Flags::D.set(&mut self.status);
        self.use_cycles(instructions::SED_IMP_CCOST);
    }

    fn nop_imp(&mut self) {
        self.use_cycles(instructions::NOP_IMP_CCOST);
    }
}
