#![allow(unused_imports)]

#[cfg(test)]
mod tests {
    use crate::mos6502::{
        Mos6502,
        Mos6502Flags,
    };
    use crate::memory::{
        Memory,
        Word,
        Byte,
    };
    use crate::instructions;
    use crate::tests_utils;

    const PC_START: Word = 0xFFFC;

    #[test]
    fn lda_imm() {
        let val = 0x84;
        tests_utils::ld_into_reg(
            vec![
                (PC_START, instructions::LDA_IMM),
                (PC_START + 1, val),
            ],
            val,
            instructions::LDA_IMM_CCOST,
            tests_utils::Registers::A,
            vec![Mos6502Flags::N],
            None::<fn(&mut Mos6502)>
        );
    }

    #[test]
    fn lda_imm_wzero() {
        let val = 0x00;
        tests_utils::ld_into_reg(
            vec![
                (PC_START, instructions::LDA_IMM),
                (PC_START + 1, val),
            ],
            val,
            instructions::LDA_IMM_CCOST,
            tests_utils::Registers::A,
            vec![Mos6502Flags::Z],
            Some(|cpu: &mut Mos6502| { cpu.set_xreg(0x44) })
        );
    }

    #[test]
    fn lda_zp() {
        let (addr, val) = (0x42, 0x37);
        tests_utils::ld_into_reg(
            vec![
                (PC_START, instructions::LDA_ZP),
                (PC_START + 1, addr),
                (0x0042, val),
            ],
            val,
            instructions::LDA_ZP_CCOST,
            tests_utils::Registers::A,
            vec![],
            None::<fn(&mut Mos6502)>
        );
    }

    #[test]
    fn lda_zpx() {
        let (addr, val, xreg) = (0x42, 0x37, 5);
        tests_utils::ld_into_reg(
            vec![
                (PC_START, instructions::LDA_ZPX),
                (PC_START + 1, addr),
                ((addr + xreg) as u16, val),
            ],
            val,
            instructions::LDA_ZPX_CCOST,
            tests_utils::Registers::A,
            vec![],
            Some(|cpu: &mut Mos6502| { cpu.set_xreg(xreg); })
        );
    }

    #[test]
    fn lda_zpx_wwrap() {
        let (addr, val, xreg) = (0x80, 0x37, 0xFF);
        tests_utils::ld_into_reg(
            vec![
                (PC_START, instructions::LDA_ZPX),
                (PC_START + 1, 0x80),
                (tests_utils::word_from_byte_addition(addr, xreg), val),
            ],
            val,
            instructions::LDA_ZPX_CCOST,
            tests_utils::Registers::A,
            vec![],
            Some(|cpu: &mut Mos6502| { cpu.set_xreg(xreg); })
        );
    }

    #[test]
    fn lda_abs() {
        let (lo, hi, val) = (0x80, 0x44, 0x37);
        tests_utils::ld_into_reg(
            vec![
                (PC_START, instructions::LDA_ABS),
                (PC_START + 1, lo),
                (PC_START + 2, hi),
                (tests_utils::word_from_bytes(hi, lo), val),
            ],
            val,
            instructions::LDA_ABS_CCOST,
            tests_utils::Registers::A,
            vec![],
            None::<fn(&mut Mos6502)>
        );
    }

    #[test]
    fn lda_absx_wopage_boundary() {
        let (hi, lo, xreg, val) = (0x44, 0x80, 1, 0x37);
        let dest_addr =
            tests_utils::word_from_bytes(hi, lo) + Word::from(xreg);
        tests_utils::ld_into_reg(
            vec![
                (PC_START, instructions::LDA_ABSX),
                (PC_START + 1, lo),
                (PC_START + 2, hi),
                (dest_addr, val),
            ],
            val,
            instructions::LDA_ABSX_CCOST,
            tests_utils::Registers::A,
            vec![],
            Some(|cpu: &mut Mos6502| { cpu.set_xreg(xreg) })
        );
    }

