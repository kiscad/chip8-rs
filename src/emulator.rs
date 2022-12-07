use super::keypad::Keypad;
use super::opcode::OpCode;
use super::screen::Screen;

use rand::rngs::ThreadRng;
use rand::Rng;

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
    fonts_map: [u16; 16],
    rnd_gen: ThreadRng,
}

impl Emulator {
    pub fn new() -> Self {
        let mut memory = [0; 4096];
        let fonts_map = write_fonts(&mut memory);

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
            fonts_map,
            rnd_gen: rand::thread_rng(),
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
            // 00E0: CLS - clear screen
            OpCode(0, 0, 0xE, 0, _, _) => self.screen.clear(),
            // 00EE: RET - Return from a subroutine.
            OpCode(0, 0, 0xE, 0xE, ..) => {
                assert!(self.sp > 0);
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            }
            // 1NNN: JP addr - jump to location nnn.
            OpCode(1, .., nnn) => self.pc = nnn,
            // 2NNN: CALL addr - Call subroutine at NNN.
            OpCode(2, .., nnn) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            }
            // 3xnn: SE Vx, byte - Skip next instruction if Vx = nn
            OpCode(3, x, .., nn, _) => {
                if self.vx[x as usize] == nn {
                    self.inc_pc();
                }
            }
            // 4xnn: SNE Vx, byte - Skip next instruction if Vx != nn.
            OpCode(4, x, .., nn, _) => {
                if self.vx[x as usize] != nn {
                    self.inc_pc();
                }
            }
            // 5xy0: SE Vx, Vy - Skip next instruction if Vx = Vy.
            OpCode(5, x, y, 0, ..) => {
                if self.vx[x as usize] == self.vx[y as usize] {
                    self.inc_pc();
                }
            }
            // 6XNN: LD Vx, byte - Set Vx = nn
            OpCode(6, x, .., nn, _) => self.vx[x as usize] = nn,
            // 7XNN: ADD Vx, byte - Set Vx = Vx + nn
            OpCode(7, x, .., nn, _) => self.vx[x as usize] += nn,
            // 8XY0: LD Vx, Vy - Set Vx = Vy.
            OpCode(8, x, y, 0, ..) => self.vx[x as usize] = self.vx[y as usize],
            // 8XY1: OR Vx, Vy - Set Vx = Vx OR Vy.
            OpCode(8, x, y, 1, ..) => {
                self.vx[x as usize] = self.vx[x as usize] | self.vx[y as usize]
            }
            // 8XY2: AND Vx, Vy - Set Vx = Vx AND Vy.
            OpCode(8, x, y, 2, ..) => {
                self.vx[x as usize] = self.vx[x as usize] & self.vx[y as usize]
            }
            // 8XY3: XOR Vx, Vy - Set Vx = Vx XOR Vy.
            OpCode(8, x, y, 3, ..) => {
                self.vx[x as usize] = self.vx[x as usize] ^ self.vx[y as usize]
            }
            // 8XY4: ADD Vx, Vy - Set Vx = Vx + Vy, set VF = carry.
            OpCode(8, x, y, 4, ..) => {
                let x = x as usize;
                let y = y as usize;
                let (s, c) = self.vx[x].overflowing_add(self.vx[y]);
                self.vx[x] = s;
                self.vx[0xF] = c as u8;
            }
            // 8XY5: SUB Vx, Vy - Set Vx = Vx - Vy, set Vf = NOT borrow.
            OpCode(8, x, y, 5, ..) => {
                let brw;
                (self.vx[x as usize], brw) =
                    self.vx[x as usize].overflowing_sub(self.vx[y as usize]);
                self.vx[0xF] = !brw as u8;
            }
            // 8XY6: SHR Vx {, Vy} - Set Vx = Vx SHR 1.
            OpCode(8, x, _, 6, ..) => {
                self.vx[0xF] = self.vx[x as usize] & 0x1;
                self.vx[x as usize] /= 2;
            }
            // 8XY7: SUBN Vx, Vy - Set Vx = Vy - Vx, set VF = NOT borrow.
            OpCode(8, x, y, 7, ..) => {
                let brw;
                (self.vx[x as usize], brw) =
                    self.vx[y as usize].overflowing_sub(self.vx[x as usize]);
                self.vx[0xF] = !brw as u8;
            }
            // 8XYE: SHL Vx {, Vy} - Set Vx = Vx SHL 1.
            OpCode(8, x, _, 0xE, ..) => {
                self.vx[0xF] = self.vx[x as usize] & 0x80;
                (self.vx[x as usize], _) = self.vx[x as usize].overflowing_mul(2);
            }
            // 9XY0: SNE Vx, Vy - Skip next instruction if Vx != Vy.
            OpCode(9, x, y, 0, ..) => {
                if self.vx[x as usize] != self.vx[y as usize] {
                    self.inc_pc();
                }
            }
            // ANNN: LD I, addr - set index register
            OpCode(0xA, .., nnn) => self.ip = nnn,
            // BNNN: JP V0, addr - Jump to location NNN + V0
            OpCode(0xB, .., nnn) => self.pc = self.vx[0] as u16 + nnn,
            // CXNN: RND Vx, byte - Set Vx = random byte AND nn.
            OpCode(0xC, x, .., nn, _) => {
                let rnd = self.rnd_gen.gen_range(0..256) as u8;
                self.vx[x as usize] = rnd & nn;
            }
            // DXYN: Draw Vx, Vy, nibble - Display n-byte sprite starting at memory
            // location I at (Vx, Vy), set VF = collision.
            OpCode(0xD, x, y, n, ..) => self._dxyn(x, y, n),
            // EX9E: SKP Vx - Skip next instruction if key with the value of Vx is pressed.
            OpCode(0xE, x, 9, 0xE, ..) => {
                if self.keypad.is_pressed(self.vx[x as usize]) {
                    self.inc_pc();
                }
            }
            // EXA1: SKNP Vx - Skip next instruction if key with the value of Vx is not pressed.
            OpCode(0xE, x, 0xA, 1, ..) => {
                if !self.keypad.is_pressed(self.vx[x as usize]) {
                    self.inc_pc();
                }
            }
            // FX07: LD Vx, DT - Set Vx = delay timer value.
            OpCode(0xF, x, 0, 7, ..) => self.vx[x as usize] = self.dt,
            // FX0A: LD Vx, k - Wait for a key press, store the value of the key in Vx.
            OpCode(0xF, x, 0, 0xA, ..) => loop {
                if self.keypad.is_pressed(self.vx[x as usize]) {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(100)); // sleep 0.1 sec
            },
            // Fx15: LD DT, Vx - Set delay timer = Vx.
            OpCode(0xF, x, 1, 5, ..) => self.dt = self.vx[x as usize],
            // Fx18: LD ST, Vx - Set sound timer = Vx.
            OpCode(0xF, x, 1, 8, ..) => self.st = self.vx[x as usize],
            // FX1E: ADD I, Vx - Set I = I + Vx.
            OpCode(0xF, x, 1, 0xE, ..) => self.ip += self.vx[x as usize] as u16,
            // FX29: LD F, Vx - Set I = location of sprite for digit Vx.
            OpCode(0xF, x, 2, 9, ..) => self.ip = self.fonts_map[self.vx[x as usize] as usize],
            // FX33: LD B, Vx - Store BCD representation of Vx in memory locations I, I+1, and I+2.
            OpCode(0xF, x, 3, 3, ..) => {
                self.memory[self.ip as usize] = self.vx[x as usize] / 100;
                self.memory[self.ip as usize + 1] = (self.vx[x as usize] / 10) % 10;
                self.memory[self.ip as usize + 2] = self.vx[x as usize] % 10;
            }
            // FX55: LD [I], Vx - Store registers V0 through Vx in memory starting at location I.
            OpCode(0xF, x, 5, 5, ..) => {
                for i in 0..(x as usize + 1) {
                    self.memory[self.ip as usize + i] = self.vx[i];
                }
            }
            // FX65: LD Vx, [I] - Read registers V0 through Vx from memory starting at location I.
            OpCode(0xF, x, 6, 5, ..) => {
                for i in 0..(x as usize + 1) {
                    self.vx[i] = self.memory[self.ip as usize + i];
                }
            }
            _ => panic!("unknown op-code"),
        }
    }

    fn _dxyn(&mut self, x: u8, y: u8, n: u8) {
        let vx = self.vx[x as usize];
        let vy = self.vx[y as usize];
        let idx = self.ip as usize;
        let sprite = &self.memory[idx..(idx + n as usize)];
        let collision = self.screen.draw_sprite(vx, vy, sprite);
        self.vx[0xF] = collision as u8;
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

fn write_fonts(mem: &mut [u8; 4096]) -> [u16; 16] {
    let start_addr = 0x0;
    for (i, font) in FONTS.iter().enumerate() {
        for (j, &byte) in font.iter().enumerate() {
            let indx = start_addr + i * 5 + j;
            mem[indx] = byte;
        }
    }
    let mut fonts_map = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    fonts_map.iter_mut().for_each(|offset| {
        *offset *= 5;
        *offset += start_addr as u16;
    });
    fonts_map
}
