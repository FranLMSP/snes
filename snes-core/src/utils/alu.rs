use crate::utils::num_trait::SnesNum;
use crate::common::flags::{Flags, Flags::*};

pub fn adc_bin<T: SnesNum>(target: T, value: T, carry: bool) -> (T, [Flags; 4]) {
    let result = target.add_snes(value, carry);
    (result, [
        Negative(result.is_negative()),
        Overflow(target.is_overflow(value, result)),
        Zero(result.is_zero()),
        Carry(target.add_will_carry(value, carry)),
    ])
}

pub fn adc_bcd<T: SnesNum>(target: T, value: T, carry: bool) -> (T, [Flags; 4]) {
    let original_target = target;
    let original_value = value;
    let nibble_bytes = target.bytes() * 2;
    let mut is_carry = carry;
    let target = target.to_u32();
    let value = value.to_u32();
    let mut result = 0;

    let mut value_mask = 0x0F;
    let mut result_mask = 0x00;
    let mut carry_compare = 0x09;
    let mut bcd_six_add = 0x06;
    let mut carry_shift = 0;

    let mut is_overflow = false;
    for _ in 0..nibble_bytes {
        result = (target & value_mask) + (value & value_mask) + ((is_carry as u32) << carry_shift) + (result & result_mask);
        is_overflow = original_target.is_overflow(original_value, T::from_u32(result));
        if result > carry_compare {
            result = result.wrapping_add(bcd_six_add);
        }
        is_carry = result > carry_compare;
        value_mask <<= 4;
        bcd_six_add <<= 4;
        carry_shift += 4;
        result_mask = (result_mask << 4) | 0xF;
        carry_compare = (carry_compare << 4) | 0xF;
    }

    let result = T::from_u32(result);
    (result, [
        Negative(result.is_negative()),
        Overflow(is_overflow),
        Zero(result.is_zero()),
        Carry(is_carry),
    ])
}

pub fn sbc_bin<T: SnesNum>(target: T, value: T, carry: bool) -> (T, [Flags; 4]) {
    adc_bin(target, value.invert(), carry)
}

pub fn sbc_bcd<T: SnesNum>(target: T, value: T, carry: bool) -> (T, [Flags; 4]) {
    if target.bytes() == 1 {
        let target = ((target.to_u32() as u16) as i16) & 0xFF;
        let value = !(value.to_u32() as u16) as i16;
        let mut carry = if carry {1} else {0};
        let mut result: i16 = (target & 0x000F) + (value & 0x000F) + carry;
        if result <= 0x000F {
            result -= 0x06;
        }
        carry = if result > 0x000F {1} else {0};
        result = (target & 0x00F0) + (value & 0x00F0) + (carry << 4) + (result & 0x000F);
        let is_overflow = !(target ^ value) & (target ^ result) & 0x80 != 0;
        if result <= 0x00FF {
            result -= 0x60;
        }
        let is_carry = result > 0xFF;
        let result = T::from_u32(result as u32);
        (result, [
            Negative(result.is_negative()),
            Overflow(is_overflow),
            Zero(result.is_zero()),
            Carry(is_carry),
        ])
    } else {
        let target = ((target.to_u32() as u16) as i32) & 0xFFFF;
        let value = !(value.to_u32() as u16) as i32;
        let mut carry = if carry {1} else {0};
        let mut result: i32 = (target & 0x000F) + (value & 0x000F) + carry;
        if result <= 0x000F {
            result -= 0x0006;
        }
        carry = if result > 0x000F {1} else {0};
        result = (target & 0x00F0) + (value & 0x00F0) + (carry << 4) + (result & 0x000F);
        if result <= 0x00FF {
            result -= 0x0060;
        }

        carry = if result > 0x00FF {1} else {0};
        result = (target & 0x0F00) + (value & 0x0F00) + (carry << 8) + (result & 0x00FF);
        if result <= 0x0FFF {
            result -= 0x0600;
        }
        carry = if result > 0x0FFF {1} else {0};
        result = (target & 0xF000) + (value & 0xF000) + (carry << 12) + (result & 0x0FFF);
        let is_overflow = !(target ^ value) & (target ^ result) & 0x8000 != 0;
        if result <= 0xFFFF {
            result -= 0x6000;
        }
        let is_carry = result > 0xFFFF;
        let result = T::from_u32((result) as u32);
        (result, [
            Negative(result.is_negative()),
            Overflow(is_overflow),
            Zero(result.is_zero()),
            Carry(is_carry),
        ])
    }
}

