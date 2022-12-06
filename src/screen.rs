use super::bit::Bit;

pub struct Screen {
    pixels: [[bool; 64]; 32], // 64X32 pixels
}

impl Screen {
    pub fn new() -> Self {
        Self {
            pixels: [[false; 64]; 32],
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
        for (j, pix8) in sprite.iter().enumerate() {
            for i in (0..8).rev() {
                self.pixels[y + j][x + (7 - i)] = pix8.bit(i);
            }
        }
        false // no collision
    }

    pub fn display(&self) {
        self.pixels.iter().for_each(|row| {
            let repr: String = row.iter().map(|&p| if p { 'x' } else { '-' }).collect();
            println!("{}", repr);
        });
    }
}
