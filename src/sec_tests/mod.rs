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
    fn sec_imp() {
        let mut cpu = tests_utils::cpu_mem_set(vec![
            (0xFFFC, instructions::SEC_IMP),
        ]);
        cpu.exe(Some(instructions::SEC_IMP_CCOST));
        tests_utils::assert_all_status_flags_false_except(&cpu, vec![Mos6502Flags::C]);
        assert!(cpu.carry_flag());
        assert_eq!(cpu.get_cycles(), instructions::SEC_IMP_CCOST);
    }
}
