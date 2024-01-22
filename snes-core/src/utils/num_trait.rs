pub trait SnesNum: Copy + Clone + Sized + Eq + PartialEq {
    fn add_will_carry(&self, v: Self, carry: bool) -> bool;
    fn sbc_will_carry(&self, v: Self, carry: bool) -> bool;
    fn is_overflow(&self, v: Self, r: Self) -> bool;
    fn add_snes(&self, v: Self, carry: bool) -> Self;
    fn sbc_snes(&self, v: Self, carry: bool) -> Self;
    fn and(&self, v: Self) -> Self;
    fn asl(&self) -> Self;
    fn lsr(&self) -> Self;
    fn xor(&self, v: Self) -> Self;
    fn ora(&self, v: Self) -> Self;
    fn rol(&self, carry: bool) -> Self;
    fn ror(&self, carry: bool) -> Self;
    fn is_negative(&self) -> bool;
    fn is_zero(&self) -> bool;
    fn next_to_highest_bit(&self) -> bool;
    fn lowest_bit(&self) -> bool;
    fn to_u32(&self) -> u32;
    fn from_u32(v: u32) -> Self;
    fn invert(&self) -> Self;
    fn bytes(&self) -> usize;
}

macro_rules! define_will_carry {
    ($t:ty, $method:ident, $inner_method:ident) => {
        fn $method(&self, v: $t, carry: bool) -> bool {
            match self.$inner_method(v) {
                None => true,
                Some(res) => match res.$inner_method(carry as $t) {
                    None => true,
                    Some(_) => false,
                },
            }
        }
    }
}

macro_rules! define_operation {
    ($t:ty, $method:ident, $inner_method:ident) => {
        fn $method(&self, v: $t, carry: bool) -> $t {
            self.$inner_method(v).$inner_method(carry as $t)
        }
    }
}

macro_rules! define_is_overflow {
    ($t:ty) => {
        fn is_overflow(&self, v: $t, r: $t) -> bool {
            let target_msb = (*self).is_negative();
            let value_msb = v.is_negative();
            let result_msb = r.is_negative();
            (target_msb ^ result_msb) && !(target_msb ^ value_msb)
        }
    }
}

macro_rules! define_impl {
    ($t:ty, $bytes:literal) => {
        impl SnesNum for $t {
            define_will_carry!($t, add_will_carry, checked_add);
            define_will_carry!($t, sbc_will_carry, checked_sub);

            define_operation!($t, add_snes, wrapping_add);
            define_operation!($t, sbc_snes, wrapping_sub);

            define_is_overflow!($t);

            fn and(&self, v: $t) -> $t {
                (* self) & v
            }

            fn asl(&self) -> $t {
                (* self) << 1
            }

            fn lsr(&self) -> $t {
                (* self) >> 1
            }

            fn xor(&self, v: $t) -> $t {
                (* self) ^ v
            }

            fn ora(&self, v: $t) -> $t {
                (* self) | v
            }

            fn rol(&self, carry: bool) -> $t {
                ((* self) << 1) | (carry as $t)
            }

            fn ror(&self, carry: bool) -> $t {
                let mut result = ((* self) >> 1);
                if carry {
                    result |= (<$t>::MAX) & !(<$t>::MAX >> 1)
                }
                result
            }

            fn is_negative(&self) -> bool {
                (*self) & !(<$t>::MAX >> 1) != 0
            }

            fn is_zero(&self) -> bool {
                (*self) == 0
            }

            fn lowest_bit(&self) -> bool {
                (*self) & 1 == 1
            }

            fn next_to_highest_bit(&self) -> bool {
                ((*self) << 1) & !(<$t>::MAX >> 1) != 0
            }

            fn to_u32(&self) -> u32 {
                (* self) as u32
            }

            fn from_u32(v: u32) -> $t {
                v as $t
            }

            fn invert(&self) -> $t {
                !(* self)
            }

            fn bytes(&self) -> usize {
                $bytes
            }
        }
    };
}

define_impl!(u8, 1);
define_impl!(u16, 2);