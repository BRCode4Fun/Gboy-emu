pub mod timer;

use super::cartridge::CartContext;

pub trait Memory {
    fn fetch_byte(&self, addr : u16) -> u8;

    fn set_byte(&mut self, addr : u16, value : u8);

    fn fetch_word(&self, addr : u16) -> u16 {
        u16::from(self.fetch_byte(addr)) | (u16::from(self.fetch_byte(addr + 1)) << 8)
    }
     fn set_word(&mut self, addr : u16, value : u16) {
        self.set_byte(addr, (value & 0xFF) as u8);
        self.set_byte(addr + 1, (value >> 8) as u8)
    }
}

// 0x0000 - 0x3FFF: ROM Bank 00 (from Cartridge)
// 0x4000 - 0x7FFF: ROM Bank 01 - Switchable (from Cartridge)
// 0x8000 - 0x9FFF: Video RAM (VRAM)
// 0xA000 - 0xBFFF: External RAM
// 0xC000 - 0xCFFF: Work RAM (WRAM)
// 0xD000 - 0xDFFF: Work RAM (WRAM)
// 0xE000 - 0xFDFF: Mirror of 0xC000-0xDDFF (ECHO RAM)
// 0xFE00 - 0xFEF9: Sprite Attribute table (OAM)
// 0xFEA0 - 0xFEFF: Not Usable
// 0xFF00 - 0xFF7F: I/O Registers
// 0xFF80 - 0xFFFE: High RAM (HRAM)
// 0xFFFF - 0xFFFF: Interrupt ENable Register (IE)

const HRAM_SIZE : usize = 0x7F;
const WRAM_SIZE : usize = 0x8000;

pub struct Mmu {
    // pub apu : Option<Apu>,
    // pub gpu : Gpu,
    // pub joypad : Joypad,
    cartridge : CartContext,
    hram      : [u8; HRAM_SIZE],
    wram      : [u8; WRAM_SIZE],
    wram_bank : usize,
}

impl Mmu {
    pub fn new(cartridge : &CartContext) -> Self {
        Mmu {
            cartridge : CartContext::default(), // todo!(update to set from args)
            hram      : [0; HRAM_SIZE],
            wram      : [0; WRAM_SIZE],
            wram_bank : 0,
        }
    }
}

impl Memory for Mmu {
     fn fetch_byte(&self, addr : u16) -> u8 {
        match addr {

          0x0000..= 0x7fff  => return self.cartridge.rom_data[addr as usize],
          0xc000..= 0xdfff  => return self.wram[(addr & 0x1fff) as usize],
          0xFF80..= 0xFFFE  => return self.hram[(addr - 0xFF80) as usize],
          _ => panic!("Error Invalid Address")

      };
    }

     fn set_byte(&mut self, addr : u16, value : u8) {

        match addr {

          0x0000..= 0x7fff  => self.cartridge.rom_data[addr as  usize] = value,
          0xC000..= 0xDFFF  => self.wram[(addr & 0x1FFF) as usize]     = value,
          0xFF80..= 0xFFFE  => self.hram[(addr - 0xFF80) as usize]     = value,
          _ => panic!("Error  Invalid Address")

      };
    }
}
