#![allow(unused_imports)]

#[cfg(test)]
mod tests {
    use crate::mos6502::{
        Mos6502,
        Mos6502Flags,
    };
    use crate::memory::Memory;
    use crate::instructions;
    use crate::tests_utils;

    #[test]
    fn ldx_imm() {
        let mut cpu = tests_utils::cpu_mem_set(vec![
            (0xFFFC, instructions::LDX_IMM),
            (0xFFFD, 0x84),
        ]);

        cpu.exe(Some(instructions::LDX_IMM_CCOST));

        assert_eq!(cpu.get_accumulator(), 0x84);
        assert_eq!(cpu.get_cycles(), instructions::LDX_IMM_CCOST);

        tests_utils::assert_all_status_flags_false_except(&cpu, vec![Mos6502Flags::N]);
        assert!(cpu.negative_flag());
    }

    #[test]
    fn ldx_imm_wzero() {
        let mut cpu = tests_utils::cpu_mem_set(vec![
            (0xFFFC, instructions::LDX_IMM),
            (0xFFFD, 0x00),
        ]);

        cpu.set_xreg(0x44);
        cpu.exe(Some(instructions::LDX_IMM_CCOST));

        assert_eq!(cpu.get_cycles(), instructions::LDX_IMM_CCOST);

        tests_utils::assert_all_status_flags_false_except(&cpu, vec![Mos6502Flags::Z]);
        assert!(cpu.zero_flag());
    }

    #[test]
    fn ldx_abs() {
        let mut cpu = tests_utils::cpu_mem_set(vec![
            (0xFFFC, instructions::LDX_ABS),
            (0xFFFD, 0x80),
            (0xFFFE, 0x44), // 4480
            (0x4480, 0x37),
        ]);

        cpu.exe(Some(instructions::LDX_ABS_CCOST));

        assert_eq!(cpu.get_accumulator(), 0x37);
        assert_eq!(cpu.get_cycles(), instructions::LDX_ABS_CCOST);
        tests_utils::assert_all_status_flags_false_except(&cpu, vec![]);
    }

    #[test]
    fn ldx_absy_wopage_boundary() {
        let mut cpu = tests_utils::cpu_mem_set(vec![
            (0xFFFC, instructions::LDX_ABSY),
            (0xFFFD, 0x80),
            (0xFFFE, 0x44), // 4480
            (0x4481, 0x37),
        ]);

        cpu.set_yreg(1);
        cpu.exe(Some(instructions::LDX_ABSY_CCOST));

        assert_eq!(cpu.get_accumulator(), 0x37);
        assert_eq!(cpu.get_cycles(), instructions::LDX_ABSY_CCOST);

        tests_utils::assert_all_status_flags_false_except(&cpu, vec![]);
    }

    #[test]
    fn ldx_absy_wpage_boundary() {
        let mut cpu = tests_utils::cpu_mem_set(vec![
            (0xFFFC, instructions::LDX_ABSY),
            (0xFFFD, 0x02),
            (0xFFFE, 0x44), // 0x4402
            (0x4501, 0x37), // 0x4402 + 0xFF crosses page boundary
        ]);

        cpu.set_yreg(0xFF);
        cpu.exe(Some(instructions::LDX_ABSY_CCOST + 1));

        assert_eq!(cpu.get_accumulator(), 0x37);
        assert_eq!(cpu.get_cycles(), instructions::LDX_ABSY_CCOST + 1);

        tests_utils::assert_all_status_flags_false_except(&cpu, vec![]);
    }

    #[test]
    fn ldx_zp() {
        let mut cpu = tests_utils::cpu_mem_set(vec![
            (0xFFFC, instructions::LDX_ZP),
            (0xFFFD, 0x42),
            (0x0042, 0x37),
        ]);

        cpu.exe(Some(instructions::LDX_ZP_CCOST));

        assert_eq!(cpu.get_accumulator(), 0x37);
        assert_eq!(cpu.get_cycles(), instructions::LDX_ZP_CCOST);
        tests_utils::assert_all_status_flags_false_except(&cpu, vec![]);
    }

    #[test]
    fn ldx_zpy() {
        let mut cpu = tests_utils::cpu_mem_set(vec![
            (0xFFFC, instructions::LDX_ZPY),
            (0xFFFD, 0x42),
            (0x0047, 0x37),
        ]);

        cpu.set_xreg(5);
        cpu.exe(Some(instructions::LDX_ZPY_CCOST));

        assert_eq!(cpu.get_accumulator(), 0x37);
        assert_eq!(cpu.get_cycles(), instructions::LDX_ZPY_CCOST);
        tests_utils::assert_all_status_flags_false_except(&cpu, vec![]);
    }

    #[test]
    fn ldx_zpy_wwrap() {
        let mut cpu = tests_utils::cpu_mem_set(vec![
            (0xFFFC, instructions::LDX_ZPY),
            (0xFFFD, 0x80),
            (0x007F, 0x37),
        ]);

        cpu.set_xreg(0xFF);
        cpu.exe(Some(instructions::LDX_ZPY_CCOST));

        assert_eq!(cpu.get_accumulator(), 0x37);
        assert_eq!(cpu.get_cycles(), instructions::LDX_ZPY_CCOST);
        tests_utils::assert_all_status_flags_false_except(&cpu, vec![]);
    }

}
