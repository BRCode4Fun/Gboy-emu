#[cfg(test)]
mod test {
    
    use utils::cpu::regs::{
        Registers,
        CpuFlag::{Z, N, H, C},
    };

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
}