    #[test]
    fn lda_absx_wpage_boundary() {
        let (hi, lo, xreg, val) = (0x44, 0x02, 0xFF, 0x37);
        let dest_addr =
            tests_utils::word_from_bytes(hi, lo) + Word::from(xreg);
        tests_utils::ld_into_reg(
            vec![
                (PC_START, instructions::LDA_ABSX),
                (PC_START + 1, lo),
                (PC_START + 2, hi),
                (dest_addr, val),
            ],
            val,
            instructions::LDA_ABSX_CCOST + 1,
            tests_utils::Registers::A,
            vec![],
            Some(|cpu: &mut Mos6502| { cpu.set_xreg(xreg) })
        );
    }

    #[test]
    fn lda_absy_wopage_boundary() {
        let (hi, lo, yreg, val) = (0x44, 0x80, 1, 0x37);
        let dest_addr =
            tests_utils::word_from_bytes(hi, lo) + Word::from(yreg);
        tests_utils::ld_into_reg(
            vec![
                (PC_START, instructions::LDA_ABSY),
                (PC_START + 1, lo),
                (PC_START + 2, hi),
                (dest_addr, val),
            ],
            val,
            instructions::LDA_ABSY_CCOST,
            tests_utils::Registers::A,
            vec![],
            Some(|cpu: &mut Mos6502| { cpu.set_yreg(yreg) })
        );
    }

    #[test]
    fn lda_absy_wpage_boundary() {
        let (hi, lo, yreg, val) = (0x44, 0x02, 0xFF, 0x37);
        let dest_addr =
            tests_utils::word_from_bytes(hi, lo) + Word::from(yreg);
        tests_utils::ld_into_reg(
            vec![
                (PC_START, instructions::LDA_ABSY),
                (PC_START + 1, lo),
                (PC_START + 2, hi),
                (dest_addr, val),
            ],
            val,
            instructions::LDA_ABSY_CCOST + 1,
            tests_utils::Registers::A,
            vec![],
            Some(|cpu: &mut Mos6502| { cpu.set_yreg(yreg) })
        );
    }

    #[test]
    fn lda_zpx_ind() {
        let (hi, lo, xreg, val) = (0x80, 0x00, 0x04u8, 0x37);
        let arb_addr = 0x02u8;
        let dest_addr1 = tests_utils::word_from_byte_addition(arb_addr, xreg);
        let dest_addr2 = tests_utils::word_from_bytes(hi, lo);
        tests_utils::ld_into_reg(
            vec![
                (PC_START, instructions::LDA_ZPX_IND),
                (PC_START + 1, arb_addr),
                (dest_addr1, lo),
                (dest_addr1 + 1, hi),
                (dest_addr2, val),
            ],
            val,
            instructions::LDA_ZPX_IND_CCOST,
            tests_utils::Registers::A,
            vec![],
            Some(|cpu: &mut Mos6502| { cpu.set_xreg(xreg) })
        );
    }

    #[test]
    fn lda_zpy_ind_wopage_boundary() {
        let (hi, lo, yreg, val) = (0x80, 0x00, 0x04, 0x37);
        let arb_addr = 0x02;
        let dest_addr = tests_utils::word_from_bytes(hi, lo) + yreg as Word;
        tests_utils::ld_into_reg(
            vec![
                (PC_START, instructions::LDA_ZPY_IND),
                (PC_START + 1, arb_addr),
                (Word::from(arb_addr), lo),
                (Word::from(arb_addr) + 1, hi),
                (dest_addr, val),
            ],
            val,
            instructions::LDA_ZPY_IND_CCOST,
            tests_utils::Registers::A,
            vec![],
            Some(|cpu: &mut Mos6502| { cpu.set_yreg(yreg) })
        );
    }

    #[test]
    fn lda_zpy_ind_wpage_boundary() {
        let (hi, lo, yreg, val) = (0x80, 0x02, 0xFF, 0x37);
        let arb_addr = 0x02;
        let dest_addr = tests_utils::word_from_bytes(hi, lo) + yreg as Word;
        tests_utils::ld_into_reg(
            vec![
                (PC_START, instructions::LDA_ZPY_IND),
                (PC_START + 1, arb_addr),
                (Word::from(arb_addr), lo),
                (Word::from(arb_addr) + 1, hi),
                (dest_addr, val),
            ],
            val,
            instructions::LDA_ZPY_IND_CCOST + 1,
            tests_utils::Registers::A,
            vec![],
            Some(|cpu: &mut Mos6502| { cpu.set_yreg(yreg) })
        );
    }
}
