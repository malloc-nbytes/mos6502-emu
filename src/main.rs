#![warn(clippy::pedantic)]

mod instructions;
mod mos6502;
mod memory;

mod lda_tests;
mod ldx_tests;
mod jsr_tests;
mod nop_tests;
mod sec_tests;
mod sei_tests;
mod sed_tests;
mod cli_tests;
mod clv_tests;
mod cld_tests;
mod tests_utils;

fn cpu_mem_set(instrs: Vec<(u16, u8)>) -> mos6502::Mos6502 {
    let mut mem = memory::Memory::new();
    for (addr, opcode) in instrs {
        *mem.at(addr as usize) = opcode;
    }
    let mut cpu = mos6502::Mos6502::new(mem);
    cpu.reset(false);
    cpu
}

#[allow(dead_code)]
#[allow(unused_mut)]
#[allow(unused_variables)]
fn main() {
    let mut cpu = cpu_mem_set(vec![
        (0xFFFC, instructions::LDA_IMM),
        (0xFFFD, 0x84),
    ]);
    cpu.exe(Some(instructions::LDA_IMM_CCOST));
    println!("{cpu}");
}
