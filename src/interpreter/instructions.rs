use super::{Addr, Cell, Nibble};

#[derive(Debug)]
pub enum Instruction {
    NOP,
    CLS,
    JP(Addr),
    LD(Nibble, Cell),
    ADD(Nibble, Cell),
    LDI(Addr),
    DRW(Nibble, Nibble, Nibble),
}

#[derive(Debug)]
pub struct Opcode {
    msb: Cell,
    lsb: Cell,
}

fn first_nibble(byte: Cell) -> Nibble {
    (byte & 0xF0) >> 4
}

fn second_nibble(byte: Cell) -> Nibble {
    byte & 0x0F
}

fn nibbles_to_addr(n1: Nibble, n2: Nibble, n3: Nibble) -> Addr {
    (Addr::from(n1) << 8) | (Addr::from(n2) << 4) | Addr::from(n3)
}

fn nibbles_to_cell(n1: Nibble, n2: Nibble) -> Cell {
    (n1 << 4) | n2
}

impl Opcode {
    pub fn new(msb: Cell, lsb: Cell) -> Self {
        Self { msb, lsb }
    }

    fn as_nibbles(&self) -> (Nibble, Nibble, Nibble, Nibble) {
        return (
            first_nibble(self.msb),
            second_nibble(self.msb),
            first_nibble(self.lsb),
            second_nibble(self.lsb),
        );
    }

    pub fn decode(&self) -> Instruction {
        // println!("{:?}", self.as_nibbles());
        match self.as_nibbles() {
            (0, 0, 0, 0) => Instruction::NOP,
            (0, 0, 0xE, 0) => Instruction::CLS,
            (1, n1, n2, n3) => Instruction::JP(nibbles_to_addr(n1, n2, n3)),
            (6, x, k1, k2) => Instruction::LD(x, nibbles_to_cell(k1, k2)),
            (7, x, k1, k2) => Instruction::ADD(x, nibbles_to_cell(k1, k2)),
            (0xA, n1, n2, n3) => Instruction::LDI(nibbles_to_addr(n1, n2, n3)),
            (0xD, x, y, n) => Instruction::DRW(x, y, n),
            _ => todo!("Unimplemented instruction!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_nibble() {
        assert_eq!(first_nibble(0xAB), 0xA);
    }

    #[test]
    fn test_second_nibble() {
        assert_eq!(second_nibble(0xAB), 0xB);
    }

    #[test]
    fn test_nibbles_to_addr() {
        assert_eq!(nibbles_to_addr(0xA, 0xB, 0xC), 0xABC);
    }

    #[test]
    fn test_nibbles_to_cell() {
        assert_eq!(nibbles_to_cell(0xA, 0xB), 0xAB);
    }

    #[test]
    fn test_new_opcode() {
        let (msb, lsb) = (5, 10);
        let new_opcode = Opcode::new(msb, lsb);
        assert_eq!(new_opcode.msb, msb);
        assert_eq!(new_opcode.lsb, lsb);
    }
}
