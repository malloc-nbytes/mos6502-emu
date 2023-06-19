use crate::mos6502::{
    Mos6502,
    Mos6502Flags,
};
use crate::memory::Memory;

pub fn cpu_mem_set(instrs: Vec<(u16, u8)>) -> Mos6502 {
    let mut mem = Memory::new();
    for (addr, opcode) in instrs {
        *mem.at(addr as usize) = opcode;
    }
    let mut cpu = Mos6502::new(mem);
    cpu.reset(false);
    cpu
}

pub fn assert_all_status_flags_false_except(cpu: &Mos6502, excluded_flags: Vec<Mos6502Flags>) {
    let assert_flag_state = |flag: bool, flag_name: &str, current_flag: Mos6502Flags| {
        if !excluded_flags.contains(&current_flag) {
            assert!(!flag, "Failed assertion for flag: {}", flag_name);
        }
    };

    assert_flag_state(cpu.carry_flag(), "CARRY", Mos6502Flags::C);
    assert_flag_state(cpu.zero_flag(), "ZERO", Mos6502Flags::Z);
    assert_flag_state(cpu.interrupts_disable_flag(), "INTERRUPTS DISABLE", Mos6502Flags::I);
    assert_flag_state(cpu.decimal_mode_flag(), "DECIMAL MODE", Mos6502Flags::D);
    assert_flag_state(cpu.break_flag(), "BREAK", Mos6502Flags::B);
    assert_flag_state(cpu.unused_flag(), "UNUSED", Mos6502Flags::U);
    assert_flag_state(cpu.overflow_flag(), "OVERFLOW", Mos6502Flags::V);
    assert_flag_state(cpu.negative_flag(), "NEGATIVE", Mos6502Flags::N);
}
