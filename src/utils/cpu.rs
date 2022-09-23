pub enum Flag {
    Z,
    N,
    H,
    C,
}

pub struct Cpu
{
    pub a      : u8,
    pub f      : u8,
    pub b      : u8,
    pub c      : u8,
    pub d      : u8,
    pub e      : u8,
    pub h      : u8,
    pub l      : u8,
    pub sp     : u16,
    pub pc     : u16,
    pub mie    : bool,
    pub halted : bool,

}

impl Cpu {

    pub fn get_a  (&self) -> u8 { self.a }
    pub fn get_f  (&self) -> u8 { self.f }
    pub fn get_b  (&self) -> u8 { self.b }
    pub fn get_c  (&self) -> u8 { self.c }
    pub fn get_d  (&self) -> u8 { self.d }
    pub fn get_e  (&self) -> u8 { self.e }
    pub fn get_h  (&self) -> u8 { self.h }
    pub fn get_l  (&self) -> u8 { self.l }


    pub fn get_af  (&self)  -> u16   { Cpu::get_u16( &self.a, &self.f) }
    pub fn get_bc  (&self)  -> u16   { Cpu::get_u16( &self.b, &self.c) }
    pub fn get_de  (&self)  -> u16   { Cpu::get_u16( &self.d, &self.e) }
    pub fn get_hl  (&self)  -> u16   { Cpu::get_u16( &self.h, &self.l) }
    pub fn get_sp  (&self)  -> u16   { self.sp  }
    pub fn get_pc  (&self)  -> u16   { self.pc  }
    pub fn get_mie (&self) ->  bool  { self.mie }

    fn get_u16 (h: &u8 ,l: &u8) -> u16
    {
        ((*h as u16) << 8) | *l as u16
    }


    pub fn set_a (&mut self, n: u8){ self.a = n; }
    pub fn set_f (&mut self, n: u8){ self.f = n; }
    pub fn set_b (&mut self, n: u8){ self.b = n; }
    pub fn set_c (&mut self, n: u8){ self.c = n; }
    pub fn set_d (&mut self, n: u8){ self.d = n; }
    pub fn set_e (&mut self, n: u8){ self.e = n; }
    pub fn set_h (&mut self, n: u8){ self.h = n; }
    pub fn set_l (&mut self, n: u8){ self.l = n; }


    pub fn set_af (&mut self, n : u16) { Cpu::set_u16(&mut self.a , &mut self.f, n); }
    pub fn set_bc (&mut self, n : u16) { Cpu::set_u16(&mut self.b , &mut self.c, n); }
    pub fn set_de (&mut self, n : u16) { Cpu::set_u16(&mut self.d , &mut self.e, n); }
    pub fn set_hl (&mut self, n : u16) { Cpu::set_u16(&mut self.h , &mut self.l, n); }
    pub fn set_sp (&mut self, n : u16) { self.sp = n; }
    pub fn set_pc (&mut self, n : u16) { self.pc = n; }

    fn set_u16 (high: &mut u8, low: &mut u8, n: u16)
    {
        *high  = (n >> 8) as u8;
        *low   = n as u8;
    }
    pub fn set_mie (&mut self, b : bool){  self.mie = b; }


    pub fn get_flags (&self , f : Flag) -> bool
    {
        match  f {
            Flag::Z => self.f & 0b1000_0000 > 0,
            Flag::N => self.f & 0b0100_0000 > 0,
            Flag::H => self.f & 0b0010_0000 > 0,
            Flag::C => self.f & 0b0001_0000 > 0
        }


    }

    pub fn set_flag (&mut self, f: Flag){
        match f
        {
            Flag::Z => self.set_f(self.get_f() | 0b10000000),
            Flag::N => self.set_f(self.get_f() | 0b01000000),
            Flag::H => self.set_f(self.get_f() | 0b00100000),
            Flag::C => self.set_f(self.get_f() | 0b00010000)
        }
    }

    pub fn clear_flag (&mut self, f: Flag){

        match f
        {
            Flag::Z => self.set_f(self.get_f() & 0b01111111),
            Flag::N => self.set_f(self.get_f() & 0b10111111),
            Flag::H => self.set_f(self.get_f() & 0b11011111),
            Flag::C => self.set_f(self.get_f() & 0b11101111)
        }
    }
}

pub fn nop(cpu : &mut Cpu) {}