pub fn and<T: SnesNum>(target: T, value: T) -> (T, [Flags; 2]) {
    let result = target.and(value);
    (result, [Negative(result.is_negative()), Zero(result.is_zero())])
}

pub fn asl<T: SnesNum>(target: T) -> (T, [Flags; 3]) {
    let result = target.asl();
    (result, [
        Negative(result.is_negative()),
        Zero(result.is_zero()),
        Carry(target.is_negative()), // High bit becomes carry
    ])
}

pub fn lsr<T: SnesNum>(target: T) -> (T, [Flags; 3]) {
    let result = target.lsr();
    (result, [
        Negative(false),
        Zero(result.is_zero()),
        Carry(target.to_u32() & 1 == 1), // Low bit becomes carry
    ])
}

pub fn eor<T: SnesNum>(target: T, value: T) -> (T, [Flags; 2]) {
    let result = target.xor(value);
    (result, [
        Negative(result.is_negative()),
        Zero(result.is_zero()),
    ])
}

pub fn ora<T: SnesNum>(target: T, value: T) -> (T, [Flags; 2]) {
    let result = target.ora(value);
    (result, [
        Negative(result.is_negative()),
        Zero(result.is_zero()),
    ])
}

pub fn rol<T: SnesNum>(target: T, carry: bool) -> (T, [Flags; 3]) {
    let result = target.rol(carry);
    (result, [
        Negative(result.is_negative()),
        Zero(result.is_zero()),
        Carry(target.is_negative()),
    ])
}

pub fn ror<T: SnesNum>(target: T, carry: bool) -> (T, [Flags; 3]) {
    let result = target.ror(carry);
    (result, [
        Negative(result.is_negative()),
        Zero(result.is_zero()),
        Carry(target.lowest_bit()),
    ])
}


#[cfg(test)]
mod alu_tests {
    use super::*;

    #[test]
    fn test_adc_bin() {
        // 8 bits
        let (result, affected_flags) = adc_bin(0_u8, 0_u8, false);
        assert_eq!(result, 0);
        assert_eq!(affected_flags, [Negative(false), Overflow(false), Zero(true), Carry(false)]);

        let (result, affected_flags) = adc_bin(0_u8, 50_u8, false);
        assert_eq!(result, 50);
        assert_eq!(affected_flags, [Negative(false), Overflow(false), Zero(false), Carry(false)]);

        let (result, affected_flags) = adc_bin(200_u8, 155_u8, false);
        assert_eq!(result, 99);
        assert_eq!(affected_flags, [Negative(false), Overflow(true), Zero(false), Carry(true)]);

        let (result, affected_flags) = adc_bin(200_u8, 155_u8, true);
        assert_eq!(result, 100);
        assert_eq!(affected_flags, [Negative(false), Overflow(true), Zero(false), Carry(true)]);

        let (result, affected_flags) = adc_bin(200_u8, 54_u8, true);
        assert_eq!(result, 255);
        assert_eq!(affected_flags, [Negative(true), Overflow(false), Zero(false), Carry(false)]);

        let (result, affected_flags) = adc_bin(200_u8, 54_u8, true);
        assert_eq!(result, 255);
        assert_eq!(affected_flags, [Negative(true), Overflow(false), Zero(false), Carry(false)]);

        let (result, affected_flags) = adc_bin(0x7F_u8, 0x81_u8, false);
        assert_eq!(result, 0x00);
        assert_eq!(affected_flags, [Negative(false), Overflow(false), Zero(true), Carry(true)]);

        let (result, affected_flags) = adc_bin(0x7F_u8, 0x7F_u8, true);
        assert_eq!(result, 0xFF);
        assert_eq!(affected_flags, [Negative(true), Overflow(true), Zero(false), Carry(false)]);

        // 16 bits
        let (result, affected_flags) = adc_bin(0_u16, 0_u16, false);
        assert_eq!(result, 0);
        assert_eq!(affected_flags, [Negative(false), Overflow(false), Zero(true), Carry(false)]);

        let (result, affected_flags) = adc_bin(0_u16, 50_u16, false);
        assert_eq!(result, 50);
        assert_eq!(affected_flags, [Negative(false), Overflow(false), Zero(false), Carry(false)]);

        let (result, affected_flags) = adc_bin(65530_u16, 10_u16, false);
        assert_eq!(result, 4);
        assert_eq!(affected_flags, [Negative(false), Overflow(false), Zero(false), Carry(true)]);

        let (result, affected_flags) = adc_bin(65530_u16, 10_u16, true);
        assert_eq!(result, 5);
        assert_eq!(affected_flags, [Negative(false), Overflow(false), Zero(false), Carry(true)]);

        let (result, affected_flags) = adc_bin(65530_u16, 4_u16, true);
        assert_eq!(result, 65535);
        assert_eq!(affected_flags, [Negative(true), Overflow(false), Zero(false), Carry(false)]);
    }

