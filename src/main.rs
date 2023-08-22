mod interpreter;

use crate::interpreter::VirtualMachine;

fn main() {
    chip8_base::run(VirtualMachine::new());
}
