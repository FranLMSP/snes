pub fn rgb555_to_rgb888(rgb555: (u8, u8, u8)) -> (u8, u8, u8) {
    let red   = rgb555.0 & 0b11111;
    let green = rgb555.1 & 0b11111;
    let blue  = rgb555.2 & 0b11111;
    (
        (red << 3) | (red >> 2),
        (green << 3) | (green >> 2),
        (blue << 3) | (blue >> 2),
    )
}


#[cfg(test)]
mod color_tests {
    use super::*;

    #[test]
    fn test_rgb555_to_rgb888() {
        assert_eq!(rgb555_to_rgb888((0x00, 0x00, 0x00)), (0x00, 0x00, 0x00));
        assert_eq!(
            rgb555_to_rgb888((0b0001_1111, 0b0001_1111, 0b0001_1111)),
            (0xFF, 0xFF, 0xFF)
        );
        assert_eq!(
            rgb555_to_rgb888((0b10101, 0b01010, 0b11011)),
            (0xAD, 0x52, 0xDE)
        );
    }
}