//0x01
pub fn ld_bc_d16(cpu : &mut Cpu, h : u8, l: u8){
    cpu.set_b(h);
    cpu.set_c(l);
}

// 0x03
pub fn inc_bc(cpu : &mut Cpu ){
    cpu.set_bc(cpu.get_bc() + 0b1);
}

// 0x04
pub fn inc_b(cpu :&mut Cpu ){
    if cpu.get_b() == 0x0f {
        cpu.set_flag(Flag::H);
    }else{
        cpu.clear_flag(Flag::H);
    }

    cpu.set_b(cpu.get_b().wrapping_add(1));

    if cpu.get_b() == 0 {
        cpu.set_flag(Flag::Z);
    } else {
        cpu.clear_flag(Flag::Z);
    } //zero flag
    cpu.clear_flag(Flag::N);
}

//0x05
pub fn dec_b(cpu : &mut Cpu) {

    if cpu.get_b() & 0x0f > 0 {
        cpu.clear_flag(Flag::H);
    }else{
        cpu.set_flag(Flag::H);
    }

    cpu.set_b(cpu.get_b().wrapping_sub(1));
    cpu.set_flag(Flag::N);
}

// 0x06
pub fn  ld_b_d8(cpu : &mut Cpu, n : u8){
    cpu.set_b(n);
}

// 0x07
pub fn  rlca(cpu : &mut Cpu){
    todo!("Not implemented");
}

// 0x08
pub fn ld_addr16_sp(cpu : &mut Cpu){
    todo!("Not implemented");
}

// 0x09
pub fn add_hl_bc(cpu : &mut Cpu){

    match cpu.get_hl().checked_add(cpu.get_bc()) {
        None => {
            cpu.set_flag(Flag::C);
        }
        Some(_) => {
            cpu.clear_flag(Flag::C);
        }
    }
    if cpu.get_h() & 0x0f + ((cpu.get_bc() >> 8) as u8 & 0x0f) > 0x0f {
        cpu.set_flag(Flag::H);
    } else {
        cpu.clear_flag(Flag::H);
    }
    cpu.set_hl(cpu.get_hl().wrapping_add(cpu.get_bc()));
    cpu.clear_flag(Flag::N);
}

// 0x0A

pub fn  ld_a_addr_bc(cpu : &mut Cpu){
    todo!("Not implemented");
}
// 0x0B
pub fn  dec_bc(cpu : &mut Cpu){
    todo!("Not implemented");
}
// 0x0C
pub fn  inc_c(cpu : &mut Cpu){
    todo!("Not implemented");
}

// 0x0D
pub fn  dec_c(cpu : &mut Cpu){
    todo!("Not implemented");
}

//0x0e
pub fn  ld_c_u8(cpu : &mut Cpu, n : u8){
    cpu.set_c(n);
}

// 0x0F
pub fn  RRCA(cpu : &mut Cpu){
    todo!("Not implemented");
}

// 0x10

pub fn  STOP(cpu : &mut Cpu){
    todo!("Not implemented");
}

//0x11
pub fn ld_de_u16(cpu : &mut Cpu, h : u8, l: u8){
    cpu.set_d(h);
    cpu.set_e(l);
}

// 0x12
pub fn ld_de_addr_a(cpu : &mut Cpu){
    todo!("Not implemented");
}

// 0x13

pub fn inc_de(cpu : &mut Cpu){
    cpu.set_de(cpu.get_de() + 1);
}

// 0x14

pub fn inc_d(cpu : &mut Cpu){
    todo!("Not implemented");
}

// 0x15
pub fn dec_d(cpu : &mut Cpu){
    todo!("Not implemented");
}

// 0x16
pub fn ld_d_d8(cpu : &mut Cpu , n: u8){
    cpu.set_d(n);
}

// 0x17
pub fn rla(cpu : &mut Cpu){
    todo!("Not implemented");
}

// 0x18
pub fn jr_r8(cpu : &mut Cpu){
    todo!("Not implemented");
}

// 0x19
pub fn add_hl_de(cpu : &mut Cpu){
    todo!("Not implemented");
}

// 0x1A
pub fn ld_a_addr_de(cpu : &mut Cpu){
    todo!("Not implemented");
}

// 0x1B
pub fn dec_de(cpu : &mut Cpu){
    cpu.set_de(cpu.get_de() + 1);
}

