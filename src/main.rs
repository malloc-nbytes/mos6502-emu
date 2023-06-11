#![warn(clippy::pedantic)]

mod instructions;
mod mos6502;
mod memory;

#[allow(dead_code)]
#[allow(unused_mut)]
#[allow(unused_variables)]
fn main() {
    let mut mem = memory::Memory::new();

    // inline
    *mem.at(0xFFFC) = instructions::JSR_ABS;
    *mem.at(0xFFFD) = 0x42;
    *mem.at(0xFFFE) = 0x42;
    *mem.at(0x4242) = instructions::LDA_IMM;
    *mem.at(0x4243) = 0x84;
    // end inline

    let mut cpu = mos6502::Mos6502::new(Some(instructions::JSR_ABS_CCOST), mem);

    cpu.exe();

    println!("{cpu}");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        unimplemented!();
    }
}
