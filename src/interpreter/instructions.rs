const FIRST_NIBBLE_MASK: u8 = 0xF0;
const SECOND_NIBBLE_MASK: u8 = 0x0F;

#[derive(Debug)]
pub struct Opcode {
    nb1: u8,
    nb2: u8,
    nb3: u8,
    nb4: u8,
}

impl Opcode {
    pub fn new(msb: u8, lsb: u8) -> Opcode {
        Opcode {
            nb1: msb & FIRST_NIBBLE_MASK,
            nb2: msb & SECOND_NIBBLE_MASK,
            nb3: lsb & FIRST_NIBBLE_MASK,
            nb4: lsb & SECOND_NIBBLE_MASK,
        }
    }

    fn as_nibbles(&self) -> (u8, u8, u8, u8) {
        return (self.nb1, self.nb2, self.nb3, self.nb4);
    }
}

#[derive(Debug)]
pub enum Instruction {
    NOP,
    CLS,
}

pub fn decode(opcode: Opcode) -> Instruction {
    match opcode.as_nibbles() {
        (0, 0, 0, 0) => Instruction::NOP,
        (0, 0, 0xE, 0) => Instruction::CLS,
        _ => panic!("Invalid instruction!"),
    }
}
