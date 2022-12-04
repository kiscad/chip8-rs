pub trait Bit {
    fn bit(&self, idx: usize) -> bool;
}

impl Bit for u8 {
    fn bit(&self, idx: usize) -> bool {
        let mask = 0x1 << idx;
        (self & mask) >> idx == 1
    }
}

#[test]
fn test_u8_bit() {
    assert_eq!(0xA5u8.bit(0), true);
    assert_eq!(0xA5u8.bit(1), false);
    assert_eq!(0xA5u8.bit(2), true);
    assert_eq!(0xA5u8.bit(3), false);
    assert_eq!(0xA5u8.bit(4), false);
    assert_eq!(0xA5u8.bit(5), true);
    assert_eq!(0xA5u8.bit(6), false);
    assert_eq!(0xA5u8.bit(7), true);
}
