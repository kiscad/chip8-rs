use super::keypad::Keypad;
use super::opcode::OpCode;
use super::screen::Screen;

const FONTS: [[u8; 5]; 16] = [
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
    [0x20, 0x60, 0x20, 0x20, 0x70], // 1
    [0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
    [0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
    [0x90, 0x90, 0xF0, 0x10, 0x10], // 4
    [0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
    [0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
    [0xF0, 0x10, 0x20, 0x40, 0x40], // 7
    [0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
    [0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
    [0xF0, 0x90, 0xF0, 0x90, 0x90], // A
    [0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
    [0xF0, 0x80, 0x80, 0x80, 0xF0], // C
    [0xE0, 0x90, 0x90, 0x90, 0xE0], // D
    [0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
    [0xF0, 0x80, 0xF0, 0x80, 0x80], // F
];

pub struct Emulator {
    memory: [u8; 4096],
    pc: u16,
    ip: u16,      // index pointer
    dt: u8,       // delay timer
    st: u8,       // sound timer
    vx: [u8; 16], // general-purpose register
    sp: u8,       // stack pointer
    stack: [u16; 16],
    screen: Screen,
    keypad: Keypad,
}

impl Emulator {
    pub fn new() -> Self {
        let mut memory = [0; 4096];
        write_fonts(&mut memory);

        Self {
            memory,
            pc: 0x200,
            ip: 0,
            dt: 0,
            st: 0,
            vx: [0; 16],
            sp: 0,
            stack: [0; 16],
            screen: Screen::new(),
            keypad: Keypad::new(),
        }
    }

    pub fn load_rom(&mut self, path: &str) {
        use std::fs;
        let rom = fs::read(path).unwrap();
        for i in 0..rom.len() {
            self.memory[0x200 + i] = rom[i];
        }
    }

    pub fn cycle(&mut self) {
        let opcode = self.fetch_opcode();
        self.exec_opcode(opcode);
    }

    fn exec_opcode(&mut self, opcode: OpCode) {
        match opcode {
            // 00E0: clear screen
            OpCode(0, 0, 0xE, 0, _, _) => self.screen.clear(),
            // 1NNN: jump
            OpCode(1, .., nnn) => self.pc = nnn,
            // 6XNN: set register Vx
            OpCode(6, x, .., nn, _) => self.vx[x as usize] = nn,
            // 7XNN: add value to register Vx
            OpCode(7, x, .., nn, _) => self.vx[x as usize] += nn,
            // ANNN: set index register
            OpCode(0xA, .., nnn) => self.ip = nnn,
            // DXYN: Draw
            OpCode(0xD, x, y, n, ..) => self._dxyn(x, y, n),
            _ => (),
        }
    }

    fn _dxyn(&mut self, x: u8, y: u8, n: u8) {
        let vx = self.vx[x as usize];
        let vy = self.vx[y as usize];
        // println!("Dxyn: {vx}, {vy}, {n}");
        let idx = self.ip as usize;
        let sprite = &self.memory[idx..(idx + n as usize)];
        self.screen.draw_sprite(vx, vy, sprite);
        self.screen.display();
    }

    fn inc_pc(&mut self) {
        self.pc += 2;
    }

    fn fetch_opcode(&mut self) -> OpCode {
        let b0 = self.memory[self.pc as usize];
        let b1 = self.memory[self.pc as usize + 1];
        self.inc_pc();
        OpCode::new(b0, b1)
    }
}

fn write_fonts(mem: &mut [u8; 4096]) {
    for (i, font) in FONTS.iter().enumerate() {
        for (j, &byte) in font.iter().enumerate() {
            let indx = 0x0 + i * 5 + j;
            mem[indx] = byte;
        }
    }
}
