mod instructions;

use chip8_base::{Display, Interpreter, Keys, Pixel};
use instructions::{decode, Instruction, Opcode};
use log::debug;
use std::time::Duration;

pub const MEM_SIZE: usize = 4096;
pub const STACK_LIMIT: usize = 16;
pub const NUM_REG: usize = 16;

type Data = u8;
type Instr = u16;
type Addr = u16;

#[derive(Debug)]
pub struct VirtualMachine {
    pub memory: [Data; MEM_SIZE],   // Addressable memory
    pub pc: Addr,                   // Program counter
    pub registers: [Data; NUM_REG], // Vx registers
    pub mar: Addr,                  // I (memory address) register
    pub stack: [Addr; STACK_LIMIT], // Stack memory
    pub stack_pointer: Data,        // Stack pointer
    pub delay_timer: Data,          // Delay timer
    pub sound_timer: Data,          // Sound timer
    pub display: Display,           // Display output
    speed: Duration,                // Clock period
}

fn empty_display() -> [[Pixel; 64]; 32] {
    [[Pixel::White; 64]; 32]
}

impl VirtualMachine {
    pub fn new(speed: Duration) -> VirtualMachine {
        VirtualMachine {
            memory: [0; MEM_SIZE],
            mar: 0,
            pc: 0,
            registers: [0; NUM_REG],
            stack: [0; STACK_LIMIT],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            display: empty_display(),
            speed,
        }
    }
}

impl Interpreter for VirtualMachine {
    fn step(&mut self, keys: &Keys) -> Option<Display> {
        debug!("Program Counter: {}", self.pc);

        let opcode = self.fetch();
        debug!("Opcode: {:?}", opcode);

        let instruction = decode(opcode);
        debug!("Instruction {:?}", instruction);

        self.execute(instruction)
    }

    fn speed(&self) -> Duration {
        self.speed
    }

    fn buzzer_active(&self) -> bool {
        false
    }
}

impl VirtualMachine {
    fn fetch(&mut self) -> Opcode {
        // Could this be an array slice instead?
        let opcode = Opcode::new(
            self.memory[usize::from(self.pc)],
            self.memory[usize::from(self.pc + 1)],
        );
        self.increment_pc();
        opcode
    }

    fn increment_pc(&mut self) {
        if self.pc + 2 < MEM_SIZE as Addr {
            self.pc += 2;
            return;
        }
        self.pc = 0;
    }

    fn execute(&mut self, instruction: Instruction) -> Option<Display> {
        match instruction {
            Instruction::NOP => (),
            Instruction::CLS => {
                self.display = empty_display();
                return Some(self.display);
            }
        }
        None
    }
}
