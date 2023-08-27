mod interpreter;

use interpreter::{Frequency, VirtualMachine};

fn main() {
    env_logger::init();
    let clock = Frequency(700);
    let vm: VirtualMachine = VirtualMachine::new(clock).load_rom("./roms/uwcs.ch8");
    chip8_base::run(vm);
}
