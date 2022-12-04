use super::bit::Bit;
use super::opcode::OpCode;

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
    screen: [bool; 32 * 64],
    inputs: [bool; 16],
    pc: u16,
    index: u16,
    stack: u16,
    delay_timer: u8,
    sound_timer: u8,
    gp_vars: [u8; 16],
}

impl Emulator {
    pub fn new() -> Self {
        let mut memory = [0; 4096];
        write_fonts(&mut memory);

        Self {
            memory,
            screen: [false; 32 * 64],
            inputs: [false; 16],
            pc: 0x200,
            index: 0,
            stack: 0,
            delay_timer: 0,
            sound_timer: 0,
            gp_vars: [0; 16],
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
            // clear screen
            OpCode {
                op: 0,
                x: 0,
                y: 0xE,
                n: 0,
                ..
            } => self.op_clear_screen(),
            // jump
            OpCode { op: 1, nnn, .. } => {
                println!("jump pc from {:#X} to {:#X}", self.pc, nnn);
                self.pc = nnn;
            }
            // set register Vx
            OpCode { op: 6, x, nn, .. } => {
                println!("set register V{x:x} to {nn:X}");
                self.gp_vars[x as usize] = nn;
            }
            // add value to register Vx
            OpCode { op: 7, x, nn, .. } => {
                println!("add value {nn:#X} to register V{x:x}");
                self.gp_vars[x as usize] += nn;
            }
            // set index register
            OpCode { op: 0xA, nnn, .. } => {
                println!("set register index to {nnn:X}");
                self.index = nnn;
            }
            OpCode {
                op: 0xD, x, y, n, ..
            } => self.op_display(x, y, n),
            _ => (),
        }
    }

    fn op_display(&mut self, x: u8, y: u8, n: u8) {
        println!("display {x} {y} {n}");
        let n = n as usize;
        let idx = self.index as usize;
        let bytes = &self.memory[idx..(idx + n)];
        println!("disp bytes: {:?}", bytes);
        let vx = self.gp_vars[x as usize] as usize;
        let vy = self.gp_vars[y as usize] as usize;
        for b in bytes {
            let i = vx + vy * 64;
            for j in (0..8).rev() {
                self.screen[i + (7 - j)] = b.bit(j)
            }
        }
        self.print_screen();
    }

    fn op_clear_screen(&mut self) {
        println!("clear screen");
        for px in self.screen.iter_mut() {
            *px = false;
        }
    }

    fn fetch_opcode(&mut self) -> OpCode {
        let b0 = self.memory[self.pc as usize];
        let b1 = self.memory[self.pc as usize + 1];
        self.pc += 2;
        OpCode::new(b0, b1)
    }

    pub fn print_memory(&self) {
        let mem_pairs: Vec<_> = self.memory.iter().enumerate().collect();
        let mem_chunks = mem_pairs.chunks(8);
        for chunk in mem_chunks {
            println!(
                "{:#>4X}: {:>2X}, {:>2X}, {:>2X}, {:>2X}, {:>2X}, {:>2X}, {:>2X}, {:>2X}",
                chunk[0].0,
                chunk[0].1,
                chunk[1].1,
                chunk[2].1,
                chunk[3].1,
                chunk[4].1,
                chunk[5].1,
                chunk[6].1,
                chunk[7].1
            );
        }
    }

    pub fn print_screen(&self) {
        self.screen.chunks(64).for_each(|row| {
            let s: String = row.iter().map(|&b| if b { 'x' } else { '-' }).collect();
            println!("{}", s);
        })
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
