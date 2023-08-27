mod instructions;

use chip8_base::{Display, Interpreter, Keys, Pixel};
use instructions::{Instruction, Opcode};
use log::debug;
use std::time::Duration;

pub const MEM_SIZE: usize = 4096;
pub const STACK_LIMIT: usize = 16;
pub const NUM_REG: usize = 16;

#[derive(Debug)]
pub struct VirtualMachine {
    memory: [u8; MEM_SIZE],   // Addressable memory
    pc: u16,                  // Program counter
    registers: [u8; NUM_REG], // Vx registers
    mar: u16,                 // I (memory address) register
    stack: Vec<u16>,          // Stack memory
    delay_timer: u8,          // Delay timer
    sound_timer: u8,          // Sound timer
    display: Display,         // Display output
    speed: Duration,          // Clock period
}

impl Interpreter for VirtualMachine {
    fn step(&mut self, keys: &Keys) -> Option<Display> {
        debug!("Program Counter: {}", self.pc);
        let opcode = self.fetch();
        debug!("Opcode: {:?}", opcode);
        let instruction = Instruction::decode(opcode);
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
    pub fn new(clock: Frequency) -> Self {
        Self {
            memory: [0; MEM_SIZE],
            mar: 0,
            pc: 0,
            registers: [0; NUM_REG],
            stack: vec![0; STACK_LIMIT],
            delay_timer: 0,
            sound_timer: 0,
            display: empty_display(),
            speed: clock.into(),
        }
    }

    pub fn load_rom(mut self, filename: &str) -> Self {
        let program = std::fs::read(filename).expect("Unable to read ROM file!");
        self.memory[0x200..(0x200 + program.len())].copy_from_slice(&program);
        self.pc = 0x200;
        self
    }

    fn fetch(&mut self) -> Opcode {
        let opcode = Opcode(
            self.memory[self.pc as usize],
            self.memory[(self.pc + 1) as usize],
        );
        self.increment_pc();
        opcode
    }

    fn increment_pc(&mut self) {
        self.pc += 2;
        self.pc &= (MEM_SIZE - 1) as u16;
    }

    fn draw(&mut self, x: u8, y: u8, n: u8) {
        let x_offset = (self.registers[x as usize] % 64) as usize;
        let y_offset = (self.registers[y as usize] % 32) as usize;
        self.registers[0x0F] = 0;

        let sprite_addr = self.mar as usize;
        let sprite = &self.memory[sprite_addr..(sprite_addr + n as usize)];
        for (row_num, row) in sprite.iter().enumerate() {
            let y_pos = y_offset + row_num;
            if y_pos > 31 {
                break;
            }
            for pixel_num in 0..8 {
                let x_pos = x_offset + pixel_num;
                if x_pos > 63 {
                    break;
                }
                if nth_bit(*row, pixel_num) == true {
                    if self.display[y_pos][x_pos] == Pixel::White {
                        self.display[y_pos][x_pos] = Pixel::Black;
                        self.registers[0x0F] = 1;
                    } else {
                        self.display[y_pos][x_pos] = Pixel::White;
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

pub struct Frequency(pub u32);
impl Into<Duration> for Frequency {
    fn into(self) -> Duration {
        Duration::from_secs_f64(1_f64 / self.0 as f64)
    }
}

fn empty_display() -> [[Pixel; 64]; 32] {
    [[Pixel::Black; 64]; 32]
}

fn nth_bit(byte: u8, n: usize) -> bool {
    (byte & (1 << (7 - n))) != 0
}
