use std::time::Duration;

pub const MEM_SIZE: usize = 4096;
pub const STACK_SIZE: usize = 16;
pub const NUM_GP_REG: usize = 16;

pub struct VirtualMachine {
    pub mem: [u8; MEM_SIZE],          // Addressable memory
    pub stack: [u16; STACK_SIZE],     // Stack memory
    pub vxs: [u8; NUM_GP_REG],        // Vx registers (VF as flag)
    pub i: u16,                       // I register
    pub pc: u16,                      // Program counter
    pub sp: u8,                       // Stack pointer
    pub dt: u8,                       // Display timer
    pub st: u8,                       // Sound timer
    pub display: chip8_base::Display, // Display output
    speed: Duration,                  // Clock period
}

impl VirtualMachine {
    pub fn new(speed: Duration) -> VirtualMachine {
        VirtualMachine {
            mem: [0; MEM_SIZE],
            stack: [0; STACK_SIZE],
            vxs: [0; NUM_GP_REG],
            i: 0,
            pc: 0,
            sp: 0,
            dt: 0,
            st: 0,
            display: [[chip8_base::Pixel::Black; 64]; 32],
            speed,
        }
    }
}

impl chip8_base::Interpreter for VirtualMachine {
    fn step(&mut self, keys: &chip8_base::Keys) -> Option<chip8_base::Display> {
        None
    }

    fn speed(&self) -> Duration {
        self.speed
    }

    fn buzzer_active(&self) -> bool {
        false
    }
}