    #[test]
    fn test_adc_bcd() {
        // 8 bits
        let (result, affected_flags) = adc_bcd(5_u8, 5_u8, false);
        assert_eq!(result, 0b0001_0000);
        assert_eq!(affected_flags, [Negative(false), Overflow(false), Zero(false), Carry(false)]);

        let (result, affected_flags) = adc_bcd(7_u8, 9_u8, false);
        assert_eq!(result, 0b0001_0110);
        assert_eq!(affected_flags, [Negative(false), Overflow(false), Zero(false), Carry(false)]);

        let (result, affected_flags) = adc_bcd(5_u8, 4_u8, true);
        assert_eq!(result, 0b0001_0000);
        assert_eq!(affected_flags, [Negative(false), Overflow(false), Zero(false), Carry(false)]);

        let (result, affected_flags) = adc_bcd(7_u8, 8_u8, true);
        assert_eq!(result, 0b0001_0110);
        assert_eq!(affected_flags, [Negative(false), Overflow(false), Zero(false), Carry(false)]);

        let (result, affected_flags) = adc_bcd(0_u8, 0_u8, false);
        assert_eq!(result, 0);
        assert_eq!(affected_flags, [Negative(false), Overflow(false), Zero(true), Carry(false)]);

        let (result, affected_flags) = adc_bcd(0b0001_1001_u8, 0b0010_1000_u8, false);
        assert_eq!(result, 0b0100_0111);
        assert_eq!(affected_flags, [Negative(false), Overflow(false), Zero(false), Carry(false)]);

        // 16 bits
        let (result, affected_flags) = adc_bcd(5_u16, 5_u16, false);
        assert_eq!(result, 0b0001_0000);
        assert_eq!(affected_flags, [Negative(false), Overflow(false), Zero(false), Carry(false)]);

        let (result, affected_flags) = adc_bcd(7_u16, 9_u16, false);
        assert_eq!(result, 0b0001_0110);
        assert_eq!(affected_flags, [Negative(false), Overflow(false), Zero(false), Carry(false)]);

        let (result, affected_flags) = adc_bcd(5_u16, 4_u16, true);
        assert_eq!(result, 0b0001_0000);
        assert_eq!(affected_flags, [Negative(false), Overflow(false), Zero(false), Carry(false)]);

        let (result, affected_flags) = adc_bcd(7_u16, 8_u16, true);
        assert_eq!(result, 0b0001_0110);
        assert_eq!(affected_flags, [Negative(false), Overflow(false), Zero(false), Carry(false)]);

        let (result, affected_flags) = adc_bcd(0_u16, 0_u16, false);
        assert_eq!(result, 0);
        assert_eq!(affected_flags, [Negative(false), Overflow(false), Zero(true), Carry(false)]);

        let (result, affected_flags) = adc_bcd(0x0500_u16, 0x0500_u16, false);
        assert_eq!(result, 0b0001_0000_0000_0000);
        assert_eq!(affected_flags, [Negative(false), Overflow(false), Zero(false), Carry(false)]);

        let (result, affected_flags) = adc_bcd(0b0001_1001_u16, 0b0010_1000_u16, false);
        assert_eq!(result, 0b0100_0111);
        assert_eq!(affected_flags, [Negative(false), Overflow(false), Zero(false), Carry(false)]);
    }

