mod mos6502;
mod memory;

fn main() {
    let mut cpu = mos6502::Mos6502::new();
    let mut mem = memory::Memory::new();

    println!("{cpu}");

}
