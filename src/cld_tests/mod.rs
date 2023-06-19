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
    fn cld_imp() {
        let mut cpu = tests_utils::cpu_mem_set(vec![
            (0xFFFC, instructions::CLD_IMP),
        ]);
        cpu.exe(Some(instructions::CLD_IMP_CCOST));
        tests_utils::assert_all_status_flags_false_except(&cpu, vec![]);
        assert_eq!(cpu.get_cycles(), instructions::CLD_IMP_CCOST);
    }

    #[test]
    fn cld_imp_wset() {
        let mut cpu = tests_utils::cpu_mem_set(vec![
            (0xFFFC, instructions::CLD_IMP),
        ]);
        cpu.set_status_flag(Mos6502Flags::D);
        cpu.exe(Some(instructions::CLD_IMP_CCOST));
        tests_utils::assert_all_status_flags_false_except(&cpu, vec![]);
        assert_eq!(cpu.get_cycles(), instructions::CLD_IMP_CCOST);
    }
}
