#![allow(dead_code)]

use crate::memory::Memory;

type Byte = u8;
type Word = u16;

#[derive(PartialEq)]
pub enum Mos6502Flags {
    C = 1 << 0, // Carry
    Z = 1 << 1, // Zero
    I = 1 << 2, // Interrupts disable
    D = 1 << 3, // Decimal mode (unused)
    B = 1 << 4, // Break
    U = 1 << 5, // Unused
    V = 1 << 6, // Overflow
    N = 1 << 7, // Negative
}

impl Mos6502Flags {
    pub fn set(self, status: &mut Byte) {
        *status |= self as Byte;
    }

    fn clear(self, status: &mut Byte) {
        *status &= !(self as Byte);
    }

    fn get(self, status: Byte) -> bool {
        status & self as Byte != 0
    }
}

pub struct Mos6502 {
    a: Byte,
    x: Byte,
    y: Byte,
    status: Byte,
    sp: Byte,
    pc: Word,
    mem: Memory,
    cycles: u32,
}

impl std::fmt::Display for Mos6502 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "a: {:x}\nx: {:x}\ny: {:x}\nstatus: {:x}\nsp: {:x}\npc: {:x}\ncycles: {:x}",
               self.a, self.x, self.y, self.status, self.sp, self.pc, self.cycles)
    }
}

const LOOKUP_TBL_SIZE: usize = 0x100;

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
    Some(Mos6502::jsr_abs),
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
    Some(Mos6502::adc_absy),
    None,
    None,
    None,
    Some(Mos6502::adc_absx),
    Some(Mos6502::ror_absx),
    None,
    None,
    Some(Mos6502::sta_zpx_ind),
    None,
    None,
    Some(Mos6502::sty_zp),
    Some(Mos6502::sta_zp),
    Some(Mos6502::stx_zp),
    None,
    Some(Mos6502::dey_imp),
    None,
    Some(Mos6502::txa_imp),
    None,
    Some(Mos6502::sty_abs),
    Some(Mos6502::sta_abs),
    Some(Mos6502::stx_abs),
    None,
    Some(Mos6502::bcc_rel),
    Some(Mos6502::sta_zpy_ind),
    None,
    None,
    Some(Mos6502::sty_zpx),
    Some(Mos6502::sta_zpx),
    Some(Mos6502::stx_zpy),
    None,
    Some(Mos6502::tya_imp),
    Some(Mos6502::sta_absy),
    Some(Mos6502::txs_imp),
    None,
    None,
    Some(Mos6502::sta_absx),
    None,
    None,
    Some(Mos6502::ldy_imm),
    Some(Mos6502::lda_zpx_ind),
    Some(Mos6502::ldx_imm),
    None,
    Some(Mos6502::ldy_zp),
    Some(Mos6502::lda_zp),
    Some(Mos6502::ldx_zp),
    None,
    Some(Mos6502::tay_imp),
    Some(Mos6502::lda_imm),
    Some(Mos6502::tax_imp),
    None,
    Some(Mos6502::ldy_abs),
    Some(Mos6502::lda_abs),
    Some(Mos6502::ldx_abs),
    None,
    Some(Mos6502::bcs_rel),
    Some(Mos6502::lda_zpy_ind),
    None,
    None,
    Some(Mos6502::ldy_zpx),
    Some(Mos6502::lda_zpx),
    Some(Mos6502::ldx_zpy),
    None,
    Some(Mos6502::clv_imp),
    Some(Mos6502::lda_absy),
    Some(Mos6502::tsx_imp),
    None,
    Some(Mos6502::ldy_absx),
    Some(Mos6502::lda_absx),
    Some(Mos6502::ldx_absy),
    None,
    Some(Mos6502::cpy_imm),
    Some(Mos6502::cmp_zpx_ind),
    None,
    None,
    Some(Mos6502::cpy_zp),
    Some(Mos6502::cmp_zp),
    Some(Mos6502::dec_zp),
    None,
    Some(Mos6502::iny_imp),
    Some(Mos6502::cmp_imm),
    Some(Mos6502::dex_imp),
    None,
    Some(Mos6502::cpy_abs),
    Some(Mos6502::cmp_abs),
    Some(Mos6502::dec_abs),
    None,
    Some(Mos6502::bne_rel),
    Some(Mos6502::cmp_zpy_ind),
    None,
    None,
    None,
    Some(Mos6502::cmp_zpx),
    Some(Mos6502::dec_zpx),
    None,
    Some(Mos6502::cld_imp),
    Some(Mos6502::cmp_absy),
    None,
    None,
    None,
    Some(Mos6502::cmp_absx),
    Some(Mos6502::dec_absx),
    None,
    Some(Mos6502::cpx_imm),
    Some(Mos6502::sbc_zpx_ind),
    None,
    None,
    Some(Mos6502::cpx_zp),
    Some(Mos6502::sbc_zp),
    Some(Mos6502::inc_zp),
    None,
    Some(Mos6502::inx_imp),
    Some(Mos6502::sbc_imm),
    Some(Mos6502::nop_imp),
    None,
    Some(Mos6502::cpx_abs),
    Some(Mos6502::sbc_abs),
    Some(Mos6502::inc_abs),
    None,
    Some(Mos6502::beq_rel),
    Some(Mos6502::sbc_zpy_ind),
    None,
    None,
    None,
    Some(Mos6502::sbc_zpx),
    Some(Mos6502::inc_zpx),
    None,
    Some(Mos6502::sed_imp),
    Some(Mos6502::sbc_absy),
    None,
    None,
    None,
    Some(Mos6502::sbc_absx),
    Some(Mos6502::inc_absx),
    None,
];