    #[test]
    fn test_dec_bin() {
        // 8 bit
        let (result, affected_flags) = sbc_bin(1_u8, 1_u8, true);
        assert_eq!(result, 0);
        assert_eq!(affected_flags, [Negative(false), Overflow(false), Zero(true), Carry(true)]);

        let (result, affected_flags) = sbc_bin(0_u8, 1_u8, true);
        assert_eq!(result, 0b11111111);
        assert_eq!(affected_flags, [Negative(true), Overflow(false), Zero(false), Carry(false)]);

        let (result, affected_flags) = sbc_bin(0_u8, 1_u8, false);
        assert_eq!(result, 0b11111110);
        assert_eq!(affected_flags, [Negative(true), Overflow(false), Zero(false), Carry(false)]);

        // overflow
        let (result, affected_flags) = sbc_bin(0x50_u8, 0xB0_u8, true);
        assert_eq!(result, 0xA0);
        assert_eq!(affected_flags, [Negative(true), Overflow(true), Zero(false), Carry(false)]);
        
        // 16 bit
        let (result, affected_flags) = sbc_bin(1_u16, 1_u16, true);
        assert_eq!(result, 0);
        assert_eq!(affected_flags, [Negative(false), Overflow(false), Zero(true), Carry(true)]);

        let (result, affected_flags) = sbc_bin(0_u16, 1_u16, true);
        assert_eq!(result, 0b11111111_11111111);
        assert_eq!(affected_flags, [Negative(true), Overflow(false), Zero(false), Carry(false)]);

        let (result, affected_flags) = sbc_bin(0_u16, 1_u16, false);
        assert_eq!(result, 0b11111111_11111110);
        assert_eq!(affected_flags, [Negative(true), Overflow(false), Zero(false), Carry(false)]);
    }

    #[test]
    fn test_dec_bcd() {
        // 8 bit
        let (result, affected_flags) = sbc_bcd(0x49_u8, 0x48_u8, false);
        assert_eq!(result, 0x00);
        assert_eq!(affected_flags, [Negative(false), Overflow(false), Zero(true), Carry(true)]);

        let (result, affected_flags) = sbc_bcd(0x49_u8, 0x50_u8, true);
        assert_eq!(result, 0x99);
        assert_eq!(affected_flags, [Negative(true), Overflow(false), Zero(false), Carry(false)]);

        // 16 bit
        let (result, affected_flags) = sbc_bcd(0x4999_u16, 0x4998_u16, false);
        assert_eq!(result, 0x0000);
        assert_eq!(affected_flags, [Negative(false), Overflow(false), Zero(true), Carry(true)]);

        let (result, affected_flags) = sbc_bcd(0x4999_u16, 0x5000_u16, true);
        assert_eq!(result, 0x9999);
        assert_eq!(affected_flags, [Negative(true), Overflow(false), Zero(false), Carry(false)]);
    }

    #[test]
    fn test_and() {
        // 8 bit
        let (result, affected_flags) = and(0b0101_0101_u8, 0b0101_0101_u8);
        assert_eq!(result, 0b0101_0101);
        assert_eq!(affected_flags, [Negative(false), Zero(false)]);

        let (result, affected_flags) = and(0b0101_0101_u8, 0b1010_1010_u8);
        assert_eq!(result, 0x00);
        assert_eq!(affected_flags, [Negative(false), Zero(true)]);

        // 16 bit
        let (result, affected_flags) = and(0b01010101_01010101_u16, 0b01010101_01010101_u16);
        assert_eq!(result, 0b01010101_01010101);
        assert_eq!(affected_flags, [Negative(false), Zero(false)]);

        let (result, affected_flags) = and(0b01010101_01010101_u16, 0b10101010_10101010_u16);
        assert_eq!(result, 0x0000);
        assert_eq!(affected_flags, [Negative(false), Zero(true)]);
    }

