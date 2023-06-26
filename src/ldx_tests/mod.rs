#![allow(unused_imports)]

#[cfg(test)]
mod tests {
    use crate::mos6502::{
        Mos6502,
        Mos6502Flags,
    };
    use crate::memory::{
        Memory,
        Word,
        Byte,
    };
    use crate::instructions;
    use crate::tests_utils;
    use crate::tests_utils::PC_START;

    #[test]
    fn ldx_imm() {
        let val = 0x84;
        tests_utils::ld_into_reg(
            vec![
                (PC_START, instructions::LDX_IMM),
                (PC_START + 1, val),
            ],
            val,
            instructions::LDX_IMM_CCOST,
            tests_utils::Registers::X,
            vec![Mos6502Flags::N],
            None::<fn(&mut Mos6502)>
        );
    }

    #[test]
    fn ldx_imm_wzero() {
        let val = 0x00;
        tests_utils::ld_into_reg(
            vec![
                (PC_START, instructions::LDX_IMM),
                (PC_START + 1, val),
            ],
            val,
            instructions::LDX_IMM_CCOST,
            tests_utils::Registers::X,
            vec![Mos6502Flags::Z],
            None::<fn(&mut Mos6502)>
        );
    }

    #[test]
    fn ldx_abs() {
        let (hi, lo, val) = (0x44, 0x80, 0x37);
        let dest_addr = tests_utils::word_from_bytes(hi, lo);
        tests_utils::ld_into_reg(
            vec![
                (PC_START, instructions::LDX_ABS),
                (PC_START + 1, lo),
                (PC_START + 2, hi),
                (dest_addr, val),
            ],
            val,
            instructions::LDX_ABS_CCOST,
            tests_utils::Registers::X,
            vec![],
            None::<fn(&mut Mos6502)>
        );
    }

    #[test]
    fn ldx_absy_wopage_boundary() {
        let (hi, lo, yreg, val) = (0x44, 0x80, 1u8, 0x37);
        let dest_addr = tests_utils::word_from_bytes(hi, lo) + yreg as Word;
        tests_utils::ld_into_reg(
            vec![
                (PC_START, instructions::LDX_ABSY),
                (PC_START + 1, lo),
                (PC_START + 2, hi),
                (dest_addr, val),
            ],
            val,
            instructions::LDX_ABSY_CCOST,
            tests_utils::Registers::X,
            vec![],
            Some(|cpu: &mut Mos6502| { cpu.set_yreg(yreg) })
        );
    }

    #[test]
    fn ldx_absy_wpage_boundary() {
        let (hi, lo, yreg, val) = (0x44, 0x02, 0xFF, 0x37);
        let dest_addr = tests_utils::word_from_bytes(hi, lo) + yreg as Word;
        tests_utils::ld_into_reg(
            vec![
                (PC_START, instructions::LDX_ABSY),
                (PC_START + 1, lo),
                (PC_START + 2, hi),
                (dest_addr, val),
            ],
            val,
            instructions::LDX_ABSY_CCOST + 1,
            tests_utils::Registers::X,
            vec![],
            Some(|cpu: &mut Mos6502| { cpu.set_yreg(yreg) })
        );
    }

    #[test]
    fn ldx_zp() {
        let mut cpu = tests_utils::cpu_mem_set(vec![
            (0xFFFC, instructions::LDX_ZP),
            (0xFFFD, 0x42),
            (0x0042, 0x37),
        ]);

        cpu.exe(Some(instructions::LDX_ZP_CCOST));

        assert_eq!(cpu.get_xreg(), 0x37);
        assert_eq!(cpu.get_cycles(), instructions::LDX_ZP_CCOST);
        // tests_utils::assert_all_status_flags_false_except(&cpu, vec![]);
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
        // tests_utils::assert_all_status_flags_false_except(&cpu, vec![]);
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
        // tests_utils::assert_all_status_flags_false_except(&cpu, vec![]);
    }

}
