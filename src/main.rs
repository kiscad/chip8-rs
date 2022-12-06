mod bit;
mod emulator;
mod keypad;
mod opcode;
mod screen;

use emulator::Emulator;

fn main() {
    let mut machine = Emulator::new();
    machine.load_rom("./IBM_Logo.ch8");
    // machine.print_memory();
    // machine.print_screen();

    for _ in 0..40 {
        machine.cycle();
    }
}