// 0x1C
pub fn inc_e(cpu : &mut Cpu){
    todo!("Not implemented");
}
// 0x1D
pub fn dec_e(cpu : &mut Cpu){
    todo!("Not implemented");
}
// 0x1E
pub fn ld_e_d8(cpu : &mut Cpu , n : u8){
     cpu.set_e(n);
}
// 0x1F
pub fn RRA(cpu : &mut Cpu){
    todo!("Not implemented");
}
// 0x20
pub fn jr_nz_r8(cpu : &mut Cpu , r8 : u8){
    todo!("Not implemented");
}
// 0x21
pub fn ld_hl_u16(cpu : &mut Cpu, h : u8, l: u8){
    cpu.set_h(h);
    cpu.set_l(l);
}
// 0x22
pub fn ld_addr_hl_plus_A(cpu : &mut Cpu){
    todo!("Not implemented");
}
// 0x23
pub fn inc_hl(cpu : &mut Cpu){
    todo!("Not implemented");
}
// 0x24
pub fn inc_h(cpu : &mut Cpu){
    todo!("Not implemented");
}
// 0x25
pub fn dec_h(cpu : &mut Cpu){
    todo!("Not implemented");
}
// 0x26
pub fn  ld_h_u8(cpu : &mut Cpu, n : u8){
    cpu.set_h(n);
}
// 0x27
pub fn  aa(cpu : &mut Cpu){
    todo!("Not implemented");
}
// 0x28
pub fn  jr_z_s8(cpu : &mut Cpu ,  s8 : i8){
    todo!("Not implemented");
}
// 0x29
pub fn  add_hl_hl(cpu : &mut Cpu){
    todo!("Not implemented");
}
// 0x2A
pub fn  ld_a_addr_de_plus(cpu : &mut Cpu){
    todo!("Not implemented");
}
// 0x2B
pub fn  dec_hl(cpu : &mut Cpu){
    cpu.set_hl(cpu.get_hl() - 1);
}
// 0x2C
pub fn  inc_l(cpu : &mut Cpu){
    todo!("Not implemented");
}
// 0x2D
pub fn  dec_l(cpu : &mut Cpu){
    todo!("Not implemented");
}
// 0x2E
pub fn  ld_l_u8(cpu : &mut Cpu, n : u8){
    cpu.set_l(n);
}
// 0x2F
pub fn  CPL(cpu : &mut Cpu){
    todo!("Not implemented");
}
// 0x30
pub fn jr_nc_r8(cpu : &mut Cpu , r8 : u8){
    todo!("Not implemented");
}
// 0x31
pub fn ld_sp_d16(cpu : &mut Cpu, n : u16){
  todo!("Not implemented");
}
// 0x32
pub fn ld_addr_hl_minus_A(cpu : &mut Cpu){
    todo!("Not implemented");
}
// 0x33
pub fn inc_sp(cpu : &mut Cpu){
    todo!("Not implemented");
}
// 0x34
pub fn inc_hl_addr(cpu : &mut Cpu){
    todo!("Not implemented");
}
// 0x35
pub fn dec_hl_addr(cpu : &mut Cpu){
    todo!("Not implemented");
}

// 0x36
pub fn  ld_hl_d8(cpu : &mut Cpu, n : u8){
    todo!("Not implemented");
}
// 0x37
pub fn  scf(cpu : &mut Cpu){
    todo!("Not implemented");
}
// 0x38
pub fn  jr_c_r8(cpu : &mut Cpu,  r8 : i8){
    todo!("Not implemented");
}
// 0x39
pub fn  add_hl_sp(cpu : &mut Cpu){
    todo!("Not implemented");
}
// 0x3A
pub fn  ld_a_addr_hl_minus(cpu : &mut Cpu){
    todo!("Not implemented");
}
// 0x3B
pub fn  dec_sp(cpu : &mut Cpu){
    todo!("Not implemented");
}
// 0x3C
pub fn  inc_a(cpu : &mut Cpu){
    todo!("Not implemented");
}
// 0x3D
pub fn  dec_a(cpu : &mut Cpu){
    todo!("Not implemented");
}
// 0x3E
pub fn  ld_a_d8(cpu : &mut Cpu, n : u8){
    cpu.set_a(n);
}
// 0x3F
pub fn  CCF(cpu : &mut Cpu, n : u8){
    cpu.set_a(n);
}
// 0x40
pub fn ld_b_b(cpu : &mut Cpu){
    cpu.set_b(cpu.get_b());
}
// 0x41
pub fn ld_b_c(cpu : &mut Cpu){
    cpu.set_b(cpu.get_c());
}
// 0x42
pub fn ld_b_d(cpu : &mut Cpu){
    cpu.set_b(cpu.get_d());
}
// 0x43
pub fn ld_b_e(cpu : &mut Cpu){
    cpu.set_b(cpu.get_e());
}
// 0x44
pub fn ld_b_h(cpu : &mut Cpu){
    cpu.set_b(cpu.get_h());
}
// 0x45
pub fn ld_b_l(cpu : &mut Cpu){
    cpu.set_b(cpu.get_l());
}

