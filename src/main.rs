mod interpreter;

use crate::interpreter::VirtualMachine;
use std::time::Duration;

fn period_from_freq(freq: u64) -> Duration {
    Duration::from_micros(1000000 / freq)
}

fn main() {
    let speed: Duration = period_from_freq(700);
    let vm: VirtualMachine = VirtualMachine::new(speed);
    chip8_base::run(vm);
}
