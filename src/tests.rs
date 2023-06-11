#[cfg(test)]
mod tests {
    use crate::mos6502::Mos6502;
    use crate::memory::Memory;
    use crate::instructions;

    fn setup_cpu(instrs: Vec<(u16, u8)>) -> Mos6502 {
        let mut mem = Memory::new();
        for (addr, opcode) in instrs {
            *mem.at(addr as usize) = opcode;
        }
        Mos6502::new(mem)
    }

    #[test]
    fn lda_imm() {
        let mut cpu = setup_cpu(vec![
            (0x0000, instructions::LDA_IMM),
            (0x0001, 0x42),
        ]);
        cpu.reset(instructions::LDA_IMM_CCOST);
        cpu.exe();
        assert_eq!(cpu.get_accumulator(), 0x42);
    }

    #[test]
    fn jsr_abs() {
        let mut cpu = setup_cpu(vec![
            (0x0000, instructions::JSR_ABS),
            (0x0001, 0x42),
            (0x0002, 0x42),
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
