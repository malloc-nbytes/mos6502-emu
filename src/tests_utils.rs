#![allow(dead_code)]

use crate::mos6502::{
    Mos6502,
    Mos6502Flags,
};

use crate::memory::{
    Memory,
    Byte,
    Word,
};

pub enum Registers {
    A,
    X,
    Y,
}

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

fn assert_true_flags(cpu: &Mos6502, tflags: &Vec<Mos6502Flags>) {
    tflags
        .iter()
        .for_each(|flag| {
            match flag {
                Mos6502Flags::C => assert!(cpu.carry_flag()),
                Mos6502Flags::Z => assert!(cpu.zero_flag()),
                Mos6502Flags::I => assert!(cpu.interrupts_disable_flag()),
                Mos6502Flags::D => assert!(cpu.decimal_mode_flag()),
                Mos6502Flags::B => assert!(cpu.break_flag()),
                Mos6502Flags::U => assert!(cpu.unused_flag()),
                Mos6502Flags::V => assert!(cpu.overflow_flag()),
                Mos6502Flags::N => assert!(cpu.negative_flag()),
            }
        });
}

fn assert_register_has_value(cpu: &Mos6502, register: Registers, val: Byte) {
    match register {
        Registers::A => assert_eq!(cpu.get_accumulator(), val),
        Registers::X => assert_eq!(cpu.get_xreg(), val),
        Registers::Y => assert_eq!(cpu.get_yreg(), val),
    }
}

fn perform_ld_asserts(
    cpu: &Mos6502,
    test_register: Registers,
    ccost: u32,
    val: Byte,
    tflags: Vec<Mos6502Flags>)
{
    assert_register_has_value(&cpu, test_register, val);
    assert_eq!(cpu.get_cycles(), ccost);
    assert_true_flags(&cpu, &tflags);
    assert_all_status_flags_false_except(&cpu, tflags);
}

pub fn word_from_bytes(lo: Byte, hi: Byte) -> Word {
    (Word::from(hi) << 8) | Word::from(lo)
}

pub fn word_from_byte_addition(b1: Byte, b2: Byte) -> Word {
    (b1 as Word + b2 as Word) % 256
}

pub fn ld_into_reg<F>(
    mem: Vec<(Word, Byte)>,
    val: Byte,
    ccost: u32,
    test_register: Registers,
    tflags: Vec<Mos6502Flags>,
    mod_before_exe: Option<F>)
where F: FnOnce(&mut Mos6502)
{
    let mut cpu = cpu_mem_set(mem);

    if let Some(f) = mod_before_exe {
        f(&mut cpu);
    }

    cpu.exe(Some(ccost));
    perform_ld_asserts(&cpu, test_register, ccost, val, tflags);
}
