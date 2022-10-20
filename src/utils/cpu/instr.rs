#[derive(Debug, Copy, Clone)]
pub enum JumpTest {
    Zero,     // jump if Z flag is set
    NotZero,  // jump if Z flag is reset
    Carry,    // jump if C flag is set
    NotCarry, // jump if C flag is reset
    Always,   // jump unconditionally
}

#[derive(Debug, Copy, Clone)]
pub enum Reg8 {
    A, B, C, D, E, H, L
}

#[derive(Debug, Copy, Clone)]
pub enum Reg16 {
    Af, Bc, De, Hl, Sp
}

#[derive(Debug, Copy, Clone)]
pub enum Addr {
    WordReg(Reg16),
    WordConst,        // 16-bit immediate constant
    RegRel(Reg8),    // register offset from 0xFF00 
    ByteRel,          // byte offset from 0xFF00
}

#[derive(Debug, Copy, Clone)]
pub enum Source {
    ByteReg(Reg8),  // 8-bit register
    WordReg(Reg16), // 16-bit register
    Deref(Addr),    // memory addressing
    ByteConst,      // 8-bit constant
    WordConst,      // 16-bit constant
}

#[derive(Debug, Copy, Clone)]
pub enum Target {
    ByteReg(Reg8),  // 8-bit register
    WordReg(Reg16), // 16-bit register
    Deref(Addr),    // memory addressing
}

#[derive(Debug, Copy, Clone)]
pub enum InstructionType {
    /// Arithmetic Instructions
    Inc(Target),         // increment target
    Dec(Target),         // decrement target
    
    Add(Target, Source), // add source to target
    Adc(Target, Source), // add source + carry to target
    Sub(Source),         // subtract source from A
    Sbc(Source),         // subtract source + Carry flag from A
    And(Source),         // logically AND source with A, result in A
    Or(Source),          // logical OR source with A, result in A
    Xor(Source),         // logical exclusive OR source with A, result in A
    Cp(Source),          // compare A with source

    Ccf,  // complement carry flag 
    Scf,  // set carry flag

    Rra,  // rotate A right through carry flag
    Rla, 
    Rrca, // rotate A right
    Rlca,
    Cpl,  // complement A register (Flip all bits)
    Daa,  // decimal adjust register A

    /*
    /// Prefix Instructions
    Bit(),
    Res(),
    Set(),
    Srl(),
    Rr(),
    Rl(),
    Rrc(),
    Rlc(),
    Sra(),
    Sla(),
    Swap(),
    */

    /// Jump Instructions
    Jp(JumpTest, Source),  // jump to source address if jumptest is valid
    Jr(JumpTest, Source),  // jump relative to source if jumptest is valid
    
    /// Load Instructions
    Load(Target, Source),  // load source into target
    LoadI(Target, Source), // load source into target and increment address from HL
    LoadD(Target, Source), // load source into target and decrement address from HL
    LoadH(Target, Source),  // load relative to 0xFF00
    LoadHL(Target, Source), // load target + source effective address into HL

    /// Stack Instructions
    Push(Reg16),
    Pop(Reg16),
    Call(JumpTest, Source), // call source address if jumptest is valid
    Rst(Source),            // restart program flow to source address
    Ret(JumpTest),          // return if jumptest is true
    Reti,                   // return and enable interrupts

    /// Control Instructions
    Stop(Source),
    Halt,
    Nop,
    Di,     // interrupts disable 
    Ei,     // interrupts enable
}