impl Mos6502 {

    ////////// PUBLIC FUNCTIONS //////////

    pub fn new(mem: Memory) -> Self {
        Self {
            a: 0x00, x: 0x00, y: 0x00,
            status: 0x00, cycles: 0,
            sp: 0xFF,
            pc: 0x0000,
            mem,
        }
    }

    pub fn reset(&mut self) {
        (self.a, self.x, self.y) = (0x00, 0x00, 0x00);

        self.cycles = 0;

        // let lo: Byte = self.mem.get_byte(0xFFFC + 0);
        // let hi: Byte = self.mem.get_byte(0xFFFC + 1);
        // self.pc = ((hi as Word) << 8u8) | lo as Word;
        // self.status = 0x00 | Mos6502Flags::U as Byte;

        self.pc = 0xFFFC;
        self.sp = 0xFF;

        self.status = 0x00;

        // TODO: increment cycles by 8.
    }

    pub fn exe(&mut self, cycle_limit: Option<u32>) {
        while match cycle_limit { Some(lim) => self.cycles < lim, _ => true } {
            let opcode: Byte = self.fetch_byte();
            if let Some(instruction) = LOOKUP[opcode as usize] {
                instruction(self);
            } else {
                println!("Illegal opcode: {opcode}");
            }
        }
    }

    pub fn zero_flag(&self) -> bool {
        Mos6502Flags::Z.get(self.status)
    }

    pub fn negative_flag(&self) -> bool {
        Mos6502Flags::N.get(self.status)
    }

    pub fn carry_flag(&self) -> bool {
        Mos6502Flags::C.get(self.status)
    }

    pub fn interrupts_disable_flag(&self) -> bool {
        Mos6502Flags::I.get(self.status)
    }

    pub fn decimal_mode_flag(&self) -> bool {
        Mos6502Flags::D.get(self.status)
    }

    pub fn break_flag(&self) -> bool {
        Mos6502Flags::B.get(self.status)
    }

    pub fn unused_flag(&self) -> bool {
        Mos6502Flags::U.get(self.status)
    }

