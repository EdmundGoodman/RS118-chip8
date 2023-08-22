use std::time::Duration;

pub const MEM_SIZE: usize = 4096;
pub const NUM_GP_REG: usize = 16;

pub struct VirtualMachine {
    pub mem: [u8; MEM_SIZE],   // Addressable memory
    pub vxs: [u8; NUM_GP_REG], // Vx registers
    pub i: u16,                // I register
    pub dt: u8,                // Display timer
    pub st: u8,                // Sound timer
    pub display: chip8_base::Display,
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            mem: [0; MEM_SIZE],
            vxs: [0; NUM_GP_REG],
            i: 0,
            dt: 0,
            st: 0,
            display: [[chip8_base::Pixel::Black; 64]; 32],
        }
    }
}

impl chip8_base::Interpreter for VirtualMachine {
    fn step(&mut self, keys: &chip8_base::Keys) -> Option<chip8_base::Display> {
        None
    }

    fn speed(&self) -> std::time::Duration {
        Duration::new(1, 0)
    }

    fn buzzer_active(&self) -> bool {
        false
    }
}
