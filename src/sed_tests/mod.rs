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
    fn sed_imp() {
        let mut cpu = tests_utils::cpu_mem_set(vec![
            (0xFFFC, instructions::SED_IMP),
        ]);
        cpu.exe(Some(instructions::SED_IMP_CCOST));
        // tests_utils::assert_all_status_flags_false_except(&cpu, vec![Mos6502Flags::D]);
        assert!(cpu.decimal_mode_flag());
        assert_eq!(cpu.get_cycles(), instructions::SED_IMP_CCOST);
    }
}
