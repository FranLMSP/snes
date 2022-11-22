use super::num_trait::SnesNum;

pub fn adc_bin<T: SnesNum>(target: T, value: T, carry: bool) -> (T, bool, bool, bool, bool) {
    let is_carry = target.add_will_carry(value, carry);
    let result = target.add_snes(value, carry);
    let is_negative = result.is_negative();
    let is_zero = result.is_zero();
    let is_overflow = target.is_overflow(value, result);
    (result, is_carry, is_negative, is_zero, is_overflow)
}

pub fn adc8bcd(target: u8, value: u8, carry: bool) -> (u8, bool, bool, bool, bool) {
    let mut is_carry = carry;
    let mut result = (target & 0xF) + (value & 0xF) + (is_carry as u8);
    if result > 9 {
        result += 6;
    }
    result += (target & 0xF0) + (value & 0xF0);
    if result > 0x9F {
        result += 0x60;
    }
    is_carry = result > 0x9F;
    let is_negative = (result >> 7) == 1;
    let is_zero = result == 0;
    let is_overflow = target.is_overflow(value, result);
    (result, is_carry, is_negative, is_zero, is_overflow)
}

pub fn adc16bcd(target: u16, value: u16, carry: bool) -> (u16, bool, bool, bool, bool) {
    let mut is_carry = carry;
    let mut result = (target & 0xF) + (value & 0xF) + (is_carry as u16);
    if result > 9 {
        result += 6;
    }
    result += (target & 0xF0) + (value & 0xF0);
    if result > 0x9F {
        result += 0x60;
    }
    result += (target & 0xF00) + (value & 0xF00);
    if result > 0x9FF {
        result += 0x600;
    }
    is_carry = result > 0x9FFF;
    let is_negative = (result >> 15) == 1;
    let is_zero = result == 0;
    (result, is_carry, is_negative, is_zero, false) // TODO: return overflow
}

pub fn sbc_bin<T: SnesNum>(target: T, value: T, carry: bool) -> (T, bool, bool, bool, bool) {
    let is_carry = target.sbc_will_carry(value, carry);
    let result = target.sbc_snes(value, carry);
    let is_negative = result.is_negative();
    let is_zero = result.is_zero();
    let is_overflow = target.is_overflow(value, result);
    (result, is_carry, is_negative, is_zero, is_overflow)
}

pub fn sbc8bcd(target: u8, value: u8, carry: bool) -> (u8, bool, bool, bool, bool) {
    let mut is_carry = carry;
    let target = target as u16;
    let value = !(value as u16);
    let mut result = (target & 0x0F) + (value & 0x0F) + (is_carry as u16);
    if result <= 0x0F {
        result = result.wrapping_sub(0x06);
    }
    is_carry = result > 0x0F;
    result = (target & 0xF0) + (value & 0xF0) + ((is_carry as u16) << 4) + (result & 0x0F);
    if result <= 0xFF {
        result = result.wrapping_sub(0x60);
    }
    is_carry = result > 0xFF;
    let result = result as u8;
    let is_negative = (result >> 7) == 1;
    let is_zero = result == 0;
    (result, is_carry, is_negative, is_zero, false) // TODO: return overflow
}

pub fn sbc16bcd(target: u16, value: u16, carry: bool) -> (u16, bool, bool, bool, bool) {
    let mut is_carry = carry;
    let target = target as u32;
    let value = !(value as u32);
    let mut result = ((target & 0x0F) + (value & 0x0F) + (is_carry as u32)) as u32;
    if result <= 0x0F {
        result = result.wrapping_sub(0x06);
    }
    is_carry = result > 0xF;
    result = (target & 0xF0) + (value & 0xF0) + ((is_carry as u32) << 4) + (result & 0xF);
    if result <= 0xFF {
        result = result.wrapping_sub(0x60);
    }
    is_carry = result > 0xFF;
    result = (target & 0xF00) + (value & 0xF00) + ((is_carry as u32) << 8) + (result & 0xFF);
    if result <= 0xFFF {
        result = result.wrapping_sub(0x600);
    }
    is_carry = result > 0xFFF;
    result = (target & 0xF000) + (value & 0xF000) + ((is_carry as u32) << 12) + (result & 0xFFF);
    if result <= 0xFFFF {
        result = result.wrapping_sub(0x6000);
    }

    is_carry = result > 0xFFFF;
    let result = result as u16;
    let is_negative = (result >> 15) == 1;
    let is_zero = result == 0;
    (result, is_carry, is_negative, is_zero, false) // TODO: return overflow
}

