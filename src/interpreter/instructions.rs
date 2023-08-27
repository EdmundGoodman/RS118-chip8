#[derive(Debug)]
pub enum Instruction {
    NOP,
    CLS,
    JP(u16),
    LD(u8, u8),
    ADD(u8, u8),
    LDI(u16),
    DRW(u8, u8, u8),
}

impl Instruction {
    pub fn decode(opcode: Opcode) -> Self {
        match opcode.as_nibbles() {
            (0, 0, 0, 0) => Self::NOP,
            (0, 0, 0xE, 0) => Self::CLS,
            (1, n1, n2, n3) => Self::JP(nibbles_to_address(n1, n2, n3)),
            (6, x, k1, k2) => Self::LD(x, nibbles_to_data(k1, k2)),
            (7, x, k1, k2) => Self::ADD(x, nibbles_to_data(k1, k2)),
            (0xA, n1, n2, n3) => Self::LDI(nibbles_to_address(n1, n2, n3)),
            (0xD, x, y, n) => Self::DRW(x, y, n),
            _ => todo!("Unimplemented instruction!"),
        }
    }
}

#[derive(Debug)]
pub struct Opcode(pub u8, pub u8);

impl Opcode {
    fn as_nibbles(&self) -> (u8, u8, u8, u8) {
        return (
            first_nibble(self.0),
            second_nibble(self.0),
            first_nibble(self.1),
            second_nibble(self.1),
        );
    }
}

fn first_nibble(byte: u8) -> u8 {
    (byte & 0xF0) >> 4
}

fn second_nibble(byte: u8) -> u8 {
    byte & 0x0F
}

fn nibbles_to_address(n1: u8, n2: u8, n3: u8) -> u16 {
    ((n1 as u16) << 8) | ((n2 as u16) << 4) | (n3 as u16)
}

fn nibbles_to_data(n1: u8, n2: u8) -> u8 {
    (n1 << 4) | n2
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
    fn test_nibbles_to_address() {
        assert_eq!(nibbles_to_address(0xA, 0xB, 0xC), 0xABC);
    }

    #[test]
    fn test_nibbles_to_data() {
        assert_eq!(nibbles_to_data(0xA, 0xB), 0xAB);
    }

    #[test]
    fn test_new_opcode() {
        let (msb, lsb) = (0xAB, 0xCD);
        let new_opcode = Opcode(msb, lsb);
        assert_eq!(new_opcode.0, msb);
        assert_eq!(new_opcode.1, lsb);
        assert_eq!(new_opcode.as_nibbles(), (0xA, 0xB, 0xC, 0xD));
    }
}
