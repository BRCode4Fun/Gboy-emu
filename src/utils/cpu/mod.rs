pub mod regs;
pub mod instr;

use super::{
    cpu::{
        regs::{
            Registers,
            CpuFlag::{Z, N, H, C},
        },
        instr::{
            InstructionType::{*}, // defines each cpu instruction
            {*} // defines the required operands
        },
    },
    cartridge::CartContext,
    memory::{*},
};

enum OperandType {
    Byte(u8),
    Word(u16),
    Null,
}
use OperandType::*;

pub struct Cpu {
    pub regs   : Registers,
    pub mmu    : Mmu,  // memory management unit
    pub ime    : bool, // interrupt master enable
    pub halted : bool,
}

impl Cpu {
    pub fn new(cartridge : &CartContext) -> Self {
        Cpu {
            regs   : Registers::new(),
            mmu    : Mmu::new(cartridge),
            ime    : true,
            halted : false,
        }
    }
    fn set_pc(&mut self, address : u16) { self.regs.pc = address; }
    fn inc_pc_by(&mut self, val : u16) { self.set_pc(self.regs.pc.wrapping_add(val)); }
    fn get_pc(&self) -> u16 { self.regs.pc }

    fn fetch_byte(&self, address : u16) -> OperandType { 
        OperandType::Byte(self.mmu.fetch_byte(address)) 
    }
    fn fetch_word(&self, address : u16) -> OperandType { 
        OperandType::Word(self.mmu.fetch_word(address)) 
    }
    fn set_byte(&mut self, addr : u16, val : u8) { self.mmu.set_byte(addr, val); }

    pub fn run(&mut self) -> Result<(), ()> {

        loop {
            if self.halted {
                break;
            }
            // fetch
            let opcode = if let Byte(value) = self.fetch_byte(self.get_pc()) {
                self.inc_pc_by(1); value
            } else {
                panic!("Cannot retrieve data from memory")
            };

            // decode
            let (instruction, _cycles) = if let Some((instr, cycles)) = match opcode {

                    0xcb => InstructionType::from_byte_prefixed(opcode),
                    _ => InstructionType::from_byte(opcode),

            } {(instr, cycles) } else { panic!("Opcode {:#02X} is not valid", opcode) };

            // execute
            match instruction {

                Load(dest, src) => {

                    let operand = match src {

                        Source::ByteConst => { // LD .., n
                            let imm_bconst = self.fetch_byte(self.get_pc());
                            self.inc_pc_by(1); imm_bconst
                        },
                        Source::WordConst => { // LD .., nn
                            let imm_wconst = self.fetch_word(self.get_pc());
                            self.inc_pc_by(2); imm_wconst
                        }
                        Source::ByteReg(reg8) => { // LD .., reg8
                            Byte(match reg8 {
                                Reg8::A => self.regs.a,
                                Reg8::B => self.regs.b,
                                Reg8::C => self.regs.c,
                                Reg8::D => self.regs.d,
                                Reg8::E => self.regs.e,
                                Reg8::H => self.regs.h,
                                Reg8::L => self.regs.l,
                            })
                        },
                        Source::WordReg(reg16) => { // LD .., reg16
                            Word(match reg16 {
                                Reg16::Bc => self.regs.bc(),
                                Reg16::De => self.regs.de(),
                                Reg16::Hl => self.regs.hl(),
                                other => panic!("Unexpected operand {:?}", other)
                            })
                        },
                        Source::Deref(Addr::WordReg(Reg16::Hl)) => { // LD .., (HL)
                            self.fetch_byte(self.regs.hl())
                        },
                        Source::Deref(Addr::WordConst) => {  // LD .., (nn)
                            let imm_addr = self.fetch_word(self.get_pc());
                            self.inc_pc_by(2);
                            if let Word(imm_addr) = imm_addr { self.fetch_byte(imm_addr) } 
                            else { Null }
                        }
                        _ => todo!()
                    };

                    if let OperandType::Byte(value) = operand {

                        let target : Option<&mut u8> = match dest {

                            Target::ByteReg(reg8) => { // LD reg8, ..
                                Some(match reg8 {
                                    Reg8::A => &mut self.regs.a,
                                    Reg8::B => &mut self.regs.b,
                                    Reg8::C => &mut self.regs.c,
                                    Reg8::D => &mut self.regs.d,
                                    Reg8::E => &mut self.regs.e,
                                    Reg8::H => &mut self.regs.h,
                                    Reg8::L => &mut self.regs.l,
                                })
                            },
                            Target::Deref(Addr::WordReg(Reg16::Hl)) => { // LD (HL), ..
                                self.set_byte(self.regs.hl(), value); 
                                None
                            },
                            Target::Deref(Addr::WordConst) => { // LD (nn), ..
                                let imm_addr = self.fetch_word(self.get_pc());
                                self.inc_pc_by(2); 
                                if let Word(imm_addr) = imm_addr { 
                                    self.set_byte(imm_addr, value); 
                                }; None
                            }
                            _ => todo!(),
                        };

                        if let Some(target) = target { *target = value; }

                    } else if let OperandType::Word(value) = operand {
                        
                        if let Target::WordReg(reg16) = dest { // LD reg16, ..
                            
                            match reg16 {
                                Reg16::Bc => self.regs.set_bc(value),
                                Reg16::De => self.regs.set_de(value),
                                Reg16::Hl => self.regs.set_hl(value),
                                Reg16::Sp => {
                                    self.regs.sp = value;
                                },
                                other => panic!("Unexpected target {:?}", other)
                            }
                        }
                    }
                },
                Halt => {
                    self.halted = true;
                },
                _other => todo!(),
            }
        }

        Ok(())
    }
    pub fn reset(&mut self) {
        self.regs.pc = 0x100;
        self.halted = false;
    }