pub fn and<T: SnesNum>(target: T, value: T) -> (T, bool, bool) {
    let result = target.and(value);
    let is_negative = result.is_negative();
    let is_zero = result.is_zero();
    (result, is_negative, is_zero)
}

pub fn asl<T: SnesNum>(target: T) -> (T, bool, bool, bool) {
    let result = target.asl();
    let is_negative = result.is_negative();
    let is_zero = result.is_zero();
    let is_carry = target.is_negative();
    (result, is_negative, is_zero, is_carry)
}

#[cfg(test)]
mod alu_tests {
    use super::*;

    #[test]
    fn test_adc_bin() {
        // 8 bits
        let (result, carry, negative, zero, overflow) = adc_bin(0_u8, 0_u8, false);
        assert_eq!(result, 0);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, true);
        assert_eq!(overflow, false);

        let (result, carry, negative, zero, overflow) = adc_bin(0_u8, 50_u8, false);
        assert_eq!(result, 50);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, false);
        assert_eq!(overflow, false);

        let (result, carry, negative, zero, overflow) = adc_bin(200_u8, 155_u8, false);
        assert_eq!(result, 99);
        assert_eq!(carry, true);
        assert_eq!(negative, false);
        assert_eq!(zero, false);
        assert_eq!(overflow, false);

        let (result, carry, negative, zero, overflow) = adc_bin(200_u8, 155_u8, true);
        assert_eq!(result, 100);
        assert_eq!(carry, true);
        assert_eq!(negative, false);
        assert_eq!(zero, false);
        assert_eq!(overflow, false);

        let (result, carry, negative, zero, overflow) = adc_bin(200_u8, 54_u8, true);
        assert_eq!(result, 255);
        assert_eq!(carry, false);
        assert_eq!(negative, true);
        assert_eq!(zero, false);
        assert_eq!(overflow, false);

        let (result, carry, negative, zero, overflow) = adc_bin(200_u8, 54_u8, true);
        assert_eq!(result, 255);
        assert_eq!(carry, false);
        assert_eq!(negative, true);
        assert_eq!(zero, false);
        assert_eq!(overflow, false);

        // 16 bits
        let (result, carry, negative, zero, overflow) = adc_bin(0_u16, 0_u16, false);
        assert_eq!(result, 0);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, true);
        assert_eq!(overflow, false);

        let (result, carry, negative, zero, overflow) = adc_bin(0_u16, 50_u16, false);
        assert_eq!(result, 50);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, false);
        assert_eq!(overflow, false);

        let (result, carry, negative, zero, overflow) = adc_bin(65530_u16, 10_u16, false);
        assert_eq!(result, 4);
        assert_eq!(carry, true);
        assert_eq!(negative, false);
        assert_eq!(zero, false);
        assert_eq!(overflow, true);

        let (result, carry, negative, zero, overflow) = adc_bin(65530_u16, 10_u16, true);
        assert_eq!(result, 5);
        assert_eq!(carry, true);
        assert_eq!(negative, false);
        assert_eq!(zero, false);
        assert_eq!(overflow, true);

        let (result, carry, negative, zero, overflow) = adc_bin(65530_u16, 4_u16, true);
        assert_eq!(result, 65535);
        assert_eq!(carry, false);
        assert_eq!(negative, true);
        assert_eq!(zero, false);
        assert_eq!(overflow, false);
    }

    #[test]
    fn test_adc_bcd() {
        // 8 bits
        let (result, carry, negative, zero, overflow) = adc8bcd(5, 5, false);
        assert_eq!(result, 0b0001_0000);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, false);
        assert_eq!(overflow, false);

        let (result, carry, negative, zero, overflow) = adc8bcd(7, 9, false);
        assert_eq!(result, 0b0001_0110);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, false);
        assert_eq!(overflow, false);

        let (result, carry, negative, zero, overflow) = adc8bcd(5, 4, true);
        assert_eq!(result, 0b0001_0000);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, false);
        assert_eq!(overflow, false);

        let (result, carry, negative, zero, overflow) = adc8bcd(7, 8, true);
        assert_eq!(result, 0b0001_0110);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, false);
        assert_eq!(overflow, false);

        let (result, carry, negative, zero, overflow) = adc8bcd(0, 0, false);
        assert_eq!(result, 0);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, true);
        assert_eq!(overflow, false);

        let (result, carry, negative, zero, overflow) = adc8bcd(0b0001_1001, 0b0010_1000, false);
        assert_eq!(result, 0b0100_0111);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, false);
        assert_eq!(overflow, false);

        // 16 bits
        let (result, carry, negative, zero, overflow) = adc16bcd(5, 5, false);
        assert_eq!(result, 0b0001_0000);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, false);
        assert_eq!(overflow, false);

        let (result, carry, negative, zero, overflow) = adc16bcd(7, 9, false);
        assert_eq!(result, 0b0001_0110);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, false);
        assert_eq!(overflow, false);

        let (result, carry, negative, zero, overflow) = adc16bcd(5, 4, true);
        assert_eq!(result, 0b0001_0000);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, false);
        assert_eq!(overflow, false);

        let (result, carry, negative, zero, overflow) = adc16bcd(7, 8, true);
        assert_eq!(result, 0b0001_0110);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, false);
        assert_eq!(overflow, false);

        let (result, carry, negative, zero, overflow) = adc16bcd(0, 0, false);
        assert_eq!(result, 0);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, true);
        assert_eq!(overflow, false);

        let (result, carry, negative, zero, overflow) = adc16bcd(0x0500, 0x0500, false);
        assert_eq!(result, 0b0001_0000_0000_0000);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, false);
        assert_eq!(overflow, false);

        let (result, carry, negative, zero, overflow) = adc16bcd(0b0001_1001, 0b0010_1000, false);
        assert_eq!(result, 0b0100_0111);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, false);
        assert_eq!(overflow, false);
    }

    #[test]
    fn test_dec_bin() {
        // 8 bit
        let (result, carry, negative, zero, overflow) = sbc_bin(1_u8, 1_u8, false);
        assert_eq!(result, 0);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, true);
        assert_eq!(overflow, false);

        let (result, carry, negative, zero, overflow) = sbc_bin(0_u8, 1_u8, false);
        assert_eq!(result, 0b11111111);
        assert_eq!(carry, true);
        assert_eq!(negative, true);
        assert_eq!(zero, false);
        assert_eq!(overflow, false);

        let (result, carry, negative, zero, overflow) = sbc_bin(0_u8, 1_u8, true);
        assert_eq!(result, 0b11111110);
        assert_eq!(carry, true);
        assert_eq!(negative, true);
        assert_eq!(zero, false);
        assert_eq!(overflow, false);
        
        // 16 bit
        let (result, carry, negative, zero, overflow) = sbc_bin(1_u16, 1_u16, false);
        assert_eq!(result, 0);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, true);
        assert_eq!(overflow, false);

        let (result, carry, negative, zero, overflow) = sbc_bin(0_u16, 1_u16, false);
        assert_eq!(result, 0b11111111_11111111);
        assert_eq!(carry, true);
        assert_eq!(negative, true);
        assert_eq!(zero, false);
        assert_eq!(overflow, false);

        let (result, carry, negative, zero, overflow) = sbc_bin(0_u16, 1_u16, true);
        assert_eq!(result, 0b11111111_11111110);
        assert_eq!(carry, true);
        assert_eq!(negative, true);
        assert_eq!(zero, false);
        assert_eq!(overflow, false);
    }

    #[test]
    fn test_dec_bcd() {
        // 8 bit
        let (result, carry, negative, zero, overflow) = sbc8bcd(0x49, 0x48, false);
        assert_eq!(result, 0x00);
        assert_eq!(carry, true);
        assert_eq!(negative, false);
        assert_eq!(zero, true);
        assert_eq!(overflow, false);

        let (result, carry, negative, zero, overflow) = sbc8bcd(0x49, 0x50, true);
        assert_eq!(result, 0x99);
        assert_eq!(carry, false);
        assert_eq!(negative, true);
        assert_eq!(zero, false);
        assert_eq!(overflow, false);

        // 16 bit
        let (result, carry, negative, zero, overflow) = sbc16bcd(0x4999, 0x4998, false);
        assert_eq!(result, 0x0000);
        assert_eq!(carry, true);
        assert_eq!(negative, false);
        assert_eq!(zero, true);
        assert_eq!(overflow, false);

        let (result, carry, negative, zero, overflow) = sbc16bcd(0x4999, 0x5000, true);
        assert_eq!(result, 0x9999);
        assert_eq!(carry, false);
        assert_eq!(negative, true);
        assert_eq!(zero, false);
        assert_eq!(overflow, false);
    }

    #[test]
    fn test_and() {
        // 8 bit
        let (result, is_negative, is_zero) = and(0b0101_0101_u8, 0b0101_0101_u8);
        assert_eq!(result, 0b0101_0101);
        assert_eq!(is_negative, false);
        assert_eq!(is_zero, false);

        let (result, is_negative, is_zero) = and(0b0101_0101_u8, 0b1010_1010_u8);
        assert_eq!(result, 0x00);
        assert_eq!(is_negative, false);
        assert_eq!(is_zero, true);

        // 16 bit
        let (result, is_negative, is_zero) = and(0b01010101_01010101_u16, 0b01010101_01010101_u16);
        assert_eq!(result, 0b01010101_01010101);
        assert_eq!(is_negative, false);
        assert_eq!(is_zero, false);

        let (result, is_negative, is_zero) = and(0b01010101_01010101_u16, 0b10101010_10101010_u16);
        assert_eq!(result, 0x0000);
        assert_eq!(is_negative, false);
        assert_eq!(is_zero, true);
    }

    #[test]
    fn test_asl() {
        // 8 bit
        let (result, is_negative, is_zero, is_carry) = asl(0b0101_0101_u8);
        assert_eq!(result, 0b1010_1010);
        assert_eq!(is_negative, true);
        assert_eq!(is_zero, false);
        assert_eq!(is_carry, false);

        let (result, is_negative, is_zero, is_carry) = asl(0b1000_0000_u8);
        assert_eq!(result, 0b0000_0000);
        assert_eq!(is_negative, false);
        assert_eq!(is_zero, true);
        assert_eq!(is_carry, true);

        // 16 bit
        let (result, is_negative, is_zero, is_carry) = asl(0b01000000_00000000_u16);
        assert_eq!(result, 0b10000000_00000000);
        assert_eq!(is_negative, true);
        assert_eq!(is_zero, false);
        assert_eq!(is_carry, false);

        let (result, is_negative, is_zero, is_carry) = asl(0b10000000_00000000_u16);
        assert_eq!(result, 0b00000000_00000000);
        assert_eq!(is_negative, false);
        assert_eq!(is_zero, true);
        assert_eq!(is_carry, true);
    }
}