    pub fn overflow_flag(&self) -> bool {
        Mos6502Flags::V.get(self.status)
    }

    pub fn set_xreg(&mut self, data: Byte) {
        self.x = data;
    }

    pub fn get_accumulator(&self) -> Byte {
        self.a
    }

    pub fn get_yreg(&self) -> Byte {
        self.y
    }

    pub fn get_xreg(&self) -> Byte {
        self.x
    }

    pub fn get_status(&self) -> Byte {
        self.status
    }

    pub fn get_cycles(&self) -> u32 {
        self.cycles
    }

    ////////// HELPER FUNCTIONS //////////

    fn cycle(&mut self) {
        self.cycles += 1;
    }

    fn push_byte(&mut self, addr: usize, data: Byte) {
        *self.mem.at(addr) = data;
        self.sp -= 1;
        self.cycle();
    }

    fn push_word(&mut self, addr: usize, data: Word) {
        self.mem.write_word(addr, data);
        self.cycle();
        self.cycle();
        self.sp -= 2;
    }

    fn pop(&mut self) {
        self.sp += 1;
        self.cycle();
    }

    fn pc_assign_wcycle(&mut self, data: Word) {
        self.pc = data;
        self.cycle();
    }

    fn acc_assign_wcycle(&mut self, data: Byte) {
        self.a = data;
        self.cycle();
    }

    fn xreg_assign_wcycle(&mut self, data: Byte) {
        self.x = data;
        self.cycle();
    }

    fn yreg_assign_wcycle(&mut self, data: Byte) {
        self.y = data;
        self.cycle();
    }

    fn add_offset_wcycle(&mut self, target: &mut Byte, offset: Byte) {
        let sum = *target as u16 + offset as u16;
        let wrapped_sum = sum % 256;
        *target = wrapped_sum as Byte;
        self.cycle();
    }

    fn program_counter(&mut self) {
        self.pc += 1;
    }

    fn read_byte_at_addr(&mut self, addr: Byte) -> Byte {
        // NOTE: This function is needed to retrieve a byte
        // and not increment the program counter.
        self.cycle();
        self.mem.get_byte(addr as usize)
    }

    fn fetch_byte(&mut self) -> Byte {
        let b: Byte = self.mem.get_byte(self.pc as usize);
        self.program_counter();
        self.cycle();
        b
    }

    fn fetch_word(&mut self) -> Word {
        let w1 = u16::from(self.mem.get_byte(self.pc as usize));
        self.program_counter();
        self.cycle();

        let w2 = u16::from(self.mem.get_byte(self.pc as usize));
        self.program_counter();
        self.cycle();

        (w1 << 8) | w2
    }

    ////////// SET STATUS FUNCTIONS //////////

    fn lda_set_status(&mut self) {
        if self.a == 0 {
            Mos6502Flags::Z.set(&mut self.status);
        } else {
            Mos6502Flags::Z.clear(&mut self.status);
        }
        if self.a & (1u8 << 7) == 0 {
            Mos6502Flags::N.clear(&mut self.status);
        } else {
            Mos6502Flags::N.set(&mut self.status);
        }
    }

    ////////// CPU INSTRUCTION FUNCTIONS //////////

    fn asl_zp(&mut self) {
        todo!()
    }

    fn asl_acc(&mut self) {
        todo!()
    }

    fn asl_abs(&mut self) {
        todo!()
    }

    fn asl_zpx(&mut self) {
        todo!()
    }

    fn asl_absx(&mut self) {
        todo!()
    }

    fn and_zpx_ind(&mut self) {
        todo!()
    }

    fn and_zp(&mut self) {
        todo!()
    }

    fn and_abs(&mut self) {
        todo!()
    }

    fn and_imm(&mut self) {
        todo!()
    }

    fn and_zpy_ind(&mut self) {
        todo!()
    }

    fn and_zpx(&mut self) {
        todo!()
    }