    #[cfg(test)]
    pub fn load_rom(&mut self, arr : Vec<u8>) {
        
        let mut addr : u16 = 0x100;

        for opcode in arr.into_iter() {
            self.set_byte(addr, opcode);
            addr += 1;
        }
        self.reset(); // resets cpu state
    }
}

#[cfg(test)]
mod test {
    use crate::{
        cpu::Cpu,
        cartridge::CartContext,
    };
    #[test]
    fn exec_instr() {

        let cart = CartContext::new();
        let mut cpu = Cpu::new(&cart);

        cpu.load_rom(vec![
            0x3E, // LOAD A, n
            0x12, // byteconst 18
            0x47, // LOAD B, A
            0x48, // LOAD C, B
            0x51, // LOAD D, C
            0x5A, // LOAD E, D
            0x63, // LOAD H, E
            0x6C, // LOAD L, H
            0x76, // HALT
        ]);
        cpu.run();

        assert_eq!(cpu.regs.a, 18);
        assert_eq!(cpu.regs.l, 18);

        cpu.load_rom(vec![
            0x21,       // LOAD HL, nn
            0x64, 0x00, // wordconst 100
            0x36,       // LD (HL), n
            0x30,       // byteconst 48
            0x7E,       // LD A, (HL)
            0x76,       // HALT
        ]);
        cpu.run();

        assert_eq!(cpu.regs.hl(), 100);
        assert_eq!(cpu.regs.a, 48);

        cpu.load_rom(vec![
            0x3E,       // LOAD A, n
            0x22,       // byteconst 34
            0xEA,       // LD (nn), A
            0x64, 0x00, // wordconst 100
            0x3E,       // LOAD A, n - maybe XOR A, A ?
            0x38,       // byteconst 56
            0xFA,       // LD A, (nn)
            0x64, 0x00, // wordconst 100
            0x76,       // HALT
        ]);
        cpu.run();

        assert_eq!(cpu.regs.a, 34);
    }
}