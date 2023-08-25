mod interpreter;

use crate::interpreter::VirtualMachine;
use std::time::Duration;

fn period_from_freq(freq: u64) -> Duration {
    Duration::from_micros(1000000 / freq)
}

fn main() {
    env_logger::init();

    let speed: Duration = period_from_freq(700);
    let mut vm: VirtualMachine = VirtualMachine::new(speed);

    vm.load(String::from("./roms/uwcs.ch8"));

    chip8_base::run(vm);
}
