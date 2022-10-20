pub mod regs;
pub mod instr;

use super::{
    cpu::{
        regs::{
            Registers,
            CpuFlag::{Z, N, H, C},
        },
        instr::{InstructionType},
    },
    cartridge::CartContext,
    memory::Mmu,
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

            } { (instr, cycles) } else { panic!("Opcode {:#02X} is not valid", opcode) };

            // execute
            /*match instruction {

                Load(Source, Destination) => {
                    let s = match Source {
                        Reg::A => cpu.a,
                        Reg::B => cpu.b,
                    }
                    let d = match Destination {

                    }
                    *d = s;
                }
                Nop => {}
            }*/
        }

        Ok(())
    }
}