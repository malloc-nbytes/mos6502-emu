#[cfg(test)]
mod tests {
    use crate::mos6502::Mos6502;
    use crate::memory::Memory;
    use crate::instructions;

    fn cpu_mem_set(instrs: Vec<(u16, u8)>) -> Mos6502 {
        let mut mem = Memory::new();
        for (addr, opcode) in instrs {
            *mem.at(addr as usize) = opcode;
        }
        Mos6502::new(mem)
    }

    #[test]
    fn lda_imm() {
        let mut cpu = cpu_mem_set(vec![
            (0xFFFC, instructions::LDA_IMM),
            (0xFFFD, 0x42),
        ]);
        cpu.reset(instructions::LDA_IMM_CCOST);
        cpu.exe();
        assert_eq!(cpu.get_accumulator(), 0x42);
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
    }
}
