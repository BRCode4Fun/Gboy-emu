#[cfg(test)]
mod test {
    use utils::cpu::{
        instr::InstructionType
    };
    fn fetch(opcode : u8) -> InstructionType {
        let (instruction, _) = InstructionType::from_byte(opcode).unwrap();
        instruction
    }
    #[test]
    fn decode() {
        assert_eq!(format!("{:?}", fetch(0x3E)), "Load(ByteReg(A), ByteConst)");
        assert_eq!(format!("{:?}", fetch(0x76)), "Halt");
    }
}