#[cfg(test)]
mod test {
    use utils::{
        cpu::Cpu,
        cartridge::CartContext,
    };

    #[test]
    fn exec_instr() {

        let cart = CartContext::new();
        let mut cpu = Cpu::new(&cart);

        cpu.set_byte(0x0100, 0x3E); // LOAD A, n
        cpu.set_byte(0x0101, 0x12); // byteconst 18
        cpu.set_byte(0x0102, 0x47); // LOAD B, A
        cpu.set_byte(0x0103, 0x48); // LOAD C, B
        cpu.set_byte(0x0104, 0x51); // LOAD D, C
        cpu.set_byte(0x0105, 0x5A); // LOAD E, D
        cpu.set_byte(0x0106, 0x63); // LOAD H, E
        cpu.set_byte(0x0107, 0x6C); // LOAD L, H
        cpu.set_byte(0x0108, 0x76); // HALT

        cpu.run();

        assert_eq!(cpu.regs.a,  18);
        assert_eq!(cpu.regs.l,  18);
    }
}