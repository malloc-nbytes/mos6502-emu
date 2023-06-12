#[cfg(test)]
mod tests {
    use crate::mos6502::{
        Mos6502,
        Mos6502Flags,
    };
    use crate::memory::Memory;
    use crate::instructions;

    fn cpu_mem_set(instrs: Vec<(u16, u8)>) -> Mos6502 {
        let mut mem = Memory::new();
        for (addr, opcode) in instrs {
            *mem.at(addr as usize) = opcode;
        }
        Mos6502::new(mem)
    }

    fn assert_all_status_flags_false_except(cpu: &Mos6502, excluded_flag: Option<Mos6502Flags>) {
        let assert_flag_state = |flag: bool, flag_name: &str, current_flag: Mos6502Flags| {
            if Some(current_flag) != excluded_flag {
                assert!(!flag, "Failed assertion for flag: {flag_name}");
            }
        };

        assert_flag_state(cpu.carry_flag(), "CARRY", Mos6502Flags::C);
        assert_flag_state(cpu.zero_flag(), "ZERO", Mos6502Flags::Z);
        assert_flag_state(cpu.interrupts_disable_flag(), "INTERRUPTS DISABLE", Mos6502Flags::I);
        assert_flag_state(cpu.decimal_mode_flag(), "DECIMAL MODE", Mos6502Flags::D);
        assert_flag_state(cpu.break_flag(), "BREAK", Mos6502Flags::B);
        assert_flag_state(cpu.unused_flag(), "UNUSED", Mos6502Flags::U);
        assert_flag_state(cpu.overflow_flag(), "OVERFLOW", Mos6502Flags::V);
        assert_flag_state(cpu.negative_flag(), "NEGATIVE", Mos6502Flags::N);
    }

    ////////// LDA //////////

    #[test]
    fn lda_imm() {
        let mut cpu = cpu_mem_set(vec![
            (0xFFFC, instructions::LDA_IMM),
            (0xFFFD, 0x32),
        ]);
        cpu.reset();
        cpu.exe(Some(instructions::LDA_IMM_CCOST));
        assert_eq!(cpu.get_accumulator(), 0x32);
        assert_all_status_flags_false_except(&cpu, Some(Mos6502Flags::N));
        assert_eq!(cpu.get_cycles(), instructions::LDA_IMM_CCOST); 
    }

    #[test]
    fn lda_imm_wzero_flag() {
        let mut cpu = cpu_mem_set(vec![
            (0xFFFC, instructions::LDA_IMM),
            (0xFFFD, 0x00),
        ]);
        cpu.reset();
        cpu.exe(Some(instructions::LDA_IMM_CCOST));
        assert_eq!(cpu.get_accumulator(), 0x00);
        assert_all_status_flags_false_except(&cpu, Some(Mos6502Flags::Z));
        assert!(cpu.zero_flag());
        assert_eq!(cpu.get_cycles(), instructions::LDA_IMM_CCOST);
    }

    #[test]
    fn lda_zp() {
        let mut cpu = cpu_mem_set(vec![
            (0xFFFC, instructions::LDA_ZP),
            (0xFFFD, 0x42),
            (0x0042, 0x80),
        ]);
        cpu.reset();
        cpu.exe(Some(instructions::LDA_ZP_CCOST));
        assert_eq!(cpu.get_accumulator(), 0x80);
        assert_all_status_flags_false_except(&cpu, Some(Mos6502Flags::N));
        assert!(cpu.negative_flag());
        assert_eq!(cpu.get_cycles(), instructions::LDA_ZP_CCOST);
    }

    #[test]
    fn lda_zpx() {
        let mut cpu = cpu_mem_set(vec![
            (0xFFFC, instructions::LDA_ZPX),
            (0xFFFD, 0x42),
            (0x0047, 0x37),
        ]);
        cpu.reset();
        cpu.set_xreg(5);
        cpu.exe(Some(instructions::LDA_ZPX_CCOST));
        assert_eq!(cpu.get_accumulator(), 0x37);
        assert_all_status_flags_false_except(&cpu, None);
        assert_eq!(cpu.get_cycles(), instructions::LDA_ZPX_CCOST);
    }

    #[test]
    fn lda_zpx_wwrap() {
        let mut cpu = cpu_mem_set(vec![
            (0xFFFC, instructions::LDA_ZPX),
            (0xFFFD, 0x80),
            (0x007F, 0x37),
        ]);
        cpu.reset();
        cpu.set_xreg(0xFF);
        cpu.exe(Some(instructions::LDA_ZPX_CCOST));
        assert_eq!(cpu.get_accumulator(), 0x37);
        assert_all_status_flags_false_except(&cpu, None);
        assert_eq!(cpu.get_cycles(), instructions::LDA_ZPX_CCOST);
    }

    ////////// JSR //////////

    #[test]
    fn jsr_abs() {
        let mut cpu = cpu_mem_set(vec![
            (0xFFFC, instructions::JSR_ABS),
            (0xFFFD, 0x42),
            (0xFFFE, 0x42),
            (0x4242, instructions::LDA_IMM),
            (0x4243, 0x84),
        ]);
        cpu.reset();
        cpu.exe(Some(instructions::JSR_ABS_CCOST + instructions::LDA_IMM_CCOST));
        assert_eq!(cpu.get_accumulator(), 0x84);
        // Excluding negative flag because of the instruction
        // LDA_IMM being put into memory.
        assert_all_status_flags_false_except(&cpu, Some(Mos6502Flags::N));
        assert!(cpu.negative_flag());
        assert_eq!(cpu.get_cycles(), instructions::JSR_ABS_CCOST + instructions::LDA_IMM_CCOST);
    }

