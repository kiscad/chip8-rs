use super::bit::Bit;
use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};
use std::io::{stdout, Stdout, Write};

pub struct Screen {
    pixels: [[bool; 64]; 32], // 64X32 pixels
    buffer: Stdout,
}

impl Screen {
    pub fn new() -> Self {
        let mut buffer = stdout();
        buffer
            .execute(cursor::Hide)
            .unwrap()
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap();
        Self {
            pixels: [[false; 64]; 32],
            buffer,
        }
    }

    pub fn clear(&mut self) {
        for row in self.pixels.iter_mut() {
            for pix8 in row {
                *pix8 = false;
            }
        }
    }

    pub fn draw_sprite(&mut self, x: u8, y: u8, sprite: &[u8]) -> bool {
        let x = x as usize;
        let y = y as usize;
        let mut is_collison = false;
        for (j, pix8) in sprite.iter().enumerate() {
            for i in (0..8).rev() {
                let y_wrap = (y + j) % 32;
                let x_wrap = (x + (7 - i)) % 64;
                let px = &mut self.pixels[y_wrap][x_wrap];
                let px_new = (*px) ^ pix8.bit(i);
                is_collison = (*px) && !px_new; // old pixel is earsed.
                *px = px_new;
            }
        }
        is_collison
    }

    pub fn display(&mut self) {
        self.pixels.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, &px)| {
                if px {
                    self.buffer
                        .queue(cursor::MoveTo(2 * x as u16, y as u16))
                        .unwrap()
                        .queue(style::PrintStyledContent("█".magenta()))
                        .unwrap()
                        .queue(cursor::MoveTo(1 + 2 * x as u16, y as u16))
                        .unwrap()
                        .queue(style::PrintStyledContent("█".magenta()))
                        .unwrap();
                }
            })
        });
        self.buffer.flush().unwrap();
    }
}