impl InstructionType {
    pub fn from_byte(opcode : u8) -> Option<(InstructionType, u8)> {
        match opcode {
            // NOP
            0x00 => Some((InstructionType::Nop, 4)),
            // LD BC, nn
            0x01 => Some((InstructionType::Load(Target::WordReg(Reg16::Bc), Source::WordConst), 12)),
            // LD (BC), A
            0x02 => Some((InstructionType::Load(Target::Deref(Addr::WordReg(Reg16::Bc)), Source::ByteReg(Reg8::A)), 8)),
            // INC BC
            0x03 => Some((InstructionType::Inc(Target::WordReg(Reg16::Bc)), 8)),
            // INC B
            0x04 => Some((InstructionType::Inc(Target::ByteReg(Reg8::B)), 4)),
            // DEC B
            0x05 => Some((InstructionType::Dec(Target::ByteReg(Reg8::B)), 4)),
            // LD B, n
            0x06 => Some((InstructionType::Load(Target::ByteReg(Reg8::B), Source::ByteConst), 8)),
            // RLCA
            0x07 => Some((InstructionType::Rlca, 4)),
            // LD (nn), SP          
            0x08 => Some((InstructionType::Load(Target::Deref(Addr::WordConst), Source::WordReg(Reg16::Sp)), 20)),
            // ADD HL, BC
            0x09 => Some((InstructionType::Add(Target::WordReg(Reg16::Hl), Source::WordReg(Reg16::Bc)), 8)),
            // LD A, (BC)
            0x0A => Some((InstructionType::Load(Target::ByteReg(Reg8::A), Source::Deref(Addr::WordReg(Reg16::Bc))), 8)),
            // DEC BC
            0x0B => Some((InstructionType::Dec(Target::WordReg(Reg16::Bc)), 8)),
            // INC C
            0x0C => Some((InstructionType::Inc(Target::ByteReg(Reg8::C)), 4)),
            // DEC C
            0x0D => Some((InstructionType::Dec(Target::ByteReg(Reg8::C)), 4)),
            // LD C, n
            0x0E => Some((InstructionType::Load(Target::ByteReg(Reg8::C), Source::ByteConst), 8)),
            // RRCA
            0x0F => Some((InstructionType::Rrca, 4)),
            // STOP 00
            0x10 => Some((InstructionType::Stop(Source::ByteConst), 4)),
            // LD DE, nn
            0x11 => Some((InstructionType::Load(Target::WordReg(Reg16::De), Source::WordConst), 12)),
            // LD (DE), A
            0x12 => Some((InstructionType::Load(Target::Deref(Addr::WordReg(Reg16::Bc)), Source::ByteReg(Reg8::A)), 8)),
            // INC DE
            0x13 => Some((InstructionType::Inc(Target::WordReg(Reg16::De)), 8)),
            // INC D
            0x14 => Some((InstructionType::Inc(Target::ByteReg(Reg8::D)), 4)),
            // DEC D
            0x15 => Some((InstructionType::Dec(Target::ByteReg(Reg8::D)), 4)),
            // LD D, n
            0x16 => Some((InstructionType::Load(Target::ByteReg(Reg8::D), Source::ByteConst), 8)),
            // RLA
            0x17 => Some((InstructionType::Rla, 4)),
            // JR n
            0x18 => Some((InstructionType::Jr(JumpTest::Always, Source::ByteConst), 8)),
            // ADD HL, DE
            0x19 => Some((InstructionType::Add(Target::WordReg(Reg16::Hl), Source::WordReg(Reg16::De)), 8)),
            // LD A, (DE)
            0x1A => Some((InstructionType::Load(Target::ByteReg(Reg8::A), Source::Deref(Addr::WordReg(Reg16::De))), 8)),
            // DEC DE
            0x1B => Some((InstructionType::Dec(Target::WordReg(Reg16::De)), 8)),
            // INC E
            0x1C => Some((InstructionType::Inc(Target::ByteReg(Reg8::E)), 4)),
            // DEC E
            0x1D => Some((InstructionType::Dec(Target::ByteReg(Reg8::E)), 4)),
            // LD E, n
            0x1E => Some((InstructionType::Load(Target::ByteReg(Reg8::E), Source::ByteConst), 8)),
            // RRA
            0x1F => Some((InstructionType::Rra, 4)),
            // JR NZ, n
            0x20 => Some((InstructionType::Jr(JumpTest::NotZero, Source::ByteConst), 8)),
            // LD HL, nn
            0x21 => Some((InstructionType::Load(Target::WordReg(Reg16::Hl), Source::WordConst), 12)),
            // INC HL
            0x23 => Some((InstructionType::Inc(Target::WordReg(Reg16::Hl)), 8)),
            // INC H
            0x24 => Some((InstructionType::Inc(Target::ByteReg(Reg8::H)), 4)),
            // DEC H
            0x25 => Some((InstructionType::Dec(Target::ByteReg(Reg8::H)), 4)),
            // LD H, n
            0x26 => Some((InstructionType::Load(Target::ByteReg(Reg8::H), Source::ByteConst), 8)),
            // DAA
            0x27 => Some((InstructionType::Daa, 4)),
            // JR NZ, n
            0x28 => Some((InstructionType::Jr(JumpTest::Zero, Source::ByteConst), 8)),
            // ADD HL, HL
            0x29 => Some((InstructionType::Add(Target::WordReg(Reg16::Hl), Source::WordReg(Reg16::Hl)), 8)),
            // LDI A, (HL)
            0x2A => Some((InstructionType::LoadI(Target::ByteReg(Reg8::A), Source::Deref(Addr::WordReg(Reg16::Hl))), 8)),
            // DEC HL
            0x2B => Some((InstructionType::Dec(Target::WordReg(Reg16::Hl)), 8)),
            // INC L
            0x2C => Some((InstructionType::Inc(Target::ByteReg(Reg8::L)), 4)),
            // DEC L
            0x2D => Some((InstructionType::Dec(Target::ByteReg(Reg8::L)), 4)),
            // LD L, n
            0x2E => Some((InstructionType::Load(Target::ByteReg(Reg8::L), Source::ByteConst), 8)),
            // CPL
            0x2F => Some((InstructionType::Cpl, 4)),
            // JR NC, n
            0x30 => Some((InstructionType::Jr(JumpTest::NotCarry, Source::ByteConst), 8)),
            // LD SP, nn
            0x31 => Some((InstructionType::Load(Target::WordReg(Reg16::Sp), Source::WordConst), 12)),
            // LDD (HL), A
            0x32 => Some((InstructionType::LoadD(Target::Deref(Addr::WordReg(Reg16::Hl)), Source::ByteReg(Reg8::A)), 8)),
            // INC SP
            0x33 => Some((InstructionType::Inc(Target::WordReg(Reg16::Sp)), 8)),
            // INC (HL)
            0x34 => Some((InstructionType::Inc(Target::Deref(Addr::WordReg(Reg16::Hl))), 12)),
            // DEC (HL)
            0x35 => Some((InstructionType::Dec(Target::Deref(Addr::WordReg(Reg16::Hl))), 12)),
            // LD (HL), n
            0x36 => Some((InstructionType::Load(Target::Deref(Addr::WordReg(Reg16::Hl)), Source::ByteConst), 12)),
            // SCF
            0x37 => Some((InstructionType::Scf, 4)),
            // JR C, n
            0x38 => Some((InstructionType::Jr(JumpTest::Carry, Source::ByteConst), 8)),
            // ADD HL, SP
            0x39 => Some((InstructionType::Add(Target::WordReg(Reg16::Hl), Source::WordReg(Reg16::Sp)), 8)),
            // LDD A, (HL)
            0x3A => Some((InstructionType::LoadD(Target::ByteReg(Reg8::A), Source::Deref(Addr::WordReg(Reg16::Hl))), 8)),
            // DEC SP
            0x3B => Some((InstructionType::Dec(Target::WordReg(Reg16::Sp)), 8)),
            // INC A
            0x3C => Some((InstructionType::Inc(Target::ByteReg(Reg8::A)), 4)),
            // DEC A
            0x3D => Some((InstructionType::Dec(Target::ByteReg(Reg8::A)), 4)),
            // LD A, n
            0x3E => Some((InstructionType::Load(Target::ByteReg(Reg8::A), Source::ByteConst), 8)),
            // CCF
            0x3F => Some((InstructionType::Ccf, 4)),
            // LD B, B
            0x40 => Some((InstructionType::Nop, 4)),
            // LD B, C
            0x41 => Some((InstructionType::Load(Target::ByteReg(Reg8::B), Source::ByteReg(Reg8::C)), 4)),
            // LD B, D
            0x42 => Some((InstructionType::Load(Target::ByteReg(Reg8::B), Source::ByteReg(Reg8::D)), 4)),
            // LD B, E
            0x43 => Some((InstructionType::Load(Target::ByteReg(Reg8::B), Source::ByteReg(Reg8::E)), 4)),
            // LD B, H
            0x44 => Some((InstructionType::Load(Target::ByteReg(Reg8::B), Source::ByteReg(Reg8::H)), 4)),
            // LD B, L
            0x45 => Some((InstructionType::Load(Target::ByteReg(Reg8::B), Source::ByteReg(Reg8::L)), 4)),
            // LD B, (HL)
            0x46 => Some((InstructionType::Load(Target::ByteReg(Reg8::B), Source::Deref(Addr::WordReg(Reg16::Hl))), 8)),
            // LD B, A
            0x47 => Some((InstructionType::Load(Target::ByteReg(Reg8::B), Source::ByteReg(Reg8::A)), 4)),
            // LD C, B
            0x48 => Some((InstructionType::Load(Target::ByteReg(Reg8::C), Source::ByteReg(Reg8::B)), 4)),
            // LD C, C
            0x49 => Some((InstructionType::Nop, 4)),
            // LD C, D
            0x4A => Some((InstructionType::Load(Target::ByteReg(Reg8::C), Source::ByteReg(Reg8::D)), 4)),
            // LD C, E
            0x4B => Some((InstructionType::Load(Target::ByteReg(Reg8::C), Source::ByteReg(Reg8::E)), 4)),
            // LD C, H
            0x4C => Some((InstructionType::Load(Target::ByteReg(Reg8::C), Source::ByteReg(Reg8::H)), 4)),
            // LD C, L
            0x4D => Some((InstructionType::Load(Target::ByteReg(Reg8::C), Source::ByteReg(Reg8::L)), 4)),
            // LD C, (HL)
            0x4E => Some((InstructionType::Load(Target::ByteReg(Reg8::C), Source::Deref(Addr::WordReg(Reg16::Hl))), 8)),
            // LD C, A
            0x4F => Some((InstructionType::Load(Target::ByteReg(Reg8::C), Source::ByteReg(Reg8::A)), 4)),
            // LD D, B
            0x50 => Some((InstructionType::Load(Target::ByteReg(Reg8::D), Source::ByteReg(Reg8::B)), 4)),
            // LD D, C
            0x51 => Some((InstructionType::Load(Target::ByteReg(Reg8::D), Source::ByteReg(Reg8::C)), 4)),
            // LD D, D
            0x52 => Some((InstructionType::Nop, 4)),
            // LD D, E
            0x53 => Some((InstructionType::Load(Target::ByteReg(Reg8::D), Source::ByteReg(Reg8::E)), 4)),
            // LD D, H
            0x54 => Some((InstructionType::Load(Target::ByteReg(Reg8::D), Source::ByteReg(Reg8::H)), 4)),
            // LD D, L
            0x55 => Some((InstructionType::Load(Target::ByteReg(Reg8::D), Source::ByteReg(Reg8::L)), 4)),
            // LD D, (HL)
            0x56 => Some((InstructionType::Load(Target::ByteReg(Reg8::D), Source::Deref(Addr::WordReg(Reg16::Hl))), 8)),
            // LD D, A
            0x57 => Some((InstructionType::Load(Target::ByteReg(Reg8::D), Source::ByteReg(Reg8::A)), 4)),
            // LD E, B
            0x58 => Some((InstructionType::Load(Target::ByteReg(Reg8::E), Source::ByteReg(Reg8::B)), 4)),
            // LD E, C
            0x59 => Some((InstructionType::Load(Target::ByteReg(Reg8::E), Source::ByteReg(Reg8::C)), 4)),
            // LD E, D
            0x5A => Some((InstructionType::Load(Target::ByteReg(Reg8::E), Source::ByteReg(Reg8::D)), 4)),
            // LD E, E
            0x5B => Some((InstructionType::Nop, 4)),
            // LD E, H
            0x5C => Some((InstructionType::Load(Target::ByteReg(Reg8::E), Source::ByteReg(Reg8::H)), 4)),
            // LD E, L
            0x5D => Some((InstructionType::Load(Target::ByteReg(Reg8::E), Source::ByteReg(Reg8::L)), 4)),
            // LD E, (HL)
            0x5E => Some((InstructionType::Load(Target::ByteReg(Reg8::E), Source::Deref(Addr::WordReg(Reg16::Hl))), 8)),
            // LD E, A
            0x5F => Some((InstructionType::Load(Target::ByteReg(Reg8::E), Source::ByteReg(Reg8::A)), 4)),
            // LD H, B
            0x60 => Some((InstructionType::Load(Target::ByteReg(Reg8::H), Source::ByteReg(Reg8::B)), 4)),
            // LD H, C
            0x61 => Some((InstructionType::Load(Target::ByteReg(Reg8::H), Source::ByteReg(Reg8::C)), 4)),
            // LD H, D
            0x62 => Some((InstructionType::Load(Target::ByteReg(Reg8::H), Source::ByteReg(Reg8::D)), 4)),
            // LD H, E
            0x63 => Some((InstructionType::Load(Target::ByteReg(Reg8::H), Source::ByteReg(Reg8::E)), 4)),
            // LD H, H
            0x64 => Some((InstructionType::Nop, 4)),
            // LD H, L
            0x65 => Some((InstructionType::Load(Target::ByteReg(Reg8::H), Source::ByteReg(Reg8::L)), 4)),
            // LD H, (HL)
            0x66 => Some((InstructionType::Load(Target::ByteReg(Reg8::H), Source::Deref(Addr::WordReg(Reg16::Hl))), 8)),
            // LD H, A
            0x67 => Some((InstructionType::Load(Target::ByteReg(Reg8::H), Source::ByteReg(Reg8::A)), 4)),
            // LD L, B
            0x68 => Some((InstructionType::Load(Target::ByteReg(Reg8::L), Source::ByteReg(Reg8::B)), 4)),
            // LD L, C
            0x69 => Some((InstructionType::Load(Target::ByteReg(Reg8::L), Source::ByteReg(Reg8::C)), 4)),
            // LD L, D
            0x6A => Some((InstructionType::Load(Target::ByteReg(Reg8::L), Source::ByteReg(Reg8::D)), 4)),
            // LD L, E
            0x6B => Some((InstructionType::Load(Target::ByteReg(Reg8::L), Source::ByteReg(Reg8::E)), 4)),
            // LD L, H
            0x6C => Some((InstructionType::Load(Target::ByteReg(Reg8::L), Source::ByteReg(Reg8::H)), 4)),
            // LD L, L
            0x6D => Some((InstructionType::Nop, 4)),
            // LD H, (HL)
            0x6E => Some((InstructionType::Load(Target::ByteReg(Reg8::L), Source::Deref(Addr::WordReg(Reg16::Hl))), 8)),
            // LD L, A
            0x6F => Some((InstructionType::Load(Target::ByteReg(Reg8::L), Source::ByteReg(Reg8::A)), 4)),
            // LD (HL), B
            0x70 => Some((InstructionType::Load(Target::Deref(Addr::WordReg(Reg16::Hl)), Source::ByteReg(Reg8::B)), 8)),
            // LD (HL), C
            0x71 => Some((InstructionType::Load(Target::Deref(Addr::WordReg(Reg16::Hl)), Source::ByteReg(Reg8::C)), 8)),
            // LD (HL), D
            0x72 => Some((InstructionType::Load(Target::Deref(Addr::WordReg(Reg16::Hl)), Source::ByteReg(Reg8::D)), 8)),
            // LD (HL), E
            0x73 => Some((InstructionType::Load(Target::Deref(Addr::WordReg(Reg16::Hl)), Source::ByteReg(Reg8::E)), 8)),
            // LD (HL), H
            0x74 => Some((InstructionType::Load(Target::Deref(Addr::WordReg(Reg16::Hl)), Source::ByteReg(Reg8::H)), 8)),
            // LD (HL), L
            0x75 => Some((InstructionType::Load(Target::Deref(Addr::WordReg(Reg16::Hl)), Source::ByteReg(Reg8::L)), 8)),
            // HALT
            0x76 => Some((InstructionType::Halt, 4)),
            // LD (HL), A
            0x77 => Some((InstructionType::Load(Target::Deref(Addr::WordReg(Reg16::Hl)), Source::ByteReg(Reg8::A)), 8)),
            // LD A, B
            0x78 => Some((InstructionType::Load(Target::ByteReg(Reg8::A), Source::ByteReg(Reg8::B)), 4)),
            // LD A, C
            0x79 => Some((InstructionType::Load(Target::ByteReg(Reg8::A), Source::ByteReg(Reg8::C)), 4)),
            // LD A, D
            0x7A => Some((InstructionType::Load(Target::ByteReg(Reg8::A), Source::ByteReg(Reg8::D)), 4)),
            // LD A, E
            0x7B => Some((InstructionType::Load(Target::ByteReg(Reg8::A), Source::ByteReg(Reg8::E)), 4)),
            // LD A, H
            0x7C => Some((InstructionType::Load(Target::ByteReg(Reg8::A), Source::ByteReg(Reg8::H)), 4)),
            // LD A, L
            0x7D => Some((InstructionType::Load(Target::ByteReg(Reg8::A), Source::ByteReg(Reg8::L)), 4)),
            // LD A, (HL)
            0x7E => Some((InstructionType::Load(Target::ByteReg(Reg8::A), Source::Deref(Addr::WordReg(Reg16::Hl))), 8)),
            // LD A, A
            0x7F => Some((InstructionType::Nop, 4)),
            // ADD A, B
            0x80 => Some((InstructionType::Add(Target::ByteReg(Reg8::A), Source::ByteReg(Reg8::B)), 4)),
            // ADD A, C
            0x81 => Some((InstructionType::Add(Target::ByteReg(Reg8::A), Source::ByteReg(Reg8::C)), 4)),
            // ADD A, D
            0x82 => Some((InstructionType::Add(Target::ByteReg(Reg8::A), Source::ByteReg(Reg8::D)), 4)),
            // ADD A, E
            0x83 => Some((InstructionType::Add(Target::ByteReg(Reg8::A), Source::ByteReg(Reg8::E)), 4)),
            // ADD A, H
            0x84 => Some((InstructionType::Add(Target::ByteReg(Reg8::A), Source::ByteReg(Reg8::H)), 4)),
            // ADD A, L
            0x85 => Some((InstructionType::Add(Target::ByteReg(Reg8::A), Source::ByteReg(Reg8::L)), 4)),
            // ADD A, (HL)
            0x86 => Some((InstructionType::Add(Target::ByteReg(Reg8::A), Source::Deref(Addr::WordReg(Reg16::Hl))), 8)),
            // ADD A, A
            0x87 => Some((InstructionType::Add(Target::ByteReg(Reg8::A), Source::ByteReg(Reg8::A)), 4)),
            // ADC A, B
            0x88 => Some((InstructionType::Adc(Target::ByteReg(Reg8::A), Source::ByteReg(Reg8::B)), 4)),
            // ADC A, C
            0x89 => Some((InstructionType::Adc(Target::ByteReg(Reg8::A), Source::ByteReg(Reg8::C)), 4)),
            // ADC A, D
            0x8A => Some((InstructionType::Adc(Target::ByteReg(Reg8::A), Source::ByteReg(Reg8::D)), 4)),
            // ADC A, E
            0x8B => Some((InstructionType::Adc(Target::ByteReg(Reg8::A), Source::ByteReg(Reg8::E)), 4)),
            // ADC A, H
            0x8C => Some((InstructionType::Adc(Target::ByteReg(Reg8::A), Source::ByteReg(Reg8::H)), 4)),
            // ADC A, L
            0x8D => Some((InstructionType::Adc(Target::ByteReg(Reg8::A), Source::ByteReg(Reg8::L)), 4)),
            // ADC A, (HL)
            0x8E => Some((InstructionType::Adc(Target::ByteReg(Reg8::A), Source::Deref(Addr::WordReg(Reg16::Hl))), 8)),
            // ADC A, A
            0x8F => Some((InstructionType::Adc(Target::ByteReg(Reg8::A), Source::ByteReg(Reg8::A)), 4)),
            // SUB B
            0x90 => Some((InstructionType::Sub(Source::ByteReg(Reg8::B)), 4)),
            // SUB C
            0x91 => Some((InstructionType::Sub(Source::ByteReg(Reg8::C)), 4)),
            // SUB D
            0x92 => Some((InstructionType::Sub(Source::ByteReg(Reg8::D)), 4)),
            // SUB E
            0x93 => Some((InstructionType::Sub(Source::ByteReg(Reg8::E)), 4)),
            // SUB H
            0x94 => Some((InstructionType::Sub(Source::ByteReg(Reg8::H)), 4)),
            // SUB L
            0x95 => Some((InstructionType::Sub(Source::ByteReg(Reg8::L)), 4)),
            // SUB (HL)
            0x96 => Some((InstructionType::Sub(Source::Deref(Addr::WordReg(Reg16::Hl))), 8)),
            // SUB A
            0x97 => Some((InstructionType::Sub(Source::ByteReg(Reg8::A)), 4)),
            // SBC A, B
            0x98 => Some((InstructionType::Sbc(Source::ByteReg(Reg8::B)), 4)),
            // SBC A, C
            0x99 => Some((InstructionType::Sbc(Source::ByteReg(Reg8::C)), 4)),
            // SBC A, D
            0x9A => Some((InstructionType::Sbc(Source::ByteReg(Reg8::D)), 4)),
            // SBC A, E
            0x9B => Some((InstructionType::Sbc(Source::ByteReg(Reg8::E)), 4)),
            // SBC A, H
            0x9C => Some((InstructionType::Sbc(Source::ByteReg(Reg8::H)), 4)),
            // SBC A, L
            0x9D => Some((InstructionType::Sbc(Source::ByteReg(Reg8::L)), 4)),
            // SBC A, (HL)
            0x9E => Some((InstructionType::Sbc(Source::Deref(Addr::WordReg(Reg16::Hl))), 8)),
            // SBC A, A
            0x9F => Some((InstructionType::Sbc(Source::ByteReg(Reg8::A)), 4)),
            // AND A, B
            0xA0 => Some((InstructionType::And(Source::ByteReg(Reg8::B)), 4)),
            // AND A, C
            0xA1 => Some((InstructionType::And(Source::ByteReg(Reg8::C)), 4)),
            // AND A, D
            0xA2 => Some((InstructionType::And(Source::ByteReg(Reg8::D)), 4)),
            // AND A, E
            0xA3 => Some((InstructionType::And(Source::ByteReg(Reg8::E)), 4)),
            // AND A, H
            0xA4 => Some((InstructionType::And(Source::ByteReg(Reg8::H)), 4)),
            // AND A, L
            0xA5 => Some((InstructionType::And(Source::ByteReg(Reg8::L)), 4)),
            // AND A, (HL)
            0xA6 => Some((InstructionType::And(Source::Deref(Addr::WordReg(Reg16::Hl))), 8)),
            // AND A, A
            0xA7 => Some((InstructionType::And(Source::ByteReg(Reg8::A)), 4)),
            // XOR A, B
            0xA8 => Some((InstructionType::Xor(Source::ByteReg(Reg8::B)), 4)),
            // XOR A, C
            0xA9 => Some((InstructionType::Xor(Source::ByteReg(Reg8::C)), 4)),
            // XOR A, D
            0xAA => Some((InstructionType::Xor(Source::ByteReg(Reg8::D)), 4)),
            // XOR A, E
            0xAB => Some((InstructionType::Xor(Source::ByteReg(Reg8::E)), 4)),
            // XOR A, H
            0xAC => Some((InstructionType::Xor(Source::ByteReg(Reg8::H)), 4)),
            // XOR A, L
            0xAD => Some((InstructionType::Xor(Source::ByteReg(Reg8::L)), 4)),
            // XOR A, (HL)
            0xAE => Some((InstructionType::Xor(Source::Deref(Addr::WordReg(Reg16::Hl))), 8)),
            // XOR A, A
            0xAF => Some((InstructionType::Xor(Source::ByteReg(Reg8::A)), 4)),
            // OR A, B
            0xB0 => Some((InstructionType::Or(Source::ByteReg(Reg8::B)), 4)),
            // OR A, C
            0xB1 => Some((InstructionType::Or(Source::ByteReg(Reg8::C)), 4)),
            // OR A, D
            0xB2 => Some((InstructionType::Or(Source::ByteReg(Reg8::D)), 4)),
            // OR A, E
            0xB3 => Some((InstructionType::Or(Source::ByteReg(Reg8::E)), 4)),
            // OR A, H
            0xB4 => Some((InstructionType::Or(Source::ByteReg(Reg8::H)), 4)),
            // OR A, L
            0xB5 => Some((InstructionType::Or(Source::ByteReg(Reg8::L)), 4)),
            // OR A, (HL)
            0xB6 => Some((InstructionType::Or(Source::Deref(Addr::WordReg(Reg16::Hl))), 8)),
            // OR A, A
            0xB7 => Some((InstructionType::Or(Source::ByteReg(Reg8::A)), 4)),
            // CP A, B
            0xB8 => Some((InstructionType::Cp(Source::ByteReg(Reg8::B)), 4)),
            // CP A, C
            0xB9 => Some((InstructionType::Cp(Source::ByteReg(Reg8::C)), 4)),
            // CP A, D
            0xBA => Some((InstructionType::Cp(Source::ByteReg(Reg8::D)), 4)),
            // CP A, E
            0xBB => Some((InstructionType::Cp(Source::ByteReg(Reg8::E)), 4)),
            // CP A, H
            0xBC => Some((InstructionType::Cp(Source::ByteReg(Reg8::H)), 4)),
            // CP A, L
            0xBD => Some((InstructionType::Cp(Source::ByteReg(Reg8::L)), 4)),
            // CP A, (HL)
            0xBE => Some((InstructionType::Cp(Source::Deref(Addr::WordReg(Reg16::Hl))), 8)),
            // CP A, A
            0xBF => Some((InstructionType::Cp(Source::ByteReg(Reg8::A)), 4)),
            // RET NZ
            0xC0 => Some((InstructionType::Ret(JumpTest::NotZero), 8)),
            // POP BC
            0xC1 => Some((InstructionType::Pop(Reg16::Bc), 12)),
            // JP NZ, nn
            0xC2 => Some((InstructionType::Jp(JumpTest::NotZero, Source::WordConst), 12)),
            // JP nn
            0xC3 => Some((InstructionType::Jp(JumpTest::Always, Source::WordConst), 12)),
            // CALL NZ, nn
            0xC4 => Some((InstructionType::Call(JumpTest::NotZero, Source::WordConst), 12)),
            // PUSH BC
            0xC5 => Some((InstructionType::Push(Reg16::Bc), 16)),
            // ADD A, n
            0xC6 => Some((InstructionType::Add(Target::ByteReg(Reg8::A), Source::ByteConst), 8)),
            // RST n
            0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF => Some((InstructionType::Rst(Source::ByteConst), 32)),
            // RET Z
            0xC8 => Some((InstructionType::Ret(JumpTest::Zero), 8)),
            // RET
            0xC9 => Some((InstructionType::Ret(JumpTest::Always), 8)),
            // JP Z, nn
            0xCA => Some((InstructionType::Jp(JumpTest::Zero, Source::WordConst), 12)),
            // CALL Z, nn
            0xCC => Some((InstructionType::Call(JumpTest::Zero, Source::WordConst), 12)),
            // CALL nn
            0xCD => Some((InstructionType::Call(JumpTest::Always, Source::WordConst), 12)),
            // ADC A, n
            0xCE => Some((InstructionType::Adc(Target::ByteReg(Reg8::A), Source::ByteConst), 8)),
            // RET NC
            0xD0 => Some((InstructionType::Ret(JumpTest::NotCarry), 8)),
            // POP DE
            0xD1 => Some((InstructionType::Pop(Reg16::De), 12)),
            // JP NC, nn
            0xD2 => Some((InstructionType::Jp(JumpTest::NotCarry, Source::WordConst), 12)),
            // CALL NC, nn
            0xD4 => Some((InstructionType::Call(JumpTest::NotCarry, Source::WordConst), 12)),
            // PUSH DE
            0xD5 => Some((InstructionType::Push(Reg16::De), 16)),
            // SUB n
            0xD6 => Some((InstructionType::Sub(Source::ByteConst), 8)),
            // RET C
            0xD8 => Some((InstructionType::Ret(JumpTest::Carry), 8)),
            // RETI
            0xD9 => Some((InstructionType::Reti, 8)),
            // JP C, nn
            0xDA => Some((InstructionType::Jp(JumpTest::Carry, Source::WordConst), 12)),
            // CALL C, nn
            0xDC => Some((InstructionType::Call(JumpTest::Carry, Source::WordConst), 12)),
            // SBC A, n
            0xDE => Some((InstructionType::Sbc(Source::ByteConst), 8)),
            // LDH (n), A
            0xE0 => Some((InstructionType::LoadH(Target::Deref(Addr::ByteRel), Source::ByteReg(Reg8::A)), 12)),
            // POP HL
            0xE1 => Some((InstructionType::Pop(Reg16::Hl), 12)),
            // LD (C), A
            0xE2 => Some((InstructionType::LoadH(Target::Deref(Addr::RegRel(Reg8::C)), Source::ByteReg(Reg8::A)), 8)),
            // PUSH HL
            0xE5 => Some((InstructionType::Push(Reg16::Hl), 16)),
            // AND A, n
            0xE6 => Some((InstructionType::And(Source::ByteConst), 8)),
            // ADD SP, n
            0xE8 => Some((InstructionType::Add(Target::WordReg(Reg16::Sp), Source::ByteConst), 16)),
            // JP (HL)
            0xE9 => Some((InstructionType::Jp(JumpTest::Always, Source::Deref(Addr::WordReg(Reg16::Hl))), 4)),
            // LD (nn), A
            0xEA => Some((InstructionType::Load(Target::Deref(Addr::WordConst), Source::ByteReg(Reg8::A)), 16)),
            // XOR A, n
            0xEE => Some((InstructionType::Xor(Source::ByteConst), 8)),
            // LDH A, (n)
            0xF0 => Some((InstructionType::LoadH(Target::ByteReg(Reg8::A), Source::Deref(Addr::ByteRel)), 12)),
            // POP AF
            0xF1 => Some((InstructionType::Pop(Reg16::Af), 12)),
            // LD A, (C)
            0xF2 => Some((InstructionType::LoadH(Target::ByteReg(Reg8::A), Source::Deref(Addr::RegRel(Reg8::C))), 8)),
            // DI
            0xF3 => Some((InstructionType::Di, 4)),
            // PUSH AF
            0xF5 => Some((InstructionType::Push(Reg16::Af), 16)),
            // OR A, n
            0xF6 => Some((InstructionType::Or(Source::ByteConst), 8)),
            // LDHL SP, n
            0xF8 => Some((InstructionType::LoadHL(Target::WordReg(Reg16::Sp), Source::ByteConst), 12)),
            // LD SP, HL
            0xF9 => Some((InstructionType::Load(Target::WordReg(Reg16::Sp), Source::WordReg(Reg16::Hl)), 8)),
            // LD A, (nn)
            0xFA => Some((InstructionType::Load(Target::ByteReg(Reg8::A), Source::Deref(Addr::WordConst)), 16)),
            // EI
            0xFB => Some((InstructionType::Ei, 4)),
            // CP n
            0xFE => Some((InstructionType::Cp(Source::ByteConst), 8)),

            _    => None
        }
    }
    pub fn from_byte_prefixed(_opcode : u8) -> Option<(InstructionType, u8)> {
        todo!("Add Gameboy Color support")
    }
}