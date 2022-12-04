pub struct OpCode {
    pub op: u8,
    pub x: u8,
    pub y: u8,
    pub n: u8,
    pub nn: u8,
    pub nnn: u16,
}

impl OpCode {
    pub fn new(byte0: u8, byte1: u8) -> Self {
        Self {
            op: (byte0 & 0xF0) >> 4,
            x: byte0 & 0x0F,
            y: (byte1 & 0xF0) >> 4,
            n: byte1 & 0x0F,
            nn: byte1,
            nnn: ((byte0 & 0x0F) as u16) * 256 + byte1 as u16,
        }
    }
}

#[test]
fn test_new() {
    let opcode = OpCode::new(0xAB, 0xCD);
    assert_eq!(opcode.op, 0xA);
    assert_eq!(opcode.x, 0xB);
    assert_eq!(opcode.y, 0xC);
    assert_eq!(opcode.n, 0xD);
    assert_eq!(opcode.nn, 0xCD);
    assert_eq!(opcode.nnn, 0xBCD);
}
