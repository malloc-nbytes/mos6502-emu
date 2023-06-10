 #![warn(clippy::pedantic)]

mod instructions;
mod mos6502;
mod memory;

#[allow(dead_code)]
#[allow(unused_mut)]
#[allow(unused_variables)]
fn main() {
    let mut cpu = mos6502::Mos6502::new(None);
    let mut mem = memory::Memory::new();
    cpu.reset(Some(instructions::LDA_ZP_CCOST), &mut mem);

    // inline
    *mem.at(0xFFFC) = instructions::LDA_ZP;
    *mem.at(0xFFFD) = 0x42;
    *mem.at(0x0042) = 0x84;
    // end inline

    cpu.exe(&mut mem);

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