    ////////// SEC //////////

    #[test]
    fn nop() {
        let mut cpu = cpu_mem_set(vec![
            (0xFFFC, instructions::NOP_IMP),
        ]);
        cpu.reset();
        cpu.exe(Some(instructions::NOP_IMP_CCOST));
        assert_all_status_flags_false_except(&cpu, None);
        assert_eq!(cpu.get_cycles(), instructions::NOP_IMP_CCOST);
    }

    ////////// SEC //////////

    #[test]
    fn sec_imp() {
        let mut cpu = cpu_mem_set(vec![
            (0xFFFC, instructions::SEC_IMP),
        ]);
        cpu.reset();
        cpu.exe(Some(instructions::SEC_IMP_CCOST));
        assert_all_status_flags_false_except(&cpu, Some(Mos6502Flags::C));
        assert!(cpu.carry_flag());
        assert_eq!(cpu.get_cycles(), instructions::SEC_IMP_CCOST);
    }

    ////////// SEI //////////

    #[test]
    fn sei_imp() {
        let mut cpu = cpu_mem_set(vec![
            (0xFFFC, instructions::SEI_IMP),
        ]);
        cpu.reset();
        cpu.exe(Some(instructions::SEI_IMP_CCOST));
        assert_all_status_flags_false_except(&cpu, Some(Mos6502Flags::I));
        assert!(cpu.interrupts_disable_flag());
        assert_eq!(cpu.get_cycles(), instructions::SEI_IMP_CCOST);
    }

    ////////// SED //////////

    #[test]
    fn sed_imp() {
        let mut cpu = cpu_mem_set(vec![
            (0xFFFC, instructions::SED_IMP),
        ]);
        cpu.reset();
        cpu.exe(Some(instructions::SED_IMP_CCOST));
        assert_all_status_flags_false_except(&cpu, Some(Mos6502Flags::D));
        assert!(cpu.decimal_mode_flag());
        assert_eq!(cpu.get_cycles(), instructions::SED_IMP_CCOST);
    }

    ////////// CLI //////////

    #[test]
    fn cli_imp() {
        let mut cpu = cpu_mem_set(vec![
            (0xFFFC, instructions::CLI_IMP),
        ]);
        cpu.reset();
        cpu.exe(Some(instructions::CLI_IMP_CCOST));
        assert_all_status_flags_false_except(&cpu, None);
        assert_eq!(cpu.get_cycles(), instructions::CLI_IMP_CCOST);
    }

    #[test]
    fn cli_imp_wset() {
        let mut cpu = cpu_mem_set(vec![
            (0xFFFC, instructions::CLI_IMP),
        ]);
        cpu.reset();
        cpu.set_status_flag(Mos6502Flags::I);
        cpu.exe(Some(instructions::CLI_IMP_CCOST));
        assert_all_status_flags_false_except(&cpu, None);
        assert_eq!(cpu.get_cycles(), instructions::CLI_IMP_CCOST);
    }

    ////////// CLV //////////

    #[test]
    fn clv_imp() {
        let mut cpu = cpu_mem_set(vec![
            (0xFFFC, instructions::CLV_IMP),
        ]);
        cpu.reset();
        cpu.exe(Some(instructions::CLV_IMP_CCOST));
        assert_all_status_flags_false_except(&cpu, None);
        assert_eq!(cpu.get_cycles(), instructions::CLV_IMP_CCOST);
    }

    #[test]
    fn clv_imp_wset() {
        let mut cpu = cpu_mem_set(vec![
            (0xFFFC, instructions::CLV_IMP),
        ]);
        cpu.reset();
        cpu.set_status_flag(Mos6502Flags::V);
        cpu.exe(Some(instructions::CLV_IMP_CCOST));
        assert_all_status_flags_false_except(&cpu, None);
        assert_eq!(cpu.get_cycles(), instructions::CLV_IMP_CCOST);
    }

    ////////// CLD //////////

    #[test]
    fn cld_imp() {
        let mut cpu = cpu_mem_set(vec![
            (0xFFFC, instructions::CLD_IMP),
        ]);
        cpu.reset();
        cpu.exe(Some(instructions::CLD_IMP_CCOST));
        assert_all_status_flags_false_except(&cpu, None);
        assert_eq!(cpu.get_cycles(), instructions::CLD_IMP_CCOST);
    }

    #[test]
    fn cld_imp_wset() {
        let mut cpu = cpu_mem_set(vec![
            (0xFFFC, instructions::CLD_IMP),
        ]);
        cpu.reset();
        cpu.set_status_flag(Mos6502Flags::D);
        cpu.exe(Some(instructions::CLD_IMP_CCOST));
        assert_all_status_flags_false_except(&cpu, None);
        assert_eq!(cpu.get_cycles(), instructions::CLD_IMP_CCOST);
    }
}
