#[cfg(test)]
mod tests {
    use crate::mos6502::{Mos6502, Mos6502Flags};
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

        assert_flag_state(cpu.carry_flag(), "Carry", Mos6502Flags::C);
        assert_flag_state(cpu.zero_flag(), "Zero", Mos6502Flags::Z);
        assert_flag_state(cpu.interrupts_disable_flag(), "Interrupts Disable", Mos6502Flags::I);
        assert_flag_state(cpu.decimal_mode_flag(), "Decimal Mode", Mos6502Flags::D);
        assert_flag_state(cpu.break_flag(), "Break", Mos6502Flags::B);
        assert_flag_state(cpu.unused_flag(), "Unused", Mos6502Flags::U);
        assert_flag_state(cpu.overflow_flag(), "Overflow", Mos6502Flags::V);
        assert_flag_state(cpu.negative_flag(), "Negative", Mos6502Flags::N);
    }

    #[test]
    fn lda_imm() {
        let mut cpu = cpu_mem_set(vec![
            (0xFFFC, instructions::LDA_IMM),
            (0xFFFD, 0x84),
        ]);
        cpu.reset(instructions::LDA_IMM_CCOST);
        cpu.exe();
        assert_eq!(cpu.get_accumulator(), 0x84);
        assert_all_status_flags_false_except(&cpu, Some(Mos6502Flags::N));
    }

    #[test]
    fn lda_zp() {
        let mut cpu = cpu_mem_set(vec![
            (0xFFFC, instructions::LDA_ZP),
            (0xFFFD, 0x42),
            (0x0042, 0x80),
        ]);
        cpu.reset(instructions::LDA_ZP_CCOST);
        cpu.exe();
        assert_eq!(cpu.get_accumulator(), 0x80);
        assert_all_status_flags_false_except(&cpu, Some(Mos6502Flags::N));
    }

    #[test]
    fn lda_zpx() {
        let mut cpu = cpu_mem_set(vec![
            (0xFFFC, instructions::LDA_ZPX),
            (0xFFFD, 0x42),
            (0x0047, 0x37),
        ]);
        cpu.reset(instructions::LDA_ZPX_CCOST);
        cpu.set_xreg(5);
        cpu.exe();
        assert_eq!(cpu.get_accumulator(), 0x37);
        assert_all_status_flags_false_except(&cpu, Some(Mos6502Flags::N));
    }

    #[test]
    fn lda_zpx_wrap() {
        let mut cpu = cpu_mem_set(vec![
            (0xFFFC, instructions::LDA_ZPX),
            (0xFFFD, 0x80),
            (0x007F, 0x37),
        ]);
        cpu.reset(instructions::LDA_ZPX_CCOST);
        cpu.set_xreg(0xFF);
        cpu.exe();
        assert_eq!(cpu.get_accumulator(), 0x37);
        assert_all_status_flags_false_except(&cpu, Some(Mos6502Flags::N));
    }

    #[test]
    fn jsr_abs() {
        let mut cpu = cpu_mem_set(vec![
            (0xFFFC, instructions::JSR_ABS),
            (0xFFFD, 0x42),
            (0xFFFE, 0x42),
            (0x4242, instructions::LDA_IMM),
            (0x4243, 0x84),
        ]);
        cpu.reset(
            instructions::JSR_ABS_CCOST
                + instructions::LDA_IMM_CCOST);
        cpu.exe();
        assert_eq!(cpu.get_accumulator(), 0x84);
        // Excluding negative flag because of the instruction
        // LDA_IMM being put into memory.
        assert_all_status_flags_false_except(&cpu, Some(Mos6502Flags::N));
    }
}