    fn and_absy(&mut self) {
        todo!()
    }

    fn and_absx(&mut self) {
        todo!()
    }

    fn ora_zpx_ind(&mut self) {
        todo!()
    }

    fn ora_zp(&mut self) {
        todo!()
    }

    fn ora_imm(&mut self) {
        todo!()
    }

    fn ora_abs(&mut self) {
        todo!()
    }

    fn ora_zpy_ind(&mut self) {
        todo!()
    }

    fn ora_zpx(&mut self) {
        todo!()
    }

    fn ora_absy(&mut self) {
        todo!()
    }

    fn ora_absx(&mut self) {
        todo!()
    }

    fn brk_imp(&mut self) {
        todo!()
    }

    fn php_imp(&mut self) {
        todo!()
    }

    fn bpl_rel(&mut self) {
        todo!()
    }

    fn clc_imp(&mut self) {
        Mos6502Flags::C.clear(&mut self.status);
        todo!("cycles");
    }

    fn jsr_abs(&mut self) {
        let addr = self.fetch_word();
        self.push_word(self.pc as usize, self.pc - 1);
        self.pc_assign_wcycle(addr);
    }

    fn bit_zp(&mut self) {
        todo!()
    }

    fn rol_zp(&mut self) {
        todo!()
    }

    fn plp_imp(&mut self) {
        todo!()
    }

    fn rol_acc(&mut self) {
        todo!()
    }

    fn bit_abs(&mut self) {
        todo!()
    }

    fn rol_abs(&mut self) {
        todo!()
    }

    fn bmi_rel(&mut self) {
        todo!()
    }

    fn rol_zpx(&mut self) {
        todo!()
    }

    fn sec_imp(&mut self) {
        Mos6502Flags::C.set(&mut self.status);
        todo!("cycles");
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
        todo!("cycles");
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
        todo!("cycles");
    }

    fn adc_absy(&mut self) {
        todo!()
    }

    fn adc_absx(&mut self) {
        todo!()
    }

    fn ror_absx(&mut self) {
        todo!()
    }

    fn sta_zpx_ind(&mut self) {
        todo!()
    }

    fn sty_zp(&mut self) {
        todo!()
    }

    fn sta_zp(&mut self) {
        todo!()
    }

    fn stx_zp(&mut self) {
        todo!()
    }

    fn dey_imp(&mut self) {
        todo!()
    }

    fn txa_imp(&mut self) {
        todo!()
    }

    fn sty_abs(&mut self) {
        todo!()
    }

    fn sta_abs(&mut self) {
        todo!()
    }

    fn stx_abs(&mut self) {
        todo!()
    }

    fn bcc_rel(&mut self) {
        todo!()
    }

    fn sta_zpy_ind(&mut self) {
        todo!()
    }

    fn sty_zpx(&mut self) {
        todo!()
    }

    fn sta_zpx(&mut self) {
        todo!()
    }

    fn stx_zpy(&mut self) {
        todo!()
    }

    fn tya_imp(&mut self) {
        todo!()
    }

    fn sta_absy(&mut self) {
        todo!()
    }

    fn txs_imp(&mut self) {
        todo!()
    }

    fn sta_absx(&mut self) {
        todo!()
    }

    fn ldy_imm(&mut self) {
        todo!()
    }

    fn ldx_imm(&mut self) {
        todo!()
    }

    fn ldy_zp(&mut self) {
        todo!()
    }

    fn lda_imm(&mut self) {
        self.a = self.fetch_byte();
        self.lda_set_status();
    }

    fn lda_zpx_ind(&mut self) {
        todo!()
    }

    fn lda_zp(&mut self) {
        let zpaddr: Byte = self.fetch_byte();
        self.a = self.read_byte_at_addr(zpaddr);
        self.lda_set_status();
    }

