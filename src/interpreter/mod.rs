mod instructions;

use chip8_base::{Display, Interpreter, Keys, Pixel};
use instructions::{Instruction, Opcode};
use log::debug;
use std::fs;
use std::time::Duration;

pub const MEM_SIZE: usize = 4096;
pub const STACK_LIMIT: usize = 16;
pub const NUM_REG: usize = 16;

// These could/should be wrapped newtypes?
// https://doc.rust-lang.org/stable/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
type Nibble = u8;
type Cell = u8;
type Addr = u16;

#[derive(Debug)]
pub struct VirtualMachine {
    pub memory: [Cell; MEM_SIZE],   // Addressable memory
    pub pc: Addr,                   // Program counter
    pub registers: [Cell; NUM_REG], // Vx registers
    pub mar: Addr,                  // I (memory address) register
    pub stack: [Addr; STACK_LIMIT], // Stack memory
    pub stack_pointer: Cell,        // Stack pointer
    pub delay_timer: Cell,          // Delay timer
    pub sound_timer: Cell,          // Sound timer
    pub display: Display,           // Display output
    speed: Duration,                // Clock period
}

fn empty_display() -> [[Pixel; 64]; 32] {
    [[Pixel::Black; 64]; 32]
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

    pub fn load(&mut self, rom_path: String) {
        let mut rom = fs::read(rom_path).expect("Unable to read ROM file!");
        rom.resize(MEM_SIZE, 0);
        let rom_arr: [Cell; MEM_SIZE] = rom.try_into().expect("Unable to load ROM!");
        self.memory = rom_arr;
        self.pc = 0x200;
    }
}

impl Interpreter for VirtualMachine {
    fn step(&mut self, keys: &Keys) -> Option<Display> {
        debug!("Program Counter: {}", self.pc);

        let opcode = self.fetch();
        debug!("Opcode: {:?}", opcode);

        let instruction = opcode.decode();
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
            self.memory[self.pc as usize],
            self.memory[(self.pc + 1) as usize],
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

    fn draw(&mut self, x: u8, y: u8, n: u8) {
        let x_pos = self.registers[x as usize] % 64;
        let y_pos = self.registers[y as usize] % 32;
        self.registers[0x0F] = 0;

        for row_num in 0..n {
            let cur_y = y_pos + row_num;
            if cur_y >= 31 {
                break;
            }
            let row_addr = (self.mar + row_num as Addr) % (MEM_SIZE as Addr);
            let row_memory = self.memory[row_addr as usize];
            for pixel_num in 0..8 {
                let cur_x = x_pos + pixel_num;
                if cur_x >= 63 {
                    break;
                }
                let pixel_memory = row_memory & (1 << pixel_num) != 0;
                if pixel_memory == true {
                    if self.display[cur_y as usize][cur_x as usize] == Pixel::White {
                        self.display[cur_y as usize][cur_x as usize] = Pixel::Black;
                        self.registers[0x0F] = 1;
                    } else {
                        self.display[cur_y as usize][cur_x as usize] = Pixel::White;
                    }
                }
            }
        }
    }

    fn execute(&mut self, instruction: Instruction) -> Option<Display> {
        match instruction {
            Instruction::NOP => (),
            Instruction::CLS => {
                self.display = empty_display();
                return Some(self.display);
            }
            Instruction::JP(addr) => {
                self.pc = addr;
            }
            Instruction::LD(reg, data) => {
                self.registers[reg as usize] = data;
            }
            Instruction::ADD(reg, data) => {
                self.registers[reg as usize] = self.registers[reg as usize].wrapping_add(data);
            }
            Instruction::LDI(addr) => {
                self.mar = addr;
            }
            Instruction::DRW(x, y, n) => {
                self.draw(x, y, n);
                return Some(self.display);
            }
        }
        None
    }
}
