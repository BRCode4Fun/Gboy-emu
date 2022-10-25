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
    fn inc_pc(&mut self) { self.set_pc(self.regs.pc.wrapping_add(1)); }
    fn get_pc(&self) -> u16 { self.regs.pc }

    fn fetch_byte(&self, address : u16) -> u8  { self.mmu.fetch_byte(address) }
    fn fetch_word(&self, address : u16) -> u16 { self.mmu.fetch_word(address) }

    pub fn set_byte(&mut self, addr : u16, val : u8) { self.mmu.set_byte(addr, val); }

    pub fn run(&mut self) -> Result<(), ()> {

        loop {
            if self.halted {
                break;
            }
            // fetch
            let opcode = self.fetch_byte(self.get_pc());
            self.inc_pc();

            // decode
            let (instruction, _cycles) = if let Some((instr, cycles)) = match opcode {

                    0xcb => InstructionType::from_byte_prefixed(opcode),
                    _ => InstructionType::from_byte(opcode),

            } {(instr, cycles) } else { panic!("Opcode {:#02X} is not valid", opcode) };

            // execute
            match instruction {

                Load(dest, src) => {
                    
                    let value : u8 = match src {
                        Source::ByteConst => {
                            let c = self.fetch_byte(self.get_pc());
                            self.inc_pc();
                            c
                        },
                        Source::ByteReg(reg8) => match reg8 {
                            Reg8::A => self.regs.a,
                            Reg8::B => self.regs.b,
                            Reg8::C => self.regs.c,
                            Reg8::D => self.regs.d,
                            Reg8::E => self.regs.e,
                            Reg8::H => self.regs.h,
                            Reg8::L => self.regs.l,
                        },
                        _ => todo!(),
                    };

                    let target : &mut u8 = match dest {
                        
                        Target::ByteReg(reg8) => match reg8 {
                                Reg8::A => &mut self.regs.a,
                                Reg8::B => &mut self.regs.b,
                                Reg8::C => &mut self.regs.c,
                                Reg8::D => &mut self.regs.d,
                                Reg8::E => &mut self.regs.e,
                                Reg8::H => &mut self.regs.h,
                                Reg8::L => &mut self.regs.l,
                        },
                        _ => todo!(),
                    };
                    *target = value;
                },
                Halt => {
                    self.halted = true;
                },
                _other => todo!(),
            }
        }

        Ok(())
    }
}