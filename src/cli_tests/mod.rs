#[allow(unused_imports)]

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
    fn cli_imp() {
        let mut cpu = tests_utils::cpu_mem_set(vec![
            (0xFFFC, instructions::CLI_IMP),
        ]);
        cpu.exe(Some(instructions::CLI_IMP_CCOST));
        // tests_utils::assert_all_status_flags_false_except(&cpu, vec![]);
        assert_eq!(cpu.get_cycles(), instructions::CLI_IMP_CCOST);
    }

    #[test]
    fn cli_imp_wset() {
        let mut cpu = tests_utils::cpu_mem_set(vec![
            (0xFFFC, instructions::CLI_IMP),
        ]);
        cpu.set_status_flag(Mos6502Flags::I);
        cpu.exe(Some(instructions::CLI_IMP_CCOST));
        // tests_utils::assert_all_status_flags_false_except(&cpu, vec![]);
        assert_eq!(cpu.get_cycles(), instructions::CLI_IMP_CCOST);
    }
}
