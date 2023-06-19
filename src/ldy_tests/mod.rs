#![allow(unused_imports)]

#[cfg(test)]
mod tests {
    use crate::mos6502::{
        Mos6502,
        Mos6502Flags,
    };
    use crate::memory::Memory;
    use crate::instructions;
    use crate::tests_utils::{cpu_mem_set, assert_all_status_flags_false_except};

    
    #[test]
    fn ldy_imm() {
        let mut cpu = cpu_mem_set(vec![
            (0xFFFC, instructions::LDY_IMM),
            (0xFFFD, 0x84),
        ]);

        cpu.exe(Some(instructions::LDY_IMM_CCOST));

        assert_eq!(cpu.get_accumulator(), 0x84);
        assert_eq!(cpu.get_cycles(), instructions::LDY_IMM_CCOST);

        assert_all_status_flags_false_except(&cpu, vec![Mos6502Flags::N]);
        assert!(cpu.negative_flag());
    }

    #[test]
    fn ldy_imm_wzero() {
        let mut cpu = cpu_mem_set(vec![
            (0xFFFC, instructions::LDY_IMM),
            (0xFFFD, 0x00),
        ]);

        cpu.set_xreg(0x44);
        cpu.exe(Some(instructions::LDY_IMM_CCOST));

        assert_eq!(cpu.get_cycles(), instructions::LDY_IMM_CCOST);

        assert_all_status_flags_false_except(&cpu, vec![Mos6502Flags::Z]);
        assert!(cpu.zero_flag());
    }

    #[test]
    fn ldy_abs() {
        let mut cpu = cpu_mem_set(vec![
            (0xFFFC, instructions::LDY_ABS),
            (0xFFFD, 0x80),
            (0xFFFE, 0x44), // 4480
            (0x4480, 0x37),
        ]);

        cpu.exe(Some(instructions::LDY_ABS_CCOST));

        assert_eq!(cpu.get_accumulator(), 0x37);
        assert_eq!(cpu.get_cycles(), instructions::LDY_ABS_CCOST);
        assert_all_status_flags_false_except(&cpu, vec![]);
    }

    #[test]
    fn ldy_absx_wopage_boundary() {
        let mut cpu = cpu_mem_set(vec![
            (0xFFFC, instructions::LDY_ABSX),
            (0xFFFD, 0x80),
            (0xFFFE, 0x44), // 4480
            (0x4481, 0x37),
        ]);

        cpu.set_yreg(1);
        cpu.exe(Some(instructions::LDY_ABSX_CCOST));

        assert_eq!(cpu.get_accumulator(), 0x37);
        assert_eq!(cpu.get_cycles(), instructions::LDY_ABSX_CCOST);

        assert_all_status_flags_false_except(&cpu, vec![]);
    }

    #[test]
    fn ldy_absx_wpage_boundary() {
        let mut cpu = cpu_mem_set(vec![
            (0xFFFC, instructions::LDY_ABSX),
            (0xFFFD, 0x02),
            (0xFFFE, 0x44), // 0x4402
            (0x4501, 0x37), // 0x4402 + 0xFF crosses page boundary
        ]);

        cpu.set_yreg(0xFF);
        cpu.exe(Some(instructions::LDY_ABSX_CCOST + 1));

        assert_eq!(cpu.get_accumulator(), 0x37);
        assert_eq!(cpu.get_cycles(), instructions::LDY_ABSX_CCOST + 1);

        assert_all_status_flags_false_except(&cpu, vec![]);
    }

    #[test]
    fn ldy_zp() {
        let mut cpu = cpu_mem_set(vec![
            (0xFFFC, instructions::LDY_ZP),
            (0xFFFD, 0x42),
            (0x0042, 0x37),
        ]);

        cpu.exe(Some(instructions::LDY_ZP_CCOST));

        assert_eq!(cpu.get_accumulator(), 0x37);
        assert_eq!(cpu.get_cycles(), instructions::LDY_ZP_CCOST);
        assert_all_status_flags_false_except(&cpu, vec![]);
    }

    #[test]
    fn ldy_zpx() {
        let mut cpu = cpu_mem_set(vec![
            (0xFFFC, instructions::LDY_ZPX),
            (0xFFFD, 0x42),
            (0x0047, 0x37),
        ]);

        cpu.set_xreg(5);
        cpu.exe(Some(instructions::LDY_ZPX_CCOST));

        assert_eq!(cpu.get_accumulator(), 0x37);
        assert_eq!(cpu.get_cycles(), instructions::LDY_ZPX_CCOST);
        assert_all_status_flags_false_except(&cpu, vec![]);
    }

    #[test]
    fn ldy_zpx_wwrap() {
        let mut cpu = cpu_mem_set(vec![
            (0xFFFC, instructions::LDY_ZPX),
            (0xFFFD, 0x80),
            (0x007F, 0x37),
        ]);

        cpu.set_xreg(0xFF);
        cpu.exe(Some(instructions::LDY_ZPX_CCOST));

        assert_eq!(cpu.get_accumulator(), 0x37);
        assert_eq!(cpu.get_cycles(), instructions::LDY_ZPX_CCOST);
        assert_all_status_flags_false_except(&cpu, vec![]);
    }

}
