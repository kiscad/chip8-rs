// pub struct OpCode {
//     pub op: u8,
//     pub x: u8,
//     pub y: u8,
//     pub n: u8,
//     pub nn: u8,
//     pub nnn: u16,
// }

pub struct OpCode(
    pub u8,  // op
    pub u8,  // x
    pub u8,  // y
    pub u8,  // n
    pub u8,  // nn
    pub u16, // nnn
);

impl OpCode {
    pub fn new(byte0: u8, byte1: u8) -> Self {
        Self(
            (byte0 & 0xF0) >> 4,
            byte0 & 0x0F,
            (byte1 & 0xF0) >> 4,
            byte1 & 0x0F,
            byte1,
            ((byte0 & 0x0F) as u16) * 256 + byte1 as u16,
        )
    }

    // pub fn new(byte0: u8, byte1: u8) -> Self {
    //     Self {
    //         op: (byte0 & 0xF0) >> 4,
    //         x: byte0 & 0x0F,
    //         y: (byte1 & 0xF0) >> 4,
    //         n: byte1 & 0x0F,
    //         nn: byte1,
    //         nnn: ((byte0 & 0x0F) as u16) * 256 + byte1 as u16,
    //     }
    // }
}
