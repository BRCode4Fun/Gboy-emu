#[cfg(test)]
mod test {
    use utils::cpu::instr::InstructionType;

    fn fetch(opcode : u8) -> String {
        let (instruction, _) = InstructionType::from_byte(opcode).unwrap();
        format!("{:?}", instruction)
    }
    #[test]
    fn decode() {
        assert_eq!(fetch(0x3E), "Load(ByteReg(A), ByteConst)");
        assert_eq!(fetch(0x76), "Halt");
    }
}