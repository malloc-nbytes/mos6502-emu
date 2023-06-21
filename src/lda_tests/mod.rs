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

    #[test]
    fn lda_imm() {
        let val = 0x84;
        tests_utils::ld_into_reg(
            vec![
                (0xFFFC, instructions::LDA_IMM),
                (0xFFFD, val),
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
                (0xFFFC, instructions::LDA_IMM),
                (0xFFFD, val),
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
                (0xFFFC, instructions::LDA_ZP),
                (0xFFFD, addr),
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
                (0xFFFC, instructions::LDA_ZPX),
                (0xFFFD, addr),
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
                (0xFFFC, instructions::LDA_ZPX),
                (0xFFFD, 0x80),
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
        let (val, lo, hi) = (0x37, 0x80, 0x44);
        tests_utils::ld_into_reg(
            vec![
                (0xFFFC, instructions::LDA_ABS),
                (0xFFFD, lo),
                (0xFFFE, hi),
                (tests_utils::word_from_bytes(lo, hi), val),
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
        let mut cpu = tests_utils::cpu_mem_set(vec![
            (0xFFFC, instructions::LDA_ABSX),
            (0xFFFD, 0x80),
            (0xFFFE, 0x44), // 4480
            (0x4481, 0x37),
        ]);

        cpu.set_xreg(1);
        cpu.exe(Some(instructions::LDA_ABSX_CCOST));

        assert_eq!(cpu.get_accumulator(), 0x37);
        assert_eq!(cpu.get_cycles(), instructions::LDA_ABSX_CCOST);

        tests_utils::assert_all_status_flags_false_except(&cpu, vec![]);
    }

    #[test]
    fn lda_absx_wpage_boundary() {
        let mut cpu = tests_utils::cpu_mem_set(vec![
            (0xFFFC, instructions::LDA_ABSX),
            (0xFFFD, 0x02),
            (0xFFFE, 0x44), // 0x4402
            (0x4501, 0x37), // 0x4402 + 0xFF crosses page boundary
        ]);

        cpu.set_xreg(0xFF);
        cpu.exe(Some(instructions::LDA_ABSX_CCOST + 1));

        assert_eq!(cpu.get_accumulator(), 0x37);
        assert_eq!(cpu.get_cycles(), instructions::LDA_ABSX_CCOST + 1);

        tests_utils::assert_all_status_flags_false_except(&cpu, vec![]);
    }

    #[test]
    fn lda_absy_wopage_boundary() {
        let mut cpu = tests_utils::cpu_mem_set(vec![
            (0xFFFC, instructions::LDA_ABSY),
            (0xFFFD, 0x80),
            (0xFFFE, 0x44), // 4480
            (0x4481, 0x37),
        ]);

        cpu.set_yreg(1);
        cpu.exe(Some(instructions::LDA_ABSY_CCOST));

        assert_eq!(cpu.get_accumulator(), 0x37);
        assert_eq!(cpu.get_cycles(), instructions::LDA_ABSY_CCOST);

        tests_utils::assert_all_status_flags_false_except(&cpu, vec![]);
    }

    #[test]
    fn lda_absy_wpage_boundary() {
        let mut cpu = tests_utils::cpu_mem_set(vec![
            (0xFFFC, instructions::LDA_ABSY),
            (0xFFFD, 0x02),
            (0xFFFE, 0x44), // 0x4402
            (0x4501, 0x37), // 0x4402 + 0xFF crosses page boundary
        ]);

        cpu.set_yreg(0xFF);
        cpu.exe(Some(instructions::LDA_ABSY_CCOST + 1));

        assert_eq!(cpu.get_accumulator(), 0x37);
        assert_eq!(cpu.get_cycles(), instructions::LDA_ABSY_CCOST + 1);

        tests_utils::assert_all_status_flags_false_except(&cpu, vec![]);
    }

    #[test]
    fn lda_zpx_ind() {
        let mut cpu = tests_utils::cpu_mem_set(vec![
            (0xFFFC, instructions::LDA_ZPX_IND),
            (0xFFFD, 0x02),
            (0x0006, 0x00), // 0x0006 = 0x02 + 0x04 (xreg)
            (0x0007, 0x80),
            (0x8000, 0x37),
        ]);

        cpu.set_xreg(0x04);
        cpu.exe(Some(instructions::LDA_ZPX_IND_CCOST));

        assert_eq!(cpu.get_accumulator(), 0x37);
        assert_eq!(cpu.get_cycles(), instructions::LDA_ZPX_IND_CCOST);

        tests_utils::assert_all_status_flags_false_except(&cpu, vec![]);
    }

    #[test]
    fn lda_zpy_ind_wopage_boundary() {
        let mut cpu = tests_utils::cpu_mem_set(vec![
            (0xFFFC, instructions::LDA_ZPY_IND),
            (0xFFFD, 0x02),
            (0x0002, 0x00),
            (0x0003, 0x80),
            (0x8004, 0x37), // 0x8004 = 0x8000 + 0x04 (yreg)
        ]);

        cpu.set_yreg(0x04);
        cpu.exe(Some(instructions::LDA_ZPY_IND_CCOST));

        assert_eq!(cpu.get_accumulator(), 0x37);
        assert_eq!(cpu.get_cycles(), instructions::LDA_ZPY_IND_CCOST);

        tests_utils::assert_all_status_flags_false_except(&cpu, vec![]);
    }

    #[test]
    fn lda_zpy_ind_wpage_boundary() {
        let mut cpu = tests_utils::cpu_mem_set(vec![
            (0xFFFC, instructions::LDA_ZPY_IND),
            (0xFFFD, 0x02),
            (0x0002, 0x02),
            (0x0003, 0x80),
            (0x8101, 0x37), // 0x8101 = 0x8002 + 0xFF (yreg)
        ]);

        cpu.set_yreg(0xFF);
        cpu.exe(Some(instructions::LDA_ZPY_IND_CCOST + 1));

        assert_eq!(cpu.get_accumulator(), 0x37);
        assert_eq!(cpu.get_cycles(), instructions::LDA_ZPY_IND_CCOST + 1);

        tests_utils::assert_all_status_flags_false_except(&cpu, vec![]);
    }
}
