pub fn adc8bin(target: u8, value: u8, carry: bool) -> (u8, bool, bool, bool) {
    let is_carry = match target.checked_add(value) {
        None => true,
        Some(res) => match res.checked_add(carry as u8) {
            None => true,
            Some(_) => false,
        },
    };
    let result = target
        .wrapping_add(value)
        .wrapping_add(carry as u8);
    let is_negative = (result >> 7) == 1;
    let is_zero = result == 0;
    (result, is_carry, is_negative, is_zero)
}

pub fn adc16bin(target: u16, value: u16, carry: bool) -> (u16, bool, bool, bool) {
    let is_carry = match target.checked_add(value) {
        None => true,
        Some(res) => match res.checked_add(carry as u16) {
            None => true,
            Some(_) => false,
        },
    };
    let result = target
        .wrapping_add(value)
        .wrapping_add(carry as u16);
    let is_negative = (result >> 15) == 1;
    let is_zero = result == 0;
    (result, is_carry, is_negative, is_zero)
}

pub fn adc8bcd(target: u8, value: u8, carry: bool) -> (u8, bool, bool, bool) {
    let mut is_carry = carry;
    let mut result = (target & 0xF) + (value & 0xF) + (is_carry as u8);
    is_carry = result > 9;
    if is_carry {
        result += 6;
    }
    result = (result & 0xF0) + (target & 0xF0) + (value & 0xF0) + ((is_carry as u8) << 4);
    is_carry = result > 0x9F;
    if is_carry {
        result = result.wrapping_add(0x60);
    }
    let is_negative = (result >> 7) == 1;
    let is_zero = result == 0;
    (result, is_carry, is_negative, is_zero)
}

pub fn adc16bcd(target: u16, value: u16, carry: bool) -> (u16, bool, bool, bool) {
    let mut is_carry = carry;
    let mut result = (target & 0xF) + (value & 0xF) + (is_carry as u16);
    is_carry = false;
    if result > 9 {
        result += 6;
        is_carry = true;
    }
    result = (result & 0xF0) + (target & 0xF0) + (value & 0xF0) + ((is_carry as u16) << 4);
    is_carry = false;
    if result > 0x9F {
        result += 0x60;
        is_carry = true;
    }
    result = (result & 0xF00) + (target & 0xF00) + (value & 0xF00) + ((is_carry as u16) << 8);
    is_carry = false;
    if result > 0x9FF {
        result += 0x600;
        is_carry = true;
    }
    result = (result & 0xF000) + (target & 0xF000) + (target & 0xF000) + ((is_carry as u16) << 12);
    is_carry = false;
    if result > 0x9FFF {
        result += 0x6000;
        is_carry = true;
    }
    let is_negative = (result >> 15) == 1;
    let is_zero = result == 0;
    (result, is_carry, is_negative, is_zero)
}


#[cfg(test)]
mod alu_tests {
    use super::*;

    #[test]
    fn test_adc8bin() {
        let (result, carry, negative, zero) = adc8bin(0, 0, false);
        assert_eq!(result, 0);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, true);

        let (result, carry, negative, zero) = adc8bin(0, 50, false);
        assert_eq!(result, 50);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, false);

        let (result, carry, negative, zero) = adc8bin(200, 155, false);
        assert_eq!(result, 99);
        assert_eq!(carry, true);
        assert_eq!(negative, false);
        assert_eq!(zero, false);

        let (result, carry, negative, zero) = adc8bin(200, 155, true);
        assert_eq!(result, 100);
        assert_eq!(carry, true);
        assert_eq!(negative, false);
        assert_eq!(zero, false);

        let (result, carry, negative, zero) = adc8bin(200, 54, true);
        assert_eq!(result, 255);
        assert_eq!(carry, false);
        assert_eq!(negative, true);
        assert_eq!(zero, false);
    }

    #[test]
    fn test_adc16bin() {
        let (result, carry, negative, zero) = adc16bin(0, 0, false);
        assert_eq!(result, 0);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, true);

        let (result, carry, negative, zero) = adc16bin(0, 50, false);
        assert_eq!(result, 50);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, false);

        let (result, carry, negative, zero) = adc16bin(65530, 10, false);
        assert_eq!(result, 4);
        assert_eq!(carry, true);
        assert_eq!(negative, false);
        assert_eq!(zero, false);

        let (result, carry, negative, zero) = adc16bin(65530, 10, true);
        assert_eq!(result, 5);
        assert_eq!(carry, true);
        assert_eq!(negative, false);
        assert_eq!(zero, false);

        let (result, carry, negative, zero) = adc16bin(65530, 4, true);
        assert_eq!(result, 65535);
        assert_eq!(carry, false);
        assert_eq!(negative, true);
        assert_eq!(zero, false);
    }

    #[test]
    fn test_adc8bcd() {
        let (result, carry, negative, zero) = adc8bcd(5, 5, false);
        assert_eq!(result, 0b0001_0000);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, false);

        let (result, carry, negative, zero) = adc8bcd(7, 9, false);
        assert_eq!(result, 0b0001_0110);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, false);

        let (result, carry, negative, zero) = adc8bcd(5, 4, true);
        assert_eq!(result, 0b0001_0000);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, false);

        let (result, carry, negative, zero) = adc8bcd(7, 8, true);
        assert_eq!(result, 0b0001_0110);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, false);

        let (result, carry, negative, zero) = adc8bcd(0, 0, false);
        assert_eq!(result, 0);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, true);
    }

    #[test]
    fn test_adc16bcd() {
        let (result, carry, negative, zero) = adc16bcd(5, 5, false);
        assert_eq!(result, 0b0001_0000);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, false);

        let (result, carry, negative, zero) = adc16bcd(7, 9, false);
        assert_eq!(result, 0b0001_0110);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, false);

        let (result, carry, negative, zero) = adc16bcd(5, 4, true);
        assert_eq!(result, 0b0001_0000);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, false);

        let (result, carry, negative, zero) = adc16bcd(7, 8, true);
        assert_eq!(result, 0b0001_0110);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, false);

        let (result, carry, negative, zero) = adc16bcd(0, 0, false);
        assert_eq!(result, 0);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, true);

        let (result, carry, negative, zero) = adc16bcd(0x0500, 0x0500, false);
        assert_eq!(result, 0b0001_0000_0000_0000);
        assert_eq!(carry, false);
        assert_eq!(negative, false);
        assert_eq!(zero, false);
    }
}
