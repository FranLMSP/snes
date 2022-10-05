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
    let mut result: u8 = 0;
    let mut operand_mask: u8 = 0x0F;
    let mut result_mask: u8 = 0x00;
    let mut carry_check: u8= 0x09;
    let mut carry_add: u8 = 0x06;
    let mut is_carry_shifts: u8 = 0;
    for _ in 0..2 {
        result = (result & result_mask)
            .wrapping_add(target & operand_mask)
            .wrapping_add(value & operand_mask)
            .wrapping_add((is_carry as u8) << is_carry_shifts);
        is_carry = result > carry_check;
        if is_carry {
            result = result.wrapping_add(carry_add);
        }
        operand_mask = operand_mask << 4;
        carry_add = carry_add << 4;
        carry_check = (carry_check << 4) | 0x0F;
        result_mask = (result_mask << 4) | 0x0F;
        is_carry_shifts += 4;
    }
    let is_negative = (result >> 7) == 1;
    let is_zero = result == 0;
    (result, is_carry, is_negative, is_zero)
}

pub fn adc16bcd(target: u16, value: u16, carry: bool) -> (u16, bool, bool, bool) {
    let mut is_carry = carry;
    let mut result: u16 = 0;
    let mut operand_mask: u16 = 0x0F;
    let mut result_mask: u16 = 0x00;
    let mut carry_check: u16 = 0x09;
    let mut carry_add: u16 = 0x06;
    let mut is_carry_shifts: u16 = 0;
    for _ in 0..4 {
        result = (result & result_mask)
            .wrapping_add(target & operand_mask)
            .wrapping_add(value & operand_mask)
            .wrapping_add((is_carry as u16) << is_carry_shifts);
        is_carry = result > carry_check;
        if is_carry {
            result = result.wrapping_add(carry_add);
        }
        operand_mask = operand_mask << 4;
        carry_add = carry_add << 4;
        carry_check = (carry_check << 4) | 0x0F;
        result_mask = (result_mask << 4) | 0x0F;
        is_carry_shifts += 4;
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
