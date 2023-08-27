mod interpreter;

use crate::interpreter::VirtualMachine;

fn main() {
    env_logger::init();

    let vm: VirtualMachine = VirtualMachine::new(700).load("./roms/uwcs.ch8");

    chip8_base::run(vm);
}
