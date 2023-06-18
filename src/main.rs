#![warn(clippy::pedantic)]

mod instructions;
mod mos6502;
mod memory;

mod tests;

fn cpu_mem_set(instrs: Vec<(u16, u8)>) -> mos6502::Mos6502 {
    let mut mem = memory::Memory::new();
    for (addr, opcode) in instrs {
        *mem.at(addr as usize) = opcode;
    }
    let mut cpu = mos6502::Mos6502::new(mem);
    cpu.reset();
    cpu
}

#[allow(dead_code)]
#[allow(unused_mut)]
#[allow(unused_variables)]
fn main() {
    let mut cpu = cpu_mem_set(vec![
        (0xFFFC, instructions::JSR_ABS),
        (0xFFFD, 0x42),
        (0xFFFE, 0x42),
        (0x4242, instructions::LDA_IMM),
        (0x4243, 0x84),
    ]);

    cpu.exe(Some(instructions::JSR_ABS_CCOST + instructions::LDA_IMM_CCOST));
    println!("{cpu}");
}