    #[test]
    fn test_asl() {
        // 8 bit
        let (result, affected_flags) = asl(0b0101_0101_u8);
        assert_eq!(result, 0b1010_1010);
        assert_eq!(affected_flags, [Negative(true), Zero(false), Carry(false)]);

        let (result, affected_flags) = asl(0b1000_0000_u8);
        assert_eq!(result, 0b0000_0000);
        assert_eq!(affected_flags, [Negative(false), Zero(true), Carry(true)]);

        // 16 bit
        let (result, affected_flags) = asl(0b01000000_00000000_u16);
        assert_eq!(result, 0b10000000_00000000);
        assert_eq!(affected_flags, [Negative(true), Zero(false), Carry(false)]);

        let (result, affected_flags) = asl(0b10000000_00000000_u16);
        assert_eq!(result, 0b00000000_00000000);
        assert_eq!(affected_flags, [Negative(false), Zero(true), Carry(true)]);
    }

    #[test]
    fn test_lsr() {
        // 8 bit
        let (result, affected_flags) = lsr(0b0101_0101_u8);
        assert_eq!(result, 0b0010_1010);
        assert_eq!(affected_flags, [Negative(false), Zero(false), Carry(true)]);

        let (result, affected_flags) = lsr(0x01_u8);
        assert_eq!(result, 0);
        assert_eq!(affected_flags, [Negative(false), Zero(true), Carry(true)]);

        // 16 bit
        let (result, affected_flags) = lsr(0b10000000_00000000_u16);
        assert_eq!(result, 0b01000000_00000000);
        assert_eq!(affected_flags, [Negative(false), Zero(false), Carry(false)]);
    }

    #[test]
    fn test_xor() {
        // 8 bit
        let (result, affected_flags) = eor(0b0101_0101_u8, 0b0101_0101_u8);
        assert_eq!(result, 0);
        assert_eq!(affected_flags, [Negative(false), Zero(true)]);

        let (result, affected_flags) = eor(0b1000_0000_u8, 0b0000_0000_u8);
        assert_eq!(result, 0b1000_0000);
        assert_eq!(affected_flags, [Negative(true), Zero(false)]);

        // 16 bit
        let (result, affected_flags) = eor(0b01010101_00000000_u16, 0b01010101_00000000_u16);
        assert_eq!(result, 0);
        assert_eq!(affected_flags, [Negative(false), Zero(true)]);

        let (result, affected_flags) = eor(0b10000000_00000000_u16, 0b00000000_00000000_u16);
        assert_eq!(result, 0b10000000_00000000);
        assert_eq!(affected_flags, [Negative(true), Zero(false)]);
    }

    #[test]
    fn test_ora() {
        // 8 bit
        let (result, affected_flags) = ora(0b0101_0101_u8, 0b1010_1010_u8);
        assert_eq!(result, 0xFF);
        assert_eq!(affected_flags, [Negative(true), Zero(false)]);

        let (result, affected_flags) = ora(0b1000_0000_u8, 0b0000_0000_u8);
        assert_eq!(result, 0b1000_0000);
        assert_eq!(affected_flags, [Negative(true), Zero(false)]);

        // 16 bit
        let (result, affected_flags) = ora(0b01010101_00000000_u16, 0b01010101_00000000_u16);
        assert_eq!(result, 0b01010101_00000000_u16);
        assert_eq!(affected_flags, [Negative(false), Zero(false)]);

        let (result, affected_flags) = ora(0b00000000_00000000_u16, 0b00000000_00000000_u16);
        assert_eq!(result, 0);
        assert_eq!(affected_flags, [Negative(false), Zero(true)]);
    }

    #[test]
    fn test_rol() {
        let (result, affected_flags) = rol(0b1000_0000_u8, false);
        assert_eq!(result, 0);
        assert_eq!(affected_flags, [Negative(false), Zero(true), Carry(true)]);

        let (result, affected_flags) = rol(0b0100_0000_u8, true);
        assert_eq!(result, 0b10000001);
        assert_eq!(affected_flags, [Negative(true), Zero(false), Carry(false)]);
    }

    #[test]
    fn test_ror() {
        let (result, affected_flags) = ror(0b0000_0001_u8, false);
        assert_eq!(result, 0);
        assert_eq!(affected_flags, [Negative(false), Zero(true), Carry(true)]);

        let (result, affected_flags) = ror(0b0000_0000_u8, true);
        assert_eq!(result, 0b10000000);
        assert_eq!(affected_flags, [Negative(true), Zero(false), Carry(false)]);
    }
}
