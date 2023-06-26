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
    fn clv_imp() {
        let mut cpu = tests_utils::cpu_mem_set(vec![
            (0xFFFC, instructions::CLV_IMP),
        ]);
        cpu.exe(Some(instructions::CLV_IMP_CCOST));
        // tests_utils::assert_all_status_flags_false_except(&cpu, vec![]);
        assert_eq!(cpu.get_cycles(), instructions::CLV_IMP_CCOST);
    }

    #[test]
    fn clv_imp_wset() {
        let mut cpu = tests_utils::cpu_mem_set(vec![
            (0xFFFC, instructions::CLV_IMP),
        ]);
        cpu.set_status_flag(Mos6502Flags::V);
        cpu.exe(Some(instructions::CLV_IMP_CCOST));
        // tests_utils::assert_all_status_flags_false_except(&cpu, vec![]);
        assert_eq!(cpu.get_cycles(), instructions::CLV_IMP_CCOST);
    }
}
