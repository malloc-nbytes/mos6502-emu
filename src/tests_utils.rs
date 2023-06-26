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

#[derive(Debug)]
pub enum FlagAssertionError {
    Carry,
    Zero,
    InterruptsDisable,
    DecimalMode,
    Break,
    Unused,
    Overflow,
    Negative,
}

impl std::fmt::Display for FlagAssertionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlagAssertionError::Carry => write!(f, "Failed assertion for flag: CARRY"),
            FlagAssertionError::Zero => write!(f, "Failed assertion for flag: ZERO"),
            FlagAssertionError::InterruptsDisable => write!(f, "Failed assertion for flag: INTERRUPTS DISABLE"),
            FlagAssertionError::DecimalMode => write!(f, "Failed assertion for flag: DECIMAL MODE"),
            FlagAssertionError::Break => write!(f, "Failed assertion for flag: BREAK"),
            FlagAssertionError::Unused => write!(f, "Failed assertion for flag: UNUSED"),
            FlagAssertionError::Overflow => write!(f, "Failed assertion for flag: OVERFLOW"),
            FlagAssertionError::Negative => write!(f, "Failed assertion for flag: NEGATIVE"),
        }
    }
}

impl std::error::Error for FlagAssertionError {}

pub const PC_START: Word = 0xFFFC;

pub fn cpu_mem_set(instrs: Vec<(u16, u8)>) -> Mos6502 {
    let mut mem = Memory::new();
    for (addr, opcode) in instrs {
        *mem.at(addr as usize) = opcode;
    }
    let mut cpu = Mos6502::new(mem);
    cpu.reset(false);
    cpu
}

pub fn assert_all_status_flags_false_except(cpu: &Mos6502, excluded_flags: Vec<Mos6502Flags>) -> Result<(), FlagAssertionError> {
    let assert_flag_state = |flag: bool, current_flag: Mos6502Flags| {
        if !excluded_flags.contains(&current_flag) {
            if flag {
                Err(match current_flag {
                    Mos6502Flags::C => FlagAssertionError::Carry,
                    Mos6502Flags::Z => FlagAssertionError::Zero,
                    Mos6502Flags::I => FlagAssertionError::InterruptsDisable,
                    Mos6502Flags::D => FlagAssertionError::DecimalMode,
                    Mos6502Flags::B => FlagAssertionError::Break,
                    Mos6502Flags::U => FlagAssertionError::Unused,
                    Mos6502Flags::V => FlagAssertionError::Overflow,
                    Mos6502Flags::N => FlagAssertionError::Negative,
                })?;
            }
        }
        Ok(())
    };

    assert_flag_state(cpu.carry_flag(), Mos6502Flags::C)?;
    assert_flag_state(cpu.zero_flag(), Mos6502Flags::Z)?;
    assert_flag_state(cpu.interrupts_disable_flag(), Mos6502Flags::I)?;
    assert_flag_state(cpu.decimal_mode_flag(), Mos6502Flags::D)?;
    assert_flag_state(cpu.break_flag(), Mos6502Flags::B)?;
    assert_flag_state(cpu.unused_flag(), Mos6502Flags::U)?;
    assert_flag_state(cpu.overflow_flag(), Mos6502Flags::V)?;
    assert_flag_state(cpu.negative_flag(), Mos6502Flags::N)?;

    Ok(())
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
    match assert_all_status_flags_false_except(&cpu, tflags) {
        Ok(()) => println!("Flag assertion success"),
        Err(err) => eprintln!("Flag assertion error: {err}"),
    }
}

pub fn word_from_bytes(hi: Byte, lo: Byte) -> Word {
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
