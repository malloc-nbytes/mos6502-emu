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
    fn jsr_abs_wlda_imm() {
        let mut cpu = cpu_mem_set(vec![
            (0xFFFC, instructions::JSR_ABS),
            (0xFFFD, 0x42),
            (0xFFFE, 0x42),
            (0x4242, instructions::LDA_IMM),
            (0x4243, 0x84),
        ]);

        cpu.exe(Some(instructions::JSR_ABS_CCOST + instructions::LDA_IMM_CCOST));

        assert_eq!(cpu.get_accumulator(), 0x84);
        assert_all_status_flags_false_except(&cpu, vec![Mos6502Flags::N]);
        assert!(cpu.negative_flag());
        assert_eq!(cpu.get_cycles(), instructions::JSR_ABS_CCOST + instructions::LDA_IMM_CCOST);
    }
}