    fn lda_zpx(&mut self) {
        let mut zpaddr: Byte = self.fetch_byte();
        self.add_offset_wcycle(&mut zpaddr, self.x);
        self.a = self.read_byte_at_addr(zpaddr);
        self.lda_set_status();
    }

    fn ldx_zp(&mut self) {
        todo!()
    }

    fn tay_imp(&mut self) {
        todo!()
    }

    fn tax_imp(&mut self) {
        todo!()
    }

    fn ldy_abs(&mut self) {
        todo!()
    }

    fn lda_abs(&mut self) {
        todo!()
    }

    fn ldx_abs(&mut self) {
        todo!()
    }

    fn bcs_rel(&mut self) {
        todo!()
    }

    fn lda_zpy_ind(&mut self) {
        todo!()
    }

    fn ldy_zpx(&mut self) {
        todo!()
    }

    fn ldx_zpy(&mut self) {
        todo!()
    }

    fn clv_imp(&mut self) {
        Mos6502Flags::V.clear(&mut self.status);
    }

    fn lda_absy(&mut self) {
        todo!()
    }

    fn tsx_imp(&mut self) {
        todo!()
    }

    fn ldy_absx(&mut self) {
        todo!()
    }

    fn lda_absx(&mut self) {
        todo!()
    }

    fn ldx_absy(&mut self) {
        todo!()
    }

    fn cpy_imm(&mut self) {
        todo!()
    }

    fn cmp_zpx_ind(&mut self) {
        todo!()
    }

    fn cpy_zp(&mut self) {
        todo!()
    }

    fn cmp_zp(&mut self) {
        todo!()
    }

    fn dec_zp(&mut self) {
        todo!()
    }

    fn iny_imp(&mut self) {
        todo!()
    }

    fn cmp_imm(&mut self) {
        todo!()
    }

    fn dex_imp(&mut self) {
        todo!()
    }

    fn cpy_abs(&mut self) {
        todo!()
    }

    fn cmp_abs(&mut self) {
        todo!()
    }

    fn dec_abs(&mut self) {
        todo!()
    }

    fn bne_rel(&mut self) {
        todo!()
    }

    fn cmp_zpy_ind(&mut self) {
        todo!()
    }

    fn cmp_zpx(&mut self) {
        todo!()
    }

    fn dec_zpx(&mut self) {
        todo!()
    }

    fn cld_imp(&mut self) {
        Mos6502Flags::D.clear(&mut self.status);
    }

    fn cmp_absy(&mut self) {
        todo!()
    }

    fn cmp_absx(&mut self) {
        todo!()
    }

    fn dec_absx(&mut self) {
        todo!()
    }

    fn cpx_imm(&mut self) {
        todo!()
    }

    fn sbc_zpx_ind(&mut self) {
        todo!()
    }

    fn cpx_zp(&mut self) {
        todo!()
    }

    fn sbc_zp(&mut self) {
        todo!()
    }

    fn inc_zp(&mut self) {
        todo!()
    }

    fn inx_imp(&mut self) {
        todo!()
    }

    fn sbc_imm(&mut self) {
        todo!()
    }

    fn nop_imp(&mut self) {
        self.cycle();
    }

    fn cpx_abs(&mut self) {
        todo!()
    }

    fn sbc_abs(&mut self) {
        todo!()
    }

    fn inc_abs(&mut self) {
        todo!()
    }

    fn beq_rel(&mut self) {
        todo!()
    }

    fn sbc_zpy_ind(&mut self) {
        todo!()
    }

    fn sbc_zpx(&mut self) {
        todo!()
    }

    fn inc_zpx(&mut self) {
        todo!()
    }

    fn sed_imp(&mut self) {
        Mos6502Flags::D.set(&mut self.status);
        self.cycle();
    }

    fn sbc_absy(&mut self) {
        todo!()
    }

    fn sbc_absx(&mut self) {
        todo!()
    }

    fn inc_absx(&mut self) {
        todo!()
    }
}
