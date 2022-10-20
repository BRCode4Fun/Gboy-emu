#[derive(Copy, Clone)]
pub enum CpuFlag {
    Z = 0b1000_0000, // zero flag
    N = 0b0100_0000, // subtraction flag
    H = 0b0010_0000, // half-carry flag
    C = 0b0001_0000, // carry flag
}

#[derive(Copy, Clone)]
pub struct Registers {
    pub a   : u8, // accumulator
    pub b   : u8,
    pub c   : u8,
    pub d   : u8,
    pub e   : u8,
        f   : u8,  // flags register (not directly mutable)
    pub h   : u8,
    pub l   : u8,
    pub pc  : u16, // program counter
    pub sp  : u16  // stack pointer
}

impl Registers {
    pub fn new() -> Registers {
        // initializing registers based on DMG CPU power-up sequence
        // https://gbdev.io/pandocs/Power_Up_Sequence.html
        Registers {
            a  : 0x01,
            b  : 0x00,
            c  : 0x13,
            d  : 0x00,
            e  : 0xD8,
            f  : 0xB0, // set C, H and Z flags
            h  : 0x01,
            l  : 0x4D,
            pc : 0x0100,
            sp : 0xFFFE,
        }
    }
    fn get_wide_reg(&self, high : u8, low : u8) -> u16 {
        ((high as u16) << 8) | (low as u16)
    }
    // getters
    pub fn af(&self) -> u16 { self.get_wide_reg(self.a, self.f) }
    pub fn bc(&self) -> u16 { self.get_wide_reg(self.b, self.c) }
    pub fn de(&self) -> u16 { self.get_wide_reg(self.d, self.e) }
    pub fn hl(&self) -> u16 { self.get_wide_reg(self.h, self.l) }

    pub fn get_flag(&self, flag : CpuFlag) -> bool { (self.f & (flag as u8)) > 0 }

    // setters
    pub fn set_af(&mut self, value : u16) {
        self.a = (value >> 8)     as u8;
        self.f = (value & 0x00F0) as u8;
    }
    pub fn set_bc(&mut self, value : u16) { 
        self.b = (value >> 8)     as u8;
        self.c = (value & 0x00FF) as u8;    
    }
    pub fn set_de(&mut self, value : u16) { 
        self.d = (value >> 8)     as u8;
        self.e = (value & 0x00FF) as u8;
    }
    pub fn set_hl(&mut self, value : u16) { 
        self.h = (value >> 8)     as u8;
        self.l = (value & 0x00FF) as u8;
    }
    pub fn set_flag(&mut self, flag : CpuFlag, set : bool) { // set or reset the cpu flag
        
        let mask = flag as u8;

        match set {
            true  => self.f |=  mask,
            false => self.f &= !mask
        }
        self.f &= 0xF0;
    }
}

#[cfg(test)]
mod test {
    use super::Registers;
    use super::CpuFlag::{Z, N, H, C};

    #[test]
    fn wide_registers() {
        let mut regs = Registers::new();

        regs.a = 0x12;
        regs.set_flag(Z, false);
        regs.set_flag(N, false);
        regs.set_flag(C, false);
        regs.set_flag(H, true);
        assert_eq!(regs.af(), 0x1220);
        regs.set_af(0x1111);
        assert_eq!(regs.af(), 0x1110);

        regs.b = 0x34;
        regs.c = 0x45;
        assert_eq!(regs.bc(), 0x3445);
        regs.set_bc(0x1111);
        assert_eq!(regs.bc(), 0x1111);

        regs.d = 0x56;
        regs.e = 0x67;
        assert_eq!(regs.de(), 0x5667);
        regs.set_de(0x1111);
        assert_eq!(regs.de(), 0x1111);

        regs.h = 0x78;
        regs.l = 0x89;
        assert_eq!(regs.hl(), 0x7889);
        regs.set_hl(0x1111);
        assert_eq!(regs.hl(), 0x1111);
    }

    #[test]
    fn flags() {
        let mut regs = Registers::new();

        // check if only the most-significant nibble of the flag register is set
        assert_eq!(regs.f & 0x0F, 0);

        regs.set_flag(Z, false);
        regs.set_flag(N, false);
        regs.set_flag(H, false);
        regs.set_flag(C, false);

        let flags = [Z, N, H, C];

        // check if it can safely set and reset the cpu flags
        for i in 0..4 {
            let mask = flags[i];
            assert_eq!(regs.get_flag(mask), false);
            regs.set_flag(mask, true);
            assert_eq!(regs.get_flag(mask), true);
            regs.set_flag(mask, false);
            assert_eq!(regs.get_flag(mask), false);
        }
    }
}