// 0x46
pub fn ld_b_a(cpu : &mut Cpu){
    cpu.set_b(cpu.get_a());
}

// 0x48
pub fn ld_c_b(cpu : &mut Cpu){
    cpu.set_c(cpu.get_b());
}
// 0x49
pub fn ld_c_c(cpu : &mut Cpu){
    cpu.set_c(cpu.get_c());
}
// 0x4A
pub fn ld_c_d(cpu : &mut Cpu){
    cpu.set_c(cpu.get_d());
}
// 0x4B
pub fn ld_c_e(cpu : &mut Cpu){
    cpu.set_c(cpu.get_e());
}
// 0x4C
pub fn ld_c_h(cpu : &mut Cpu){
    cpu.set_c(cpu.get_h());
}
// 0x4D
pub fn ld_c_l(cpu : &mut Cpu){
    cpu.set_c(cpu.get_l());
}

// 0x4E
pub fn ld_c_addr_hl(cpu : &mut Cpu){
    todo!("Not implemented");
}

// 0x4F
pub fn ld_c_a(cpu : &mut Cpu){
    todo!("Not implemented");
}

// 0x50
pub fn ld_d_b(cpu: &mut Cpu) {
    cpu.set_d(cpu.get_b());
}
// 0x51
pub fn ld_d_c(cpu: &mut Cpu) {
    cpu.set_d(cpu.get_c());
}
// 0x52
pub fn ld_d_d(cpu: &mut Cpu) {
    cpu.set_d(cpu.get_d());
}
// 0x53
pub fn ld_d_e(cpu: &mut Cpu) {
    cpu.set_d(cpu.get_e());
}
// 0x54
pub fn ld_d_h(cpu: &mut Cpu) {
    cpu.set_d(cpu.get_h());
}
// 0x55
pub fn ld_d_l(cpu: &mut Cpu) {
    cpu.set_d(cpu.get_l());
}
// 0x56
pub fn ld_d_hlp(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x57
pub fn ld_d_a(cpu: &mut Cpu) {
    cpu.set_d(cpu.get_a());
}

// 0x58
pub fn ld_e_b(cpu: &mut Cpu) {
    cpu.set_e(cpu.get_b());
}
// 0x59
pub fn ld_e_c(cpu: &mut Cpu) {
    cpu.set_e(cpu.get_c());
}
// 0x5A
pub fn ld_e_d(cpu: &mut Cpu) {
    cpu.set_e(cpu.get_d());
}
// 0x5B
pub fn ld_e_e(cpu: &mut Cpu) {
    cpu.set_e(cpu.get_e());
}
// 0x5C
pub fn ld_e_h(cpu: &mut Cpu) {
    cpu.set_e(cpu.get_h());
}
// 0x5D
pub fn ld_e_l(cpu: &mut Cpu) {
    cpu.set_e(cpu.get_l());
}
// 0x5E
pub fn ld_e_hlp(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x5F
pub fn ld_e_a(cpu: &mut Cpu) {
    cpu.set_e(cpu.get_a());
}

// 0x60
pub fn ld_h_b(cpu: &mut Cpu) {
    cpu.set_h(cpu.get_b());
}
// 0x61
pub fn ld_h_c(cpu: &mut Cpu) {
    cpu.set_h(cpu.get_c());
}
// 0x62
pub fn ld_h_d(cpu: &mut Cpu) {
    cpu.set_h(cpu.get_d());
}
// 0x63
pub fn ld_h_e(cpu: &mut Cpu) {
    cpu.set_h(cpu.get_e());
}
// 0x64
pub fn ld_h_h(cpu: &mut Cpu) {
    cpu.set_h(cpu.get_h());
}
// 0x65
pub fn ld_h_l(cpu: &mut Cpu) {
    cpu.set_h(cpu.get_l());
}
// 0x66
pub fn ld_h_hlp(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x67
pub fn ld_h_a(cpu: &mut Cpu) {
    cpu.set_h(cpu.get_a());
}

// 0x68
pub fn ld_l_b(cpu: &mut Cpu) {
    cpu.set_l(cpu.get_b());
}
// 0x69
pub fn ld_l_c(cpu: &mut Cpu) {
    cpu.set_l(cpu.get_c());
}
// 0x6A
pub fn ld_l_d(cpu: &mut Cpu) {
    cpu.set_l(cpu.get_d());
}
// 0x6B
pub fn ld_l_e(cpu: &mut Cpu) {
    cpu.set_l(cpu.get_e());
}
// 0x6C
pub fn ld_l_h(cpu: &mut Cpu) {
    cpu.set_l(cpu.get_h());
}
// 0x6D
pub fn ld_l_l(cpu: &mut Cpu) {
    cpu.set_l(cpu.get_l());
}
// 0x6E
pub fn ld_l_hlp(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x6F
pub fn ld_l_a(cpu: &mut Cpu) {
    cpu.set_l(cpu.get_a());
}


// 0x70
pub fn ld_addr_hl_b(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x71
pub fn ld_addr_hl_c(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x72
pub fn ld_addr_hl_d(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x73
pub fn ld_addr_hl_e(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x74
pub fn ld_addr_hl_h(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x75
pub fn ld_addr_hl_l(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x76
pub fn halt(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x77
pub fn ld_addr_hl_a(cpu: &mut Cpu) {
    todo!("Not implemented");
}

// 0x78
pub fn ld_a_b(cpu: &mut Cpu) {
    cpu.set_a(cpu.get_b());
}
// 0x79
pub fn ld_a_c(cpu: &mut Cpu) {
    cpu.set_a(cpu.get_c());
}
// 0x7A
pub fn ld_a_d(cpu: &mut Cpu) {
    cpu.set_a(cpu.get_d());
}
// 0x7B
pub fn ld_a_e(cpu: &mut Cpu) {
    cpu.set_a(cpu.get_e());
}
// 0x7C
pub fn ld_a_h(cpu: &mut Cpu) {
    cpu.set_l(cpu.get_h());
}
// 0x7D
pub fn ld_a_l(cpu: &mut Cpu) {
    cpu.set_l(cpu.get_l());
}
// 0x7E
pub fn ld_a_addr_hl(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x7F
pub fn ld_a_a(cpu: &mut Cpu) {
    cpu.set_a(cpu.get_a());
}
// 0x80
pub fn add_a_b(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x81
pub fn add_a_c(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x82
pub fn add_a_d(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x83
pub fn add_a_e(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x84
pub fn add_a_h(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x85
pub fn add_a_l(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x86
pub fn add_a_addr_hl(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x87
pub fn add_a_a(cpu: &mut Cpu) {
    todo!("Not implemented");
}

// 0x88
pub fn addc_a_b(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x89
pub fn addc_a_c(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x8A
pub fn addc_a_d(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x8B
pub fn addc_a_e(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x8C
pub fn addc_a_h(cpu: &mut Cpu) {
    cpu.set_l(cpu.get_h());
}
// 0x8D
pub fn addc_a_l(cpu: &mut Cpu) {
    cpu.set_l(cpu.get_l());
}
// 0x8E
pub fn addc_a_addr_hl(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x8F
pub fn addc_a_a(cpu: &mut Cpu) {
    todo!("Not implemented");
}

// 0x90
pub fn sub_b(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x91
pub fn sub_c(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x92
pub fn sub_d(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x93
pub fn sub_e(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x94
pub fn sub_h(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x95
pub fn sub_l(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x96
pub fn sub_addr_hl(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x97
pub fn sub_a(cpu: &mut Cpu) {
    todo!("Not implemented");
}

// 0x98
pub fn sbc_a_b(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x99
pub fn subc_a_c(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x9A
pub fn subc_a_d(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x9B
pub fn subc_a_e(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x9C
pub fn subc_a_h(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x9D
pub fn subc_a_l(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x9E
pub fn subc_a_addr_hl(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0x9F
pub fn subc_a_a(cpu: &mut Cpu) {
    todo!("Not implemented");
}

// 0xA0
pub fn and_b(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xA1
pub fn and_c(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xA2
pub fn and_d(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xA3
pub fn and_e(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xA4
pub fn and_h(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xA5
pub fn and_l(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xA6
pub fn and_addr_hl(cpu: &mut Cpu) {
    todo!("Not implemented");
}
//0 xA7
pub fn and_a(cpu: &mut Cpu) {
    todo!("Not implemented");
}

// 0xA8
pub fn xor_b(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xA9
pub fn xor_c(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xAA
pub fn xor_d(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xAB
pub fn xor_e(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xAC
pub fn xor_h(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xAD
pub fn xor_l(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xAE
pub fn xor_addr_hl(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xAF
pub fn xor_a(cpu: &mut Cpu) {
    todo!("Not implemented");
}

// 0xB0
pub fn or_b(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xB1
pub fn or_c(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xB2
pub fn or_d(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xB3
pub fn or_e(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xB4
pub fn or_h(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xB5
pub fn or_l(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xB6
pub fn or_addr_hl(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xB7
pub fn or_a(cpu: &mut Cpu) {
    todo!("Not implemented");
}

// 0xB8
pub fn cp_b(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xB9
pub fn cp_c(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xBA
pub fn cp_d(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xBB
pub fn cp_e(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xBC
pub fn cp_h(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xBD
pub fn cp_l(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xBE
pub fn cp_addr_hl(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xBF
pub fn cp_a(cpu: &mut Cpu) {
    todo!("Not implemented");
}


// 0xC0
pub fn ret_nz(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xC1
pub fn  pop_bc(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xC2
pub fn jp_nz_addr16(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xC3
pub fn jp_addr16(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xC4
pub fn call_nz_addr16(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xC5
pub fn push_bc(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xC6
pub fn add_addr_d8(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xC7
pub fn rst_00h(cpu: &mut Cpu) {
    todo!("Not implemented");
}

// 0xC8
pub fn ret_z(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xC9
pub fn ret(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xCA
pub fn jp_z_addr16(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xCB
pub fn prefix_cb(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xCC
pub fn call_z_addr16(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xCD
pub fn call_a16(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xCE
pub fn adc_addr_d8(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xCF
pub fn rst_98h(cpu: &mut Cpu) {
    todo!("Not implemented");
}

// 0xD0
pub fn ret_nc(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xD1
pub fn  pop_de(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xD2
pub fn jp_nc_addr16(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xD3 - no operation

// 0xD4
pub fn call_nc_addr16(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xD5
pub fn push_de(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xD6
pub fn sub_d8(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xD7
pub fn rst_10h(cpu: &mut Cpu) {
    todo!("Not implemented");
}

// 0xD8
pub fn ret_c(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xD9
pub fn reti(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xDA
pub fn jp_c_addr16(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xDB - no operation

// 0xDC
pub fn call_c_addr16(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xDD - no operation

// 0xDE
pub fn sbc_a_addr8(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xDF
pub fn rst_18h(cpu: &mut Cpu) {
    todo!("Not implemented");
}

// 0xE0
pub fn lda_addr8_a(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xE1
pub fn  pop_hl(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xE2
pub fn ld_addr_c_a(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xE3 - no operation

// 0xE4 - no operation

// 0xE5
pub fn push_hl(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xE6
pub fn and_d8(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xE7
pub fn rst_20h(cpu: &mut Cpu) {
    todo!("Not implemented");
}

// 0xE8
pub fn add_sp_r8(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xE9
pub fn jp_addr_hl(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xEA
pub fn ld_addr16_a(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xEB - no operation

// 0xEC - no operation

// 0xED - no operation

// 0xEE
pub fn xor_d8(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xEF
pub fn rst_28h(cpu: &mut Cpu) {
    todo!("Not implemented");
}


// 0xF0
pub fn lda_a_addr8(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xF1
pub fn  pop_af(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xF2
pub fn ld_a_addr_c(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xF3 - no operation
pub fn di(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xF4 - no operation

// 0xF5
pub fn push_af(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xF6
pub fn or_d8(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xF7
pub fn rst_30h(cpu: &mut Cpu) {
    todo!("Not implemented");
}

// 0xF8
pub fn add_sp_plus_r8(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xF9
pub fn ld_sp_hl(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xFA
pub fn ld_a_addr16(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xFB - no operation
pub fn ei(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xFC - no operation

// 0xFD - no operation

// 0xFE
pub fn cp_d8(cpu: &mut Cpu) {
    todo!("Not implemented");
}
// 0xFF
pub fn rst_38h(cpu: &mut Cpu) {
    todo!("Not implemented");
}