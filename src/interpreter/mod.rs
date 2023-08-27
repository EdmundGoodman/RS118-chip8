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

fn empty_display() -> [[Pixel; 64]; 32] {
    [[Pixel::Black; 64]; 32]
}

fn speed_from_frequency(frequency: u32) -> Duration {
    Duration::from_secs_f64(1_f64 / frequency as f64)
}

impl VirtualMachine {
    pub fn new(frequency: u32) -> Self {
        Self {
            memory: [0; MEM_SIZE],
            mar: 0,
            pc: 0,
            registers: [0; NUM_REG],
            stack: vec![0; STACK_LIMIT],
            delay_timer: 0,
            sound_timer: 0,
            display: empty_display(),
            speed: speed_from_frequency(frequency),
        }
    }

    pub fn load_rom(mut self, filename: &str) -> Self {
        let program = std::fs::read(filename).expect("Unable to read ROM file!");
        self.memory[0x200..(0x200 + program.len())].copy_from_slice(&program);
        self.pc = 0x200;
        self
    }

    fn fetch(&mut self) -> Opcode {
        // Could this be an array slice instead?
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
        // if self.pc + 2 < MEM_SIZE as u16 {
        //     self.pc += 2;
        //     return;
        // }
        // self.pc = 0;
    }

    fn draw(&mut self, x: u8, y: u8, n: u8) {
        let x_pos = self.registers[x as usize] % 64;
        let y_pos = self.registers[y as usize] % 32;
        self.registers[0x0F] = 0;

        for row_num in 0..n {
            let cur_y = y_pos + row_num;
            if cur_y >= 32 {
                break;
            }
            let row_addr = (self.mar + row_num as u16) % (MEM_SIZE as u16);
            let row_memory = self.memory[row_addr as usize];
            for pixel_num in 0..8 {
                let cur_x = x_pos + pixel_num;
                if cur_x >= 64 {
                    break;
                }
                let pixel_memory = (row_memory & (1 << (7 - pixel_num))) != 0;
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

        // let tlx = self.registers[x as usize] % 64;
        // let tly = self.registers[y as usize] % 32;
        // self.registers[0xF] = 0;
        // let ind = self.mar as usize;
        // let sprite = &self.memory[ind..(ind + n as usize)];

        // for (i, row) in sprite.iter().enumerate() {
        //     let pxy = tly + i as u8;
        //     if pxy > 31 {
        //         break;
        //     }

        //     for j in 0..8 {
        //         let pxx = tlx + j;
        //         if pxx > 63 {
        //             break;
        //         }
        //         let old_px = &mut self.display[pxy as usize][pxx as usize];
        //         let mask = 2_u8.pow(7 - j as u32);
        //         let new_u8 = (row & mask) >> (7 - j);
        //         let new_px: chip8_base::Pixel = new_u8.try_into().unwrap();
        //         if (new_px & *old_px).into() {
        //             // if collision
        //             self.registers[0xF] = 1
        //         }
        //         *old_px ^= new_px;
        //     }
        // }